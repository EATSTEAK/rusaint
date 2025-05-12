use serde::Deserialize;
use std::fmt::Display;

/// 테이블 내의 셀 접근 방식
#[allow(missing_docs)]
pub enum AccessType {
    Invalid,
    Standard,
    Range,
    Toggle,
    SelectAll,
    DeselectAll,
}

impl Display for AccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AccessType::Invalid => "INVALID",
            AccessType::Standard => "STANDARD",
            AccessType::Range => "RANGE",
            AccessType::Toggle => "TOGGLE",
            AccessType::SelectAll => "SELECT_ALL",
            AccessType::DeselectAll => "DESELECT_ALL",
        }
        .to_owned();
        write!(f, "{str}")
    }
}

/// 테이블 셀의 형태 종류
#[allow(missing_docs)]
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SapTableCellDesign {
    Standard,
    Alternating,
    Transparent,
    Negative,
    Positive,
    Total,
    Subtotal,
    SubtotalLight,
    #[serde(rename = "BADVALUE_DARK")]
    BadValueDark,
    #[serde(rename = "BADVALUE_MEDIUM")]
    BadValueMedium,
    #[serde(rename = "BADVALUE_LIGHT")]
    BadValueLight,
    Critical,
    #[serde(rename = "CRITICALVALUE_DARK")]
    CriticalValueDark,
    #[serde(rename = "CRITICALVALUE_MEDIUM")]
    CriticalValueMedium,
    #[serde(rename = "CRITICALVALUE_LIGHT")]
    CriticalValueLight,
    #[serde(rename = "GOODVALUE_DARK")]
    GoodValueDark,
    #[serde(rename = "GOODVALUE_MEDIUM")]
    GoodValueMedium,
    #[serde(rename = "GOODVALUE_LIGHT")]
    GoodValueLight,
    GroupHighlighted,
    GroupHighlightedLight,
    KeyMedium,
    GroupLevel1,
    GroupLevel2,
    GroupLevel3,
    Marked,
    Filter,
    Filtericon,
    Popin,
    None,
    Disabled,
    Today,
    Selected1,
    Selected2,
    Selected3,
    Selected4,
    Selected5,
    SecondarySelected,
    CalendarMetal,
    CalendarPeach,
    CalendarBlue,
    CalendarRose,
    CalendarPurple,
    CalendarAqua,
    CalendarTeal,
    CalendarYellow,
    CalendarBrown,
    CalendarDefault,
    CalendarGreen,
    Intensified,
    ColorBlack,
    ColorBrown,
    ColorOliveGreen,
    ColorDarkGreen,
    ColorDarkTeal,
    ColorDarkBlue,
    ColorIndigo,
    #[serde(rename = "COLOR_GRAY_80")]
    ColorGray80,
    ColorDarkRed,
    ColorOrange,
    ColorDarkYellow,
    ColorGreen,
    ColorTeal,
    ColorBlue,
    ColorBlueGray,
    #[serde(rename = "COLOR_GRAY_50")]
    ColorGray50,
    ColorRed,
    ColorLightOrange,
    ColorLime,
    ColorSeaGreen,
    ColorAqua,
    ColorLightBlue,
    ColorViolet,
    #[serde(rename = "COLOR_GRAY_40")]
    ColorGray40,
    ColorPink,
    ColorGold,
    ColorYellow,
    ColorBrightGreen,
    ColorTurquoise,
    ColorSkyBlue,
    ColorPlum,
    #[serde(rename = "COLOR_GRAY_25")]
    ColorGray25,
    ColorRose,
    ColorTan,
    ColorLightYellow,
    ColorLightGreen,
    ColorLightTurquoise,
    ColorPaleBlue,
    ColorLavender,
    ColorWhite,
    #[serde(rename = "COLOR_A_1")]
    ColorA1,
    #[serde(rename = "COLOR_A_2")]
    ColorA2,
    #[serde(rename = "COLOR_A_3")]
    ColorA3,
    #[serde(rename = "COLOR_A_4")]
    ColorA4,
    #[serde(rename = "COLOR_A_5")]
    ColorA5,
    #[serde(rename = "COLOR_A_6")]
    ColorA6,
    #[serde(rename = "COLOR_A_7")]
    ColorA7,
    #[serde(rename = "COLOR_A_8")]
    ColorA8,
    #[serde(rename = "COLOR_A_9")]
    ColorA9,
    #[serde(rename = "COLOR_B_1")]
    ColorB1,
    #[serde(rename = "COLOR_B_2")]
    ColorB2,
    #[serde(rename = "COLOR_B_3")]
    ColorB3,
    #[serde(rename = "COLOR_B_4")]
    ColorB4,
    #[serde(rename = "COLOR_B_5")]
    ColorB5,
    #[serde(rename = "COLOR_B_6")]
    ColorB6,
    #[serde(rename = "COLOR_B_7")]
    ColorB7,
    #[serde(rename = "COLOR_B_8")]
    ColorB8,
    #[serde(rename = "COLOR_B_9")]
    ColorB9,
    #[serde(rename = "COLOR_C_1")]
    ColorC1,
    #[serde(rename = "COLOR_C_2")]
    ColorC2,
    #[serde(rename = "COLOR_C_3")]
    ColorC3,
    #[serde(rename = "COLOR_C_4")]
    ColorC4,
    #[serde(rename = "COLOR_C_5")]
    ColorC5,
    #[serde(rename = "COLOR_C_6")]
    ColorC6,
    #[serde(rename = "COLOR_C_7")]
    ColorC7,
    #[serde(rename = "COLOR_C_8")]
    ColorC8,
    #[serde(rename = "COLOR_C_9")]
    ColorC9,
    #[serde(rename = "COLOR_D_1")]
    ColorD1,
    #[serde(rename = "COLOR_D_2")]
    ColorD2,
    #[serde(rename = "COLOR_D_3")]
    ColorD3,
    #[serde(rename = "COLOR_D_4")]
    ColorD4,
    #[serde(rename = "COLOR_D_5")]
    ColorD5,
    #[serde(rename = "COLOR_D_6")]
    ColorD6,
    #[serde(rename = "COLOR_D_7")]
    ColorD7,
    #[serde(rename = "COLOR_D_8")]
    ColorD8,
    #[serde(rename = "COLOR_D_9")]
    ColorD9,
    #[serde(rename = "COLOR_E_1")]
    ColorE1,
    #[serde(rename = "COLOR_E_2")]
    ColorE2,
    #[serde(rename = "COLOR_E_3")]
    ColorE3,
    #[serde(rename = "COLOR_E_4")]
    ColorE4,
    #[serde(rename = "COLOR_E_5")]
    ColorE5,
    #[serde(rename = "COLOR_E_6")]
    ColorE6,
    #[serde(rename = "COLOR_E_7")]
    ColorE7,
    #[serde(rename = "COLOR_E_8")]
    ColorE8,
    #[serde(rename = "COLOR_E_9")]
    ColorE9,
    ColBackgroundIntensified,
    ColHeadingIntensified,
    ColNormalIntensified,
    ColTotalIntensified,
    ColKeyIntensified,
    ColPositiveIntensified,
    ColNegativeIntensified,
    ColGroupIntensified,
    ColInactiveIntensified,
    ColBackground,
    ColHeading,
    ColNormal,
    ColTotal,
    ColKey,
    ColPositive,
    ColNegative,
    ColGroup,
    ColInactive,
    #[serde(rename = "CALENDAR_1_INTENSIFIED")]
    Calendar1Intensified,
    #[serde(rename = "CALENDAR_2_INTENSIFIED")]
    Calendar2Intensified,
    #[serde(rename = "CALENDAR_3_INTENSIFIED")]
    Calendar3Intensified,
    #[serde(rename = "CALENDAR_4_INTENSIFIED")]
    Calendar4Intensified,
    #[serde(rename = "CALENDAR_5_INTENSIFIED")]
    Calendar5Intensified,
    #[serde(rename = "CALENDAR_6_INTENSIFIED")]
    Calendar6Intensified,
    #[serde(rename = "CALENDAR_7_INTENSIFIED")]
    Calendar7Intensified,
    #[serde(rename = "CALENDAR_8_INTENSIFIED")]
    Calendar8Intensified,
    #[serde(rename = "CALENDAR_1")]
    Calendar1,
    #[serde(rename = "CALENDAR_2")]
    Calendar2,
    #[serde(rename = "CALENDAR_3")]
    Calendar3,
    #[serde(rename = "CALENDAR_4")]
    Calendar4,
    #[serde(rename = "CALENDAR_5")]
    Calendar5,
    #[serde(rename = "CALENDAR_6")]
    Calendar6,
    #[serde(rename = "CALENDAR_7")]
    Calendar7,
    #[serde(rename = "CALENDAR_8")]
    Calendar8,
}

