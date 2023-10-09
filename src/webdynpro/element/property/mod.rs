use serde::Deserialize;

use super::complex::sap_table::property::SapTableCellDesign;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TextDesign {
    Standard,
    Monospace,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TextDirection {
    Ltr,
    Rtl,
    Inherit,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visibility {
    Visible,
    Blank,
    None,
    OnDemand,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContentVisibility {
    All,
    Text,
    Icon,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HotkeyValue {
    None,
    #[serde(rename = "CTRL_0")]
    Ctrl0,
    #[serde(rename = "CTRL_1")]
    Ctrl1,
    #[serde(rename = "CTRL_2")]
    Ctrl2,
    #[serde(rename = "CTRL_3")]
    Ctrl3,
    #[serde(rename = "CTRL_4")]
    Ctrl4,
    #[serde(rename = "CTRL_5")]
    Ctrl5,
    #[serde(rename = "CTRL_6")]
    Ctrl6,
    #[serde(rename = "CTRL_7")]
    Ctrl7,
    #[serde(rename = "CTRL_8")]
    Ctrl8,
    #[serde(rename = "CTRL_9")]
    Ctrl9,
    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlG,
    CtrlH,
    CtrlI,
    CtrlJ,
    CtrlK,
    CtrlL,
    CtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    CtrlF1,
    CtrlF2,
    CtrlF3,
    CtrlF4,
    CtrlF5,
    CtrlF6,
    CtrlF7,
    CtrlF8,
    CtrlF9,
    CtrlF10,
    CtrlF11,
    CtrlF12,
    CtrlAltN,
    #[serde(rename = "CTRL_SHIFT_0")]
    CtrlShift0,
    #[serde(rename = "CTRL_SHIFT_1")]
    CtrlShift1,
    #[serde(rename = "CTRL_SHIFT_2")]
    CtrlShift2,
    #[serde(rename = "CTRL_SHIFT_3")]
    CtrlShift3,
    #[serde(rename = "CTRL_SHIFT_4")]
    CtrlShift4,
    #[serde(rename = "CTRL_SHIFT_5")]
    CtrlShift5,
    #[serde(rename = "CTRL_SHIFT_6")]
    CtrlShift6,
    #[serde(rename = "CTRL_SHIFT_7")]
    CtrlShift7,
    #[serde(rename = "CTRL_SHIFT_8")]
    CtrlShift8,
    #[serde(rename = "CTRL_SHIFT_9")]
    CtrlShift9,
    CtrlShiftA,
    CtrlShiftB,
    CtrlShiftC,
    CtrlShiftD,
    CtrlShiftE,
    CtrlShiftF,
    CtrlShiftG,
    CtrlShiftH,
    CtrlShiftI,
    CtrlShiftJ,
    CtrlShiftK,
    CtrlShiftL,
    CtrlShiftM,
    CtrlShiftN,
    CtrlShiftO,
    CtrlShiftP,
    CtrlShiftQ,
    CtrlShiftR,
    CtrlShiftS,
    CtrlShiftT,
    CtrlShiftU,
    CtrlShiftV,
    CtrlShiftW,
    CtrlShiftX,
    CtrlShiftY,
    CtrlShiftZ,
    CtrlShiftF1,
    CtrlShiftF2,
    CtrlShiftF3,
    CtrlShiftF4,
    CtrlShiftF5,
    CtrlShiftF6,
    CtrlShiftF7,
    CtrlShiftF8,
    CtrlShiftF9,
    CtrlShiftF10,
    CtrlShiftF11,
    CtrlShiftF12,
    Escape,
    Delete,
    Insert,
    Enter,
    CtrlDot,
    CtrlComma,
    AltArrowLeft,
    AltArrowUp,
    AltArrowRight,
    AltArrowDown,
    ShiftF1,
    ShiftF2,
    ShiftF3,
    ShiftF4,
    ShiftF5,
    ShiftF6,
    ShiftF7,
    ShiftF8,
    ShiftF9,
    ShiftF10,
    ShiftF11,
    ShiftF12,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum HorizontalTextAlign {
    Center,
    Left,
    Right,
    Justify,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LockedDesign {
    Transparent,
    Border,
    Loading,
    BorderLoading,
    Semi,
    SemiBorder,
    SemiLoading,
    SemiBorderLoading,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Mode {
    Modal,
    Amodal,
    Floating,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScrollingMode {
    Auto,
    Both,
    Hide,
    None,
    Horizontal,
    HorizontalAuto,
    Vertical,
    VerticalAuto,
}

pub type SemanticColor = SapTableCellDesign;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortState {
    Disabled,
    None,
    Ascending,
    Descending,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum VerticalTextAlign {
    Middle,
    Baseline,
    Top,
    Bottom,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum QuickViewDesign {
    Standard,
    Application,
}
