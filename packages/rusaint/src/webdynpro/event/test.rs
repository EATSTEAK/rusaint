use crate::webdynpro::event;

#[test]
fn event_string_escape_test() {
    assert_eq!(
        event::escape_str("https://ecc.ssu.ac.kr/sap/bc/webdynpro/sap/zcmw2100?sap-language=KO#"),
        "https~003A~002F~002Fecc.ssu.ac.kr~002Fsap~002Fbc~002Fwebdynpro~002Fsap~002Fzcmw2100~003Fsap-language~003DKO~0023"
    );
    assert_eq!(
        event::escape_str(
            "@{\"iSelectionStart\":0,\"iSelectionEnd\":0,\"iCursorPos\":0,\"sValue\":\"2 학기\",\"sFocussedId\":\"WDDC\",\"sApplyControlId\":\"WDDC\"}"
        ),
        "~0040~007B~0022iSelectionStart~0022~003A0~002C~0022iSelectionEnd~0022~003A0~002C~0022iCursorPos~0022~003A0~002C~0022sValue~0022~003A~00222~0020~D559~AE30~0022~002C~0022sFocussedId~0022~003A~0022WDDC~0022~002C~0022sApplyControlId~0022~003A~0022WDDC~0022~007D"
    );
}

#[test]
fn event_string_unescape_test() {
    assert_eq!(event::unescape_str("https~003A~002F~002Fecc.ssu.ac.kr~002Fsap~002Fbc~002Fwebdynpro~002Fsap~002Fzcmw2100~003Fsap-language~003DKO~0023").unwrap(),
               "https://ecc.ssu.ac.kr/sap/bc/webdynpro/sap/zcmw2100?sap-language=KO#");
    assert_eq!(event::unescape_str("~0040~007B~0022iSelectionStart~0022~003A0~002C~0022iSelectionEnd~0022~003A0~002C~0022iCursorPos~0022~003A0~002C~0022sValue~0022~003A~00222~0020~D559~AE30~0022~002C~0022sFocussedId~0022~003A~0022WDDC~0022~002C~0022sApplyControlId~0022~003A~0022WDDC~0022~007D").unwrap(),
               "@{\"iSelectionStart\":0,\"iSelectionEnd\":0,\"iCursorPos\":0,\"sValue\":\"2 학기\",\"sFocussedId\":\"WDDC\",\"sApplyControlId\":\"WDDC\"}");
}