/// 테이블 셀의 종류
#[allow(missing_docs)]
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SapTableCellType {
    Text,
    Edit,
    EmptyRow,
    EmptyCell,
    Format,
    RowCreator,
    Info,
}

/// [`SapTableHeaderCell`](super::cell::SapTableHeaderCell)의 형태 종류
#[allow(missing_docs)]
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SapTableHeaderCellDesign {
    Level1Header,
    Level2Header,
    Level3Header,
}

/// [`SapTableHeaderCell`](super::cell::SapTableHeaderCell)의 종류
#[allow(missing_docs)]
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SapTableHeaderCellType {
    Standard,
    SelectionColumn,
}

/// [`SapTableHierarchicalCell`](super::cell::SapTableHierarchicalCell)의 상태
#[allow(missing_docs)]
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SapTableHierarchicalCellStatus {
    None,
    Indent,
    Expanded,
    Collapsed,
    Collapsedplus,
    Expandedminus,
    Expandedtop,
    Marktotals,
    Leaf,
    Icon,
}

/// 테이블의 선택 상태
#[allow(missing_docs)]
#[derive(Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SapTableSelectionState {
    NotSelectable,
    NotSelected,
    Selected,
    PrimarySelected,
    #[default]
    None,
}

/// 테이블 내부 행의 전체적인 선택 상태
#[allow(missing_docs)]
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SapTableRowSelectionMassState {
    None,
    Indeterminate,
    All,
}

/// 테이블 행의 종류
#[allow(missing_docs)]
#[derive(Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SapTableRowType {
    #[default]
    Unspecified,
    Standard,
    Header,
    Filter,
    TopFixed,
    BottomFixed,
    Pivot,
}

/// 테이블 열 선택시의 액션
#[allow(missing_docs)]
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SapTableSelectionColumnAction {
    None,
    SelectionMenu,
    SelectionToggle,
}
