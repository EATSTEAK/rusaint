use super::basic_element_def;

basic_element_def!(FlowLayout("FL", FlowLayoutLSData), {
    visibility: String -> "0",
    custom_data: String -> "1"
});

basic_element_def!(Container("CO", ContainerLSData), {
    locked: bool -> "0",
    printable: bool -> "1",
    print_area: bool -> "2",
    locked_design: String -> "3",
    locked_message: String -> "4",
    custom_data: String -> "5",
    custom_style: String -> "6"
});

pub mod grid_layout;
pub mod scroll_container;
