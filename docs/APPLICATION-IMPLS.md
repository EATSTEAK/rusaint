# rusaint Application 구현 가이드

이 문서는 rusaint에서 새로운 u-saint Application을 구현하는 방법을 설명합니다. u-saint의 각 페이지는 SAP WebDynpro 프레임워크로 구성되어 있으며, rusaint는 [wdpe (WebDynpro Parse Engine)](https://docs.rs/wdpe) 라이브러리를 사용하여 이러한 페이지를 파싱하고 조작합니다.

## 목차

- [개요](#개요)
- [사전 준비](#사전-준비)
- [Application 구현 단계](#application-구현-단계)
- [실제 예시: CourseRegistrationStatusApplication](#실제-예시-courseregistrationstatusapplication)
- [주요 wdpe 컴포넌트](#주요-wdpe-컴포넌트)
- [테스트 작성](#테스트-작성)
- [FFI 바인딩 추가](#ffi-바인딩-추가)
- [Cookbook: 파싱 로직 구현 레시피](#cookbook-파싱-로직-구현-레시피)
  - [레시피 1: InputField에서 값 읽기](#레시피-1-inputfield에서-값-읽기)
  - [레시피 2: SapTable에서 데이터 파싱](#레시피-2-saptable에서-데이터-파싱)
  - [레시피 3: ComboBox 선택 및 값 읽기](#레시피-3-combobox-선택-및-값-읽기)
  - [레시피 4: 버튼 클릭](#레시피-4-버튼-클릭)
  - [레시피 5: 탭 전환](#레시피-5-탭-전환)
  - [레시피 6: 팝업 창 처리](#레시피-6-팝업-창-처리)
  - [레시피 7: 커스텀 역직렬화 함수](#레시피-7-커스텀-역직렬화-함수)
  - [레시피 8: Model 구조 패턴](#레시피-8-model-구조-패턴)
  - [레시피 9: 이벤트 처리 후 파서 재생성](#레시피-9-이벤트-처리-후-파서-재생성)
- [참고 자료](#참고-자료)

## 개요

rusaint의 Application은 u-saint의 특정 웹 페이지(WebDynpro 애플리케이션)에 대한 클라이언트 래퍼입니다. 각 Application은:

- 특정 u-saint 페이지에 접근하고
- 페이지의 UI 엘리먼트들을 정의하며
- 해당 페이지에서 수행할 수 있는 작업들을 메서드로 제공합니다.

## 사전 준비

### WebDynpro 페이지 분석

새로운 Application을 구현하기 전에 대상 페이지를 분석해야 합니다:

1. **애플리케이션 이름 확인**: u-saint URL에서 애플리케이션 이름을 확인합니다.

   - 예: `https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2110` → `ZCMW2110`

2. **정적 엘리먼트 ID 획득**: WebDynpro는 기본적으로 동적 ID를 사용합니다. 정적 ID를 얻으려면:

   - URL에 `sap-wd-stableids=X` 파라미터를 추가합니다.
   - 예: `https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2110?sap-wd-stableids=X`

3. **엘리먼트 식별**: 브라우저 개발자 도구를 사용하여 조작하고자 하는 UI 엘리먼트들의 ID를 파악합니다.

## Application 구현 단계

### 1. 파일 구조 생성

```
packages/rusaint/src/application/
├── your_application.rs          # Application 구현
├── your_application/
│   └── model.rs                 # 데이터 모델
```

### 2. Application 구조체 정의

```rust
use crate::client::{USaintApplication, USaintClient};
use crate::RusaintError;
use wdpe::body::Body;

/// [페이지 설명](페이지 URL)
#[derive(Debug)]
pub struct YourApplication {
    client: USaintClient,
}
```

### 3. USaintApplication 트레이트 구현

모든 Application은 `USaintApplication` 트레이트를 구현해야 합니다:

```rust
impl USaintApplication for YourApplication {
    /// WebDynpro 애플리케이션 이름 (URL의 마지막 부분)
    const APP_NAME: &'static str = "ZCMW2110";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}
```

### 4. 엘리먼트 정의

wdpe의 `define_elements!` 매크로를 사용하여 페이지의 UI 엘리먼트들을 정의합니다:

```rust
use wdpe::{
    define_elements,
    element::{
        action::Button,
        complex::SapTable,
        selection::ComboBox,
        text::InputField,
    },
};

impl<'app> YourApplication {
    define_elements! {
        // ComboBox 예시
        PERIOD_YEAR: ComboBox<'app> = "ZCMW_PERIOD_RE.ID_xxx:VIW_MAIN.PERYR";
        PERIOD_ID: ComboBox<'app> = "ZCMW_PERIOD_RE.ID_xxx:VIW_MAIN.PERID";

        // SapTable 예시
        MAIN_TABLE: SapTable<'app> = "ZCMW2110.ID_0001:VIW_BOOKED.TABLE";

        // Button 예시
        BTN_SEARCH: Button<'app> = "ZCMW2110.ID_0001:VIW_MAIN.BTN_SEARCH";
    }
}
```

### 5. 비즈니스 로직 구현

```rust
use wdpe::{
    command::WebDynproCommandExecutor,
    command::element::selection::ComboBoxSelectEventCommand,
    element::parser::ElementParser,
};

impl<'app> YourApplication {
    /// 페이지 Body에 대한 참조를 반환합니다.
    fn body(&self) -> &Body {
        self.client.body()
    }

    /// 학기를 선택합니다.
    async fn select_semester(
        &mut self,
        parser: &ElementParser,
        year: &str,
        semester: SemesterType,
    ) -> Result<(), WebDynproError> {
        // 년도 선택 이벤트 생성 및 처리
        let year_select_event = parser.read(ComboBoxSelectEventCommand::new(
            Self::PERIOD_YEAR,
            year,
            false,
        ))?;
        self.client.process_event(false, year_select_event).await?;

        // 학기 선택 이벤트 생성 및 처리
        let semester_select_event = parser.read(ComboBoxSelectEventCommand::new(
            Self::PERIOD_ID,
            semester_to_key(semester),
            false,
        ))?;
        self.client.process_event(false, semester_select_event).await?;

        Ok(())
    }

    /// 데이터를 조회합니다.
    pub async fn fetch_data(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<YourModel>, RusaintError> {
        // 1. 파서 생성
        let parser = ElementParser::new(self.body());

        // 2. 학기 선택
        self.select_semester(&parser, &year.to_string(), semester).await?;

        // 3. 테이블 데이터 읽기
        let parser = ElementParser::new(self.body());
        let data = try_table_into_with_scroll::<YourModel>(
            &mut self.client,
            parser,
            Self::MAIN_TABLE,
        ).await?;

        Ok(data)
    }

    /// 페이지를 새로고침합니다.
    pub async fn reload(&mut self) -> Result<(), RusaintError> {
        self.client.reload().await?;
        Ok(())
    }
}
```

### 6. Model 정의

SapTable에서 데이터를 자동으로 역직렬화하려면 `FromSapTable` 트레이트를 구현합니다:

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize, de::{IntoDeserializer, value::MapDeserializer}};
use wdpe::{
    element::{
        complex::sap_table::FromSapTable,
        definition::ElementDefinition as _,
        parser::ElementParser,
    },
    error::{ElementError, WebDynproError},
};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct YourModel {
    /// 필드 설명
    #[serde(rename(deserialize = "테이블헤더이름"))]
    field_name: String,

    /// 선택적 필드
    #[serde(
        rename(deserialize = "선택적헤더"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    optional_field: Option<String>,
}

impl<'body> FromSapTable<'body> for YourModel {
    fn from_table(
        header: &'body wdpe::element::complex::sap_table::SapTableHeader,
        row: &'body wdpe::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        // 행 데이터를 HashMap으로 변환
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;

        // serde를 사용하여 역직렬화
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(YourModel::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
            element: row.table_def().id().to_string(),
            content: e.to_string(),
        })?)
    }
}
```

## 실제 예시: CourseRegistrationStatusApplication

`CourseRegistrationStatusApplication`은 수강신청 조회 페이지를 래핑합니다.

### 파일 구조

```
packages/rusaint/src/application/
├── course_registration_status.rs
├── course_registration_status/
│   └── model.rs
```

### Application 구현

```rust
// course_registration_status.rs
use crate::ApplicationError;
use crate::application::course_registration_status::model::RegisteredLecture;
use crate::application::utils::sap_table::try_table_into_with_scroll;
use crate::application::utils::semester::get_selected_semester;
use crate::client::{USaintApplication, USaintClient};
use crate::{RusaintError, model::SemesterType};
use wdpe::command::WebDynproCommandExecutor;
use wdpe::command::element::complex::SapTableBodyCommand;
use wdpe::element::ElementDefWrapper;
use wdpe::element::complex::sap_table::cell::{SapTableCell as _, SapTableCellWrapper};
use wdpe::element::parser::ElementParser;
use wdpe::{
    body::Body,
    command::element::selection::ComboBoxSelectEventCommand,
    define_elements,
    element::{complex::SapTable, selection::ComboBox},
    error::WebDynproError,
};

/// [수강신청조회(학생용)](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2110)
#[derive(Debug)]
pub struct CourseRegistrationStatusApplication {
    client: USaintClient,
}

impl USaintApplication for CourseRegistrationStatusApplication {
    const APP_NAME: &'static str = "ZCMW2110";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'app> CourseRegistrationStatusApplication {
    // 학기 코드 매핑
    fn semester_to_key(semester: SemesterType) -> &'static str {
        match semester {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    // 엘리먼트 정의
    define_elements! {
        PERIOD_YEAR: ComboBox<'app> = "ZCMW_PERIOD_RE.ID_57CC7986881470383154D0F1FF86642A:VIW_MAIN.PERYR";
        PERIOD_ID: ComboBox<'app> = "ZCMW_PERIOD_RE.ID_57CC7986881470383154D0F1FF86642A:VIW_MAIN.PERID";
        MAIN_TABLE: SapTable<'app> = "ZCMW2110.ID_0001:VIW_BOOKED.TABLE";
    }

    fn body(&self) -> &Body {
        self.client.body()
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다.
    pub fn get_selected_semester(&self) -> Result<(u32, SemesterType), RusaintError> {
        Ok(get_selected_semester(
            &self.client,
            &Self::PERIOD_YEAR,
            &Self::PERIOD_ID,
        )?)
    }

    /// 개인이 수강신청한 내역을 학기별로 찾습니다.
    pub async fn lectures(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<impl Iterator<Item = RegisteredLecture>, RusaintError> {
        // 학기 선택
        {
            let parser = ElementParser::new(self.body());
            let year_str = format!("{year}");
            self.select_semester(&parser, &year_str, semester).await?;
        }

        // 테이블 데이터 읽기
        let parser = ElementParser::new(self.body());
        let table = parser.read(SapTableBodyCommand::new(Self::MAIN_TABLE))?;

        // 빈 결과 확인
        let Some(first_row) = table.iter().next() else {
            return Err(ApplicationError::NoLectureResult.into());
        };
        // ... 결과 검증 로직 ...

        let lectures = try_table_into_with_scroll::<RegisteredLecture>(
            &mut self.client,
            parser,
            Self::MAIN_TABLE,
        ).await?;

        Ok(lectures.into_iter())
    }

    /// 페이지를 새로고침합니다.
    pub async fn reload(&mut self) -> Result<(), RusaintError> {
        self.client.reload().await?;
        Ok(())
    }
}
```

### Model 구현

```rust
// course_registration_status/model.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize, de::{IntoDeserializer, value::MapDeserializer}};
use crate::application::utils::de_with::deserialize_optional_string;
use wdpe::{
    element::{
        complex::sap_table::FromSapTable,
        definition::ElementDefinition as _,
        parser::ElementParser,
    },
    error::{ElementError, WebDynproError},
};

/// 수강신청한 과목 정보
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct RegisteredLecture {
    #[serde(rename(deserialize = "이수구분(주전공)"))]
    category: String,

    #[serde(
        rename(deserialize = "이수구분(다전공)"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    sub_category: Option<String>,

    #[serde(rename(deserialize = "과목번호"))]
    code: String,

    #[serde(rename(deserialize = "과목명"))]
    name: String,

    // ... 더 많은 필드들 ...
}

impl<'body> FromSapTable<'body> for RegisteredLecture {
    fn from_table(
        header: &'body wdpe::element::complex::sap_table::SapTableHeader,
        row: &'body wdpe::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(RegisteredLecture::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
            element: row.table_def().id().to_string(),
            content: e.to_string(),
        })?)
    }
}
```

## 주요 wdpe 컴포넌트

### ElementParser

페이지 Body에서 엘리먼트를 읽고 파싱하는 역할:

```rust
let parser = ElementParser::new(self.body());
```

### Command 패턴

wdpe는 Command 패턴을 사용하여 엘리먼트와 상호작용합니다:

- `ComboBoxSelectEventCommand`: ComboBox 선택 이벤트
- `ButtonPressEventCommand`: 버튼 클릭 이벤트
- `SapTableBodyCommand`: 테이블 본문 읽기
- `SapTableVerticalScrollEventCommand`: 테이블 스크롤

### 이벤트 처리

```rust
// 이벤트 생성
let event = parser.read(SomeCommand::new(...))?;

// 이벤트 처리 (서버로 전송)
self.client.process_event(false, event).await?;
```

## 테스트 작성

```rust
// tests/application/your_application.rs
use rusaint::application::your_application::YourApplication;
use rusaint::client::USaintClientBuilder;

#[tokio::test]
async fn test_fetch_data() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<YourApplication>()
        .await
        .unwrap();

    let data = app.fetch_data(2024, SemesterType::One).await;
    assert!(data.is_ok());
}
```

## FFI 바인딩 추가

다른 언어에서 사용하려면 `rusaint-ffi` 패키지에 바인딩을 추가합니다:

```rust
// packages/rusaint-ffi/src/application/your_application.rs
use rusaint::application::your_application::YourApplication as RusaintYourApplication;

#[derive(uniffi::Object)]
pub struct YourApplication {
    application: RwLock<RusaintYourApplication>,
}

#[uniffi::export]
impl YourApplication {
    #[uniffi::constructor]
    pub async fn new(session: Arc<USaintSession>) -> Result<Self, RusaintError> {
        let app = USaintClientBuilder::new()
            .session(session.inner().clone())
            .build_into::<RusaintYourApplication>()
            .await?;
        Ok(Self {
            application: RwLock::new(app),
        })
    }

    pub async fn fetch_data(&self, year: u32, semester: SemesterType) -> Result<Vec<YourModel>, RusaintError> {
        let mut app = self.application.write().await;
        Ok(app.fetch_data(year, semester).await?.collect())
    }
}
```

## Cookbook: 파싱 로직 구현 레시피

이 섹션에서는 기존 Application 구현에서 자주 사용되는 파싱 패턴들을 실용적인 예제와 함께 소개합니다.

### 레시피 1: InputField에서 값 읽기

InputField는 단일 값을 표시하는 텍스트 필드입니다. 학생 정보 페이지에서 학번, 이름 등을 읽을 때 사용됩니다.

**기본 사용법:**

```rust
use wdpe::{
    command::element::text::InputFieldValueCommand,
    define_elements,
    element::text::InputField,
};

impl<'a> YourModel {
    define_elements! {
        STUDENT_NAME: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.VORNA";
        STUDENT_NUMBER: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.STUDENT12";
    }

    pub fn with_parser(parser: &'a ElementParser) -> Result<Self, WebDynproError> {
        // 문자열 값 읽기
        let name = parser.read(InputFieldValueCommand::new(Self::STUDENT_NAME))?;
        
        // 숫자로 변환 (InputFieldExt 트레이트 사용)
        let student_number = parser
            .element_from_def(&Self::STUDENT_NUMBER)?
            .value_into_u32()?;
            
        // 선택적 값 처리 (값이 없을 수 있는 경우)
        let optional_value = parser
            .read(InputFieldValueCommand::new(Self::SOME_FIELD))
            .ok(); // Result를 Option으로 변환
            
        Ok(Self { name, student_number, optional_value })
    }
}
```

**InputFieldExt 트레이트 (rusaint 내부):**

```rust
use crate::application::utils::input_field::InputFieldExt;

// value_into_u32(): InputField 값을 u32로 변환
let year = parser.element_from_def(&Self::YEAR_FIELD)?.value_into_u32()?;

// value_into_f32(): InputField 값을 f32로 변환
let gpa = parser.element_from_def(&Self::GPA_FIELD)?.value_into_f32()?;
```

**실제 예시 (StudentInformation):**

```rust
// packages/rusaint/src/application/student_information/model.rs 참조
Ok(Self {
    apply_year: parser
        .element_from_def(&Self::APPLY_PERYR)?
        .value_into_u32()?,
    name: parser.read(InputFieldValueCommand::new(Self::VORNA))?,
    // 선택적 필드는 .ok()로 처리
    alias: parser.read(InputFieldValueCommand::new(Self::RUFNM)).ok(),
})
```

---

### 레시피 2: SapTable에서 데이터 파싱

SapTable은 u-saint에서 가장 흔하게 사용되는 데이터 표시 방식입니다.

**방법 1: FromSapTable 트레이트 + serde 역직렬화**

테이블 헤더를 기준으로 자동 매핑하는 가장 일반적인 방법입니다:

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize, de::{IntoDeserializer, value::MapDeserializer}};
use wdpe::element::complex::sap_table::FromSapTable;

#[derive(Debug, Serialize, Deserialize)]
pub struct LectureInfo {
    // 테이블 헤더 이름과 정확히 일치해야 함
    #[serde(rename(deserialize = "과목번호"))]
    code: String,
    
    #[serde(rename(deserialize = "과목명"))]
    name: String,
    
    // 선택적 필드 + 커스텀 역직렬화
    #[serde(
        rename(deserialize = "비고"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    remarks: Option<String>,
}

impl<'body> FromSapTable<'body> for LectureInfo {
    fn from_table(
        header: &'body SapTableHeader,
        row: &'body SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        // 행을 HashMap<String, String>으로 변환
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        
        // serde로 역직렬화
        let map_de: MapDeserializer<_, serde::de::value::Error> = 
            map_string.into_deserializer();
        
        Ok(Self::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
            element: row.table_def().id().to_string(),
            content: e.to_string(),
        })?)
    }
}
```

**방법 2: 테이블 직접 읽기 (스크롤 없이)**

작은 테이블이나 한 번에 모든 데이터가 표시되는 경우:

```rust
use wdpe::command::element::complex::SapTableBodyCommand;

let parser = ElementParser::new(self.body());
let table_body = parser.read(SapTableBodyCommand::new(Self::MAIN_TABLE))?;

// try_table_into로 전체 테이블을 Vec<T>로 변환
let items: Vec<YourModel> = table_body.try_table_into(&parser)?;
```

**방법 3: 스크롤이 필요한 큰 테이블**

화면에 표시되는 행 수보다 전체 데이터가 많은 경우:

```rust
use crate::application::utils::sap_table::try_table_into_with_scroll;

let parser = ElementParser::new(self.body());
let all_items = try_table_into_with_scroll::<YourModel>(
    &mut self.client,
    parser,
    Self::MAIN_TABLE,
).await?;
```

**테이블이 비어있는지 확인하기:**

u-saint는 데이터가 없을 때 "없습니다"라는 메시지를 포함한 단일 행을 표시합니다:

```rust
use wdpe::element::complex::sap_table::cell::{SapTableCell, SapTableCellWrapper};
use wdpe::element::ElementDefWrapper;

let table_body = parser.read(SapTableBodyCommand::new(Self::TABLE))?;

// 첫 번째 행 확인
let Some(first_row) = table_body.iter().next() else {
    return Err(ApplicationError::NoData.into());
};

// 첫 번째 셀의 내용 확인
if let Some(Ok(SapTableCellWrapper::Normal(cell))) = first_row.iter_value(&parser).next() {
    if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
        if let Ok(tv) = parser.element_from_def(&tv_def) {
            if tv.text().contains("없습니다.") {
                return Err(ApplicationError::NoData.into());
            }
        }
    }
}
```

**실제 예시 (ChapelAttendance):**

```rust
// packages/rusaint/src/application/chapel/model.rs 참조
pub(crate) fn with_parser(parser: &'a ElementParser) -> Result<Vec<Self>, WebDynproError> {
    let table = parser.read(SapTableBodyCommand::new(Self::TABLE_A))?;
    let Some(first_row) = table.iter().next() else {
        return Ok(Vec::with_capacity(0));
    };
    // 빈 테이블 확인 로직...
    table.try_table_into::<Self>(parser)
}
```

---

### 레시피 3: ComboBox 선택 및 값 읽기

ComboBox는 년도, 학기 등을 선택하는 드롭다운입니다.

**현재 선택된 값 읽기:**

```rust
use wdpe::command::element::selection::ComboBoxLSDataCommand;

let parser = ElementParser::new(self.body());
let combobox_data = parser.read(ComboBoxLSDataCommand::new(Self::YEAR_COMBO))?;

// 선택된 키 값 가져오기
if let Some(key) = combobox_data.key() {
    let year: u32 = key.parse()?;
}
```

**ComboBox 값 선택하기:**

```rust
use wdpe::command::element::selection::ComboBoxSelectEventCommand;

// 이벤트 생성
let select_event = parser.read(ComboBoxSelectEventCommand::new(
    Self::YEAR_COMBO,
    "2024",      // 선택할 키 값
    false,       // byEnter (엔터로 선택 여부)
))?;

// 서버에 이벤트 전송
self.client.process_event(false, select_event).await?;
```

**중복 요청 방지 (현재 값과 비교):**

```rust
async fn select_semester(&mut self, year: &str, semester: SemesterType) -> Result<(), RusaintError> {
    let semester_key = Self::semester_to_key(semester);
    let parser = ElementParser::new(self.body());
    
    // 현재 선택된 값 확인
    let year_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::SEL_PERYR))?;
    let semester_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::SEL_PERID))?;
    
    // 년도가 다른 경우에만 변경
    if year_lsdata.key().map(String::as_str) != Some(year) {
        let event = parser.read(ComboBoxSelectEventCommand::new(Self::SEL_PERYR, year, false))?;
        self.client.process_event(false, event).await?;
    }
    
    // 학기가 다른 경우에만 변경
    if semester_lsdata.key().map(String::as_str) != Some(semester_key) {
        let event = parser.read(ComboBoxSelectEventCommand::new(Self::SEL_PERID, semester_key, false))?;
        self.client.process_event(false, event).await?;
    }
    
    Ok(())
}
```

**학기 코드 매핑 (자주 사용되는 패턴):**

```rust
fn semester_to_key(semester: SemesterType) -> &'static str {
    match semester {
        SemesterType::One => "090",
        SemesterType::Summer => "091",
        SemesterType::Two => "092",
        SemesterType::Winter => "093",
    }
}
```

---

### 레시피 4: 버튼 클릭

조회 버튼 등을 클릭하여 데이터를 갱신할 때 사용합니다.

```rust
use wdpe::command::element::action::ButtonPressEventCommand;

define_elements! {
    BTN_SEARCH: Button<'a> = "ZCMW3681.ID_0001:V_MAIN.BTN_SEL";
}

pub async fn lookup(&mut self) -> Result<(), RusaintError> {
    let parser = ElementParser::new(self.body());
    let button_event = parser.read(ButtonPressEventCommand::new(Self::BTN_SEARCH))?;
    self.client.process_event(false, button_event).await?;
    Ok(())
}
```

---

### 레시피 5: 탭 전환

한 페이지에 여러 탭이 있는 경우 (예: 학생정보의 가족사항, 종교정보 등):

```rust
use wdpe::{
    command::element::layout::TabStripTabSelectEventCommand,
    define_elements,
    element::layout::{TabStrip, tab_strip::item::TabStripItem},
};

define_elements! {
    // 탭 스트립 컨테이너
    TAB_ADDITION: TabStrip<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_ADDITION";
    // 개별 탭 아이템
    TAB_FAMILY: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_FAMILY";
}

pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
    let parser = ElementParser::new(client.body());
    
    // 탭 선택 이벤트 생성
    let event = parser.read(TabStripTabSelectEventCommand::new(
        Self::TAB_ADDITION,  // 탭 스트립
        Self::TAB_FAMILY,    // 선택할 탭
        1,                   // firstVisibleItemIndex
        0,                   // scrollOffset
    ))?;
    
    client.process_event(false, event).await?;
    
    // 탭 전환 후 파서 재생성 필요
    let parser = ElementParser::new(client.body());
    // 이제 새 탭의 컨텐츠에 접근 가능
}
```

---

### 레시피 6: 팝업 창 처리

일부 페이지는 로딩 시 팝업이 표시됩니다 (예: 성적 조회):

```rust
use scraper::Selector;
use wdpe::element::{Element, ElementWrapper, layout::PopupWindow};

async fn close_popups(&mut self) -> Result<(), WebDynproError> {
    let popup_selector = Selector::parse(
        format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str()
    ).unwrap();
    
    fn make_close_event(body: &Body, selector: &Selector) -> Option<Event> {
        let parser = ElementParser::new(body);
        parser.document().select(selector).next().and_then(|elem| {
            let elem_wrapped = ElementWrapper::from_ref(elem).ok()?;
            if let ElementWrapper::PopupWindow(popup) = elem_wrapped {
                popup.close().ok()
            } else {
                None
            }
        })
    }
    
    // 모든 팝업이 닫힐 때까지 반복
    while let Some(event) = make_close_event(self.body(), &popup_selector) {
        self.client.process_event(false, event).await?;
    }
    Ok(())
}
```

---

### 레시피 7: 커스텀 역직렬화 함수

`serde`와 함께 사용하는 커스텀 역직렬화 함수들:

**선택적 문자열 (빈 문자열을 None으로):**

```rust
// packages/rusaint/src/application/utils/de_with.rs
pub(crate) fn deserialize_optional_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let binding = String::deserialize(deserializer)?;
    let value = binding.trim();
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_string()))
    }
}

// 사용 예시
#[serde(
    rename(deserialize = "비고"),
    default,
    deserialize_with = "deserialize_optional_string"
)]
remarks: Option<String>,
```

**문자열을 숫자로:**

```rust
pub(crate) fn deserialize_u32_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u32, D::Error> {
    let value = String::deserialize(deserializer)?;
    value.trim().parse().map_err(serde::de::Error::custom)
}

// 사용 예시
#[serde(
    rename(deserialize = "분반"),
    deserialize_with = "deserialize_u32_string"
)]
division: u32,
```

**문자열을 불리언으로:**

```rust
pub(crate) fn deserialize_bool_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(value.trim() == "true")
}
```

**학기 타입 역직렬화:**

```rust
pub(crate) fn deserialize_semester_type<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<SemesterType, D::Error> {
    let value = String::deserialize(deserializer)?;
    match value.trim() {
        "1 학기" | "1학기" => Ok(SemesterType::One),
        "여름학기" | "여름 학기" => Ok(SemesterType::Summer),
        "2 학기" | "2학기" => Ok(SemesterType::Two),
        "겨울학기" | "겨울 학기" => Ok(SemesterType::Winter),
        _ => Err(serde::de::Error::custom("Unknown SemesterType variant")),
    }
}
```

---

### 레시피 8: Model 구조 패턴

**패턴 A: with_parser (동기, Application에서 직접 호출)**

페이지 로드 시 바로 사용 가능한 데이터:

```rust
impl<'a> YourModel {
    pub(super) fn with_parser(parser: &'a ElementParser) -> Result<Self, WebDynproError> {
        // InputField 등에서 직접 데이터 읽기
        Ok(Self { /* ... */ })
    }
}

// Application에서 사용
pub fn get_data(&self) -> Result<YourModel, RusaintError> {
    Ok(YourModel::with_parser(&ElementParser::new(self.body()))?)
}
```

**패턴 B: with_client (비동기, 탭 전환 등 필요한 경우)**

UI 조작이 필요한 경우:

```rust
impl<'a> YourModel {
    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        // 탭 전환
        let parser = ElementParser::new(client.body());
        let event = parser.read(TabStripTabSelectEventCommand::new(/* ... */))?;
        client.process_event(false, event).await?;
        
        // 데이터 읽기
        let parser = ElementParser::new(client.body());
        // ...
    }
}

// Application에서 사용
pub async fn get_data(&mut self) -> Result<YourModel, RusaintError> {
    Ok(YourModel::with_client(&mut self.client).await?)
}
```

**패턴 C: 페이지 내 여러 테이블이 각자의 엘리먼트 정의를 가지는 경우**

```rust
// GeneralChapelInformation은 자체 TABLE 정의를 가짐
impl<'a> GeneralChapelInformation {
    define_elements! {
        TABLE: SapTable<'a> = "ZCMW3681.ID_0001:V_MAIN.TABLE";
    }
    
    pub(crate) fn with_parser(parser: &'a ElementParser) -> Result<Vec<Self>, RusaintError> {
        let table = parser.read(SapTableBodyCommand::new(Self::TABLE))?;
        // ...
    }
}

// ChapelAttendance는 다른 TABLE_A 정의를 가짐
impl<'a> ChapelAttendance {
    define_elements! {
        TABLE_A: SapTable<'a> = "ZCMW3681.ID_0001:V_MAIN.TABLE_A";
    }
    
    pub(crate) fn with_parser(parser: &'a ElementParser) -> Result<Vec<Self>, WebDynproError> {
        let table = parser.read(SapTableBodyCommand::new(Self::TABLE_A))?;
        // ...
    }
}
```

---

### 레시피 9: 이벤트 처리 후 파서 재생성

**중요**: 이벤트를 처리한 후에는 항상 새로운 `ElementParser`를 생성해야 합니다. 이벤트 처리로 인해 페이지 DOM이 변경되기 때문입니다.

```rust
pub async fn fetch_data(&mut self, year: u32, semester: SemesterType) -> Result<Vec<Data>, RusaintError> {
    // 1단계: 학기 선택 (이벤트 처리)
    {
        let parser = ElementParser::new(self.body());
        self.select_semester(&parser, &year.to_string(), semester).await?;
    } // 파서 스코프 종료
    
    // 2단계: 새 파서로 데이터 읽기
    let parser = ElementParser::new(self.body());  // ← 새로 생성 필수!
    let data = try_table_into_with_scroll::<Data>(
        &mut self.client,
        parser,
        Self::MAIN_TABLE,
    ).await?;
    
    Ok(data)
}
```

---

## 참고 자료

- [wdpe 문서](https://docs.rs/wdpe) - WebDynpro Parse Engine 공식 문서
- [rusaint 소스 코드](https://github.com/example/rusaint) - 기존 Application 구현 참조
- [SAP WebDynpro 문서](https://help.sap.com/docs/ABAP_PLATFORM_NEW/f2e545608079437ab165c105649b89db/4e7f2a2bf85a0e21e10000000a42189c.html) - WebDynpro 프레임워크 이해
