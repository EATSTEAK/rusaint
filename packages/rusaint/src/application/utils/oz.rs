//! OZ Report 서버와의 통신 유틸리티.
//!
//! u-saint의 강의계획서 등 일부 기능은 OZ Report Server에서 데이터를 가져옵니다.
//! 이 모듈은 WebDynpro의 `openExternalWindow` script_call에서 OZ URL을 추출하고,
//! OZ 클라이언트를 통해 DataModule 데이터를 가져오는 기능을 제공합니다.

use crate::ApplicationError;
use crate::RusaintError;

/// OZ Report Server의 공개 게스트 크리덴셜.
/// OZ viewer는 인증 없이 데이터를 조회할 수 있는 게스트 계정을 제공하며,
/// 이 값은 숭실대 OZ 서버의 표준 게스트 크리덴셜입니다.
const OZ_DEFAULT_USER: &str = "guest";
const OZ_DEFAULT_PASSWORD: &str = "guest";
const OZ_DEFAULT_HOST: &str = "office.ssu.ac.kr";

/// OZ URL에서 파싱된 파라미터들
pub(crate) struct OzUrlParams {
    pub base_url: String,
    pub ozrname: String,
    pub category: String,
    pub params: Vec<(String, String)>,
    pub odi_name: String,
}

/// JavaScript `\xNN` hex escapes를 실제 문자로 변환합니다.
///
/// OZ URL이 포함된 script_call에는 JavaScript hex escape 시퀀스가 포함될 수 있습니다.
/// 이 함수는 `\xNN` 패턴을 해당 ASCII 문자로 변환합니다.
/// 다른 백슬래시 이스케이프 시퀀스(예: `\\`, `\n`)는 원본 그대로 보존됩니다.
fn convert_js_hex_escapes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if chars.peek() == Some(&'x') {
                chars.next(); // consume 'x'
                let mut hex = String::with_capacity(2);
                for _ in 0..2 {
                    if let Some(&c) = chars.peek() {
                        if c.is_ascii_hexdigit() {
                            hex.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                if hex.len() == 2 {
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte as char);
                    } else {
                        result.push('\\');
                        result.push('x');
                        result.push_str(&hex);
                    }
                } else {
                    result.push('\\');
                    result.push('x');
                    result.push_str(&hex);
                }
            } else {
                // 다른 이스케이프 시퀀스는 원본 그대로 보존 (예: \\, \n, \t 등)
                result.push(ch);
                if let Some(&next) = chars.peek() {
                    result.push(next);
                    chars.next();
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// script_calls에서 openExternalWindow URL을 추출합니다.
pub(crate) fn extract_oz_url_from_script_calls(
    script_calls: &[String],
) -> Result<String, RusaintError> {
    let oz_url_raw = script_calls
        .iter()
        .find(|call| call.contains("openExternalWindow"))
        .and_then(|call| {
            tracing::debug!("Found openExternalWindow script_call: {}", call);
            // application.exec("openExternalWindow",{...,"url":"THE_URL",...});
            // JSON 부분만 추출하여 serde_json으로 파싱
            let json_start = call.find('{')?;
            let json_end = call.rfind('}')?;
            let json_str = &call[json_start..=json_end];
            // JavaScript \xNN hex escapes를 실제 문자로 변환 (JSON에서는 유효하지 않음)
            let json_str_converted = convert_js_hex_escapes(json_str);
            let parsed: serde_json::Value = serde_json::from_str(&json_str_converted).ok()?;
            parsed["url"].as_str().map(|s| s.to_string())
        })
        .ok_or_else(|| {
            ApplicationError::OzDataFetchError(format!(
                "No openExternalWindow URL found in script_calls: {:?}",
                script_calls
            ))
        })?;

    tracing::debug!("Parsed OZ URL: {}", oz_url_raw);

    if oz_url_raw.starts_with("http://") || oz_url_raw.starts_with("https://") {
        Ok(oz_url_raw)
    } else {
        let separator = if oz_url_raw.starts_with('/') { "" } else { "/" };
        Ok(format!(
            "https://{}{}{}",
            OZ_DEFAULT_HOST, separator, oz_url_raw
        ))
    }
}

/// OZ URL 문자열을 파싱하여 [`OzUrlParams`]를 반환합니다.
pub(crate) fn parse_oz_url_params(oz_url: &str) -> Result<OzUrlParams, RusaintError> {
    let parsed_url = url::Url::parse(oz_url).map_err(|e| {
        ApplicationError::OzDataFetchError(format!("Failed to parse OZ URL '{}': {}", oz_url, e))
    })?;

    let base_url = format!(
        "{}://{}/oz70",
        parsed_url.scheme(),
        parsed_url.host_str().unwrap_or(OZ_DEFAULT_HOST)
    );

    // NOTE: HashMap으로 수집하므로 동일 key의 query parameter가 여러 개 있을 경우
    // 마지막 값만 보존됩니다. OZ URL에서는 중복 key가 발생하지 않는 것으로 확인되었습니다.
    let query_pairs: std::collections::HashMap<String, String> = parsed_url
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    tracing::debug!("Query pairs: {:?}", query_pairs);

    let ozrname = query_pairs.get("ozrname").cloned().ok_or_else(|| {
        ApplicationError::OzDataFetchError(format!(
            "Missing ozrname in URL: {} (query_pairs: {:?})",
            oz_url, query_pairs
        ))
    })?;
    let category = query_pairs.get("category").cloned().ok_or_else(|| {
        ApplicationError::OzDataFetchError(format!("Missing category in URL: {}", oz_url))
    })?;
    let p_names_str = query_pairs.get("pName").cloned().ok_or_else(|| {
        ApplicationError::OzDataFetchError(format!("Missing pName in URL: {}", oz_url))
    })?;
    let p_values_str = query_pairs.get("pValue").cloned().ok_or_else(|| {
        ApplicationError::OzDataFetchError(format!("Missing pValue in URL: {}", oz_url))
    })?;

    let p_names: Vec<&str> = p_names_str.split(',').collect();
    let p_values: Vec<&str> = p_values_str.split(',').collect();

    if p_names.len() != p_values.len() {
        return Err(ApplicationError::OzDataFetchError(format!(
            "pName count ({}) does not match pValue count ({}) in URL: {}",
            p_names.len(),
            p_values.len(),
            oz_url
        ))
        .into());
    }

    let mut params: Vec<(String, String)> = p_names
        .iter()
        .zip(p_values.iter())
        .filter(|(name, _)| **name != "P_RANDOM")
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect();

    let has_arg4 = params.iter().any(|(name, _)| name == "arg4");
    if !has_arg4 {
        if let Some(uname_value) = params
            .iter()
            .find(|(name, _)| name == "UNAME")
            .map(|(_, v)| v.clone())
        {
            params.push(("arg4".to_string(), uname_value));
        }
    }

    let odi_name = format!("{}.odi", ozrname);

    Ok(OzUrlParams {
        base_url,
        ozrname,
        category,
        params,
        odi_name,
    })
}

/// OZ URL로부터 OzClient를 생성, 세션 초기화, 로그인, DataModule 데이터를 가져옵니다.
///
/// `script_calls`에서 OZ URL을 추출하고, URL 파라미터를 파싱한 뒤,
/// OzClient를 사용하여 DataModule 응답(`DataModuleResponse`)을 반환합니다.
pub(crate) async fn fetch_data_module_from_script_calls(
    script_calls: &[String],
) -> Result<ozra::types::DataModuleResponse, RusaintError> {
    let oz_url = extract_oz_url_from_script_calls(script_calls)?;
    tracing::debug!("Full OZ URL: {}", oz_url);

    let oz_params = parse_oz_url_params(&oz_url)?;

    tracing::debug!(
        "OZ params: base_url={}, ozrname={}, category={}, odi={}, params={:?}",
        oz_params.base_url,
        oz_params.ozrname,
        oz_params.category,
        oz_params.odi_name,
        oz_params.params
    );

    let oz_client =
        ozra::client::OzClient::new(&oz_params.base_url, OZ_DEFAULT_USER, OZ_DEFAULT_PASSWORD)
            .map_err(|e| {
                ApplicationError::OzDataFetchError(format!("OzClient creation failed: {}", e))
            })?;

    oz_client
        .init_session_with_params(&oz_params.ozrname, &oz_params.category, &oz_params.params)
        .await
        .map_err(|e| {
            ApplicationError::OzDataFetchError(format!("OZ session init failed: {}", e))
        })?;

    oz_client
        .login()
        .await
        .map_err(|e| ApplicationError::OzDataFetchError(format!("OZ login failed: {}", e)))?;

    let response = oz_client
        .fetch_data_module(
            &oz_params.odi_name,
            &format!("/{}", oz_params.category),
            &oz_params.params,
        )
        .await
        .map_err(|e| {
            ApplicationError::OzDataFetchError(format!("OZ data fetch failed: {}", e))
        })?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── convert_js_hex_escapes tests ──

    #[test]
    fn hex_escape_basic() {
        assert_eq!(convert_js_hex_escapes(r"\x41"), "A");
        assert_eq!(convert_js_hex_escapes(r"\x42"), "B");
        assert_eq!(convert_js_hex_escapes(r"\x61"), "a");
    }

    #[test]
    fn hex_escape_consecutive() {
        assert_eq!(convert_js_hex_escapes(r"\x41\x42\x43"), "ABC");
    }

    #[test]
    fn hex_escape_mixed_with_plain_text() {
        assert_eq!(convert_js_hex_escapes(r"hello\x20world"), "hello world");
    }

    #[test]
    fn hex_escape_incomplete_single_digit() {
        // \x4 (only one hex digit) — should preserve as-is
        assert_eq!(convert_js_hex_escapes(r"\x4"), "\\x4");
    }

    #[test]
    fn hex_escape_non_hex_digits() {
        // \xGG — 'G' is not a hex digit, should preserve as-is
        assert_eq!(convert_js_hex_escapes(r"\xGG"), "\\xGG");
    }

    #[test]
    fn hex_escape_empty_string() {
        assert_eq!(convert_js_hex_escapes(""), "");
    }

    #[test]
    fn hex_escape_no_escapes() {
        assert_eq!(
            convert_js_hex_escapes("plain text without escapes"),
            "plain text without escapes"
        );
    }

    #[test]
    fn hex_escape_backslash_backslash() {
        // \\ should be preserved as two characters (backslash + backslash)
        assert_eq!(convert_js_hex_escapes(r"\\"), "\\\\");
    }

    #[test]
    fn hex_escape_other_escape_sequences_preserved() {
        // \n, \t should be preserved as-is (backslash + letter)
        assert_eq!(convert_js_hex_escapes(r"\n"), "\\n");
        assert_eq!(convert_js_hex_escapes(r"\t"), "\\t");
    }

    #[test]
    fn hex_escape_url_encoding() {
        // Typical OZ URL pattern: \x3d is '=', \x26 is '&'
        assert_eq!(
            convert_js_hex_escapes(r"key\x3dvalue\x26other\x3d123"),
            "key=value&other=123"
        );
    }

    // ── extract_oz_url_from_script_calls tests ──

    #[test]
    fn extract_url_from_script_calls_absolute_url() {
        let calls = vec![
            "some.other.call()".to_string(),
            r#"application.exec("openExternalWindow",{"url":"https://office.ssu.ac.kr/oz70/viewer?ozrname=test&category=cat","title":"Test"})"#.to_string(),
        ];
        let result = extract_oz_url_from_script_calls(&calls).unwrap();
        assert_eq!(
            result,
            "https://office.ssu.ac.kr/oz70/viewer?ozrname=test&category=cat"
        );
    }

    #[test]
    fn extract_url_from_script_calls_relative_url_with_slash() {
        let calls = vec![
            r#"application.exec("openExternalWindow",{"url":"/oz70/viewer?ozrname=test&category=cat"})"#.to_string(),
        ];
        let result = extract_oz_url_from_script_calls(&calls).unwrap();
        assert_eq!(
            result,
            "https://office.ssu.ac.kr/oz70/viewer?ozrname=test&category=cat"
        );
    }

    #[test]
    fn extract_url_from_script_calls_relative_url_without_slash() {
        let calls = vec![
            r#"application.exec("openExternalWindow",{"url":"oz70/viewer?ozrname=test&category=cat"})"#.to_string(),
        ];
        let result = extract_oz_url_from_script_calls(&calls).unwrap();
        assert_eq!(
            result,
            "https://office.ssu.ac.kr/oz70/viewer?ozrname=test&category=cat"
        );
    }

    #[test]
    fn extract_url_from_script_calls_with_hex_escapes() {
        let calls = vec![
            r#"application.exec("openExternalWindow",{"url":"https\x3a\x2f\x2foffice.ssu.ac.kr/path?q\x3dval"})"#.to_string(),
        ];
        let result = extract_oz_url_from_script_calls(&calls).unwrap();
        assert_eq!(result, "https://office.ssu.ac.kr/path?q=val");
    }

    #[test]
    fn extract_url_from_script_calls_no_match() {
        let calls = vec!["unrelated.call()".to_string()];
        let result = extract_oz_url_from_script_calls(&calls);
        assert!(result.is_err());
    }

    #[test]
    fn extract_url_from_script_calls_empty() {
        let calls: Vec<String> = vec![];
        let result = extract_oz_url_from_script_calls(&calls);
        assert!(result.is_err());
    }

    // ── parse_oz_url_params tests ──

    #[test]
    fn parse_params_basic() {
        let url = "https://office.ssu.ac.kr/oz70/nview5/data/viewer7.jsp?ozrname=myreport&category=mycat&pName=A,B,C&pValue=1,2,3";
        let params = parse_oz_url_params(url).unwrap();
        assert_eq!(params.base_url, "https://office.ssu.ac.kr/oz70");
        assert_eq!(params.ozrname, "myreport");
        assert_eq!(params.category, "mycat");
        assert_eq!(params.odi_name, "myreport.odi");
        assert_eq!(
            params.params,
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
                ("C".to_string(), "3".to_string()),
            ]
        );
    }

    #[test]
    fn parse_params_filters_p_random() {
        let url = "https://office.ssu.ac.kr/oz70/viewer?ozrname=r&category=c&pName=P_RANDOM,KEY&pValue=12345,val";
        let params = parse_oz_url_params(url).unwrap();
        assert_eq!(params.params, vec![("KEY".to_string(), "val".to_string())]);
    }

    #[test]
    fn parse_params_adds_arg4_from_uname() {
        let url = "https://office.ssu.ac.kr/oz70/viewer?ozrname=r&category=c&pName=UNAME,OTHER&pValue=user123,foo";
        let params = parse_oz_url_params(url).unwrap();
        assert!(
            params
                .params
                .iter()
                .any(|(k, v)| k == "arg4" && v == "user123")
        );
    }

    #[test]
    fn parse_params_no_duplicate_arg4() {
        let url = "https://office.ssu.ac.kr/oz70/viewer?ozrname=r&category=c&pName=arg4,UNAME&pValue=existing,user123";
        let params = parse_oz_url_params(url).unwrap();
        let arg4_count = params.params.iter().filter(|(k, _)| k == "arg4").count();
        assert_eq!(arg4_count, 1);
        assert_eq!(
            params.params.iter().find(|(k, _)| k == "arg4").unwrap().1,
            "existing"
        );
    }

    #[test]
    fn parse_params_mismatched_pname_pvalue_count() {
        let url =
            "https://office.ssu.ac.kr/oz70/viewer?ozrname=r&category=c&pName=A,B,C&pValue=1,2";
        let result = parse_oz_url_params(url);
        assert!(result.is_err());
    }

    #[test]
    fn parse_params_missing_ozrname() {
        let url = "https://office.ssu.ac.kr/oz70/viewer?category=c&pName=A&pValue=1";
        let result = parse_oz_url_params(url);
        assert!(result.is_err());
    }

    #[test]
    fn parse_params_missing_category() {
        let url = "https://office.ssu.ac.kr/oz70/viewer?ozrname=r&pName=A&pValue=1";
        let result = parse_oz_url_params(url);
        assert!(result.is_err());
    }

    #[test]
    fn parse_params_invalid_url() {
        let result = parse_oz_url_params("not a valid url");
        assert!(result.is_err());
    }
}
