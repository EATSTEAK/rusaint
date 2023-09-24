use rusaint::{
    definitions::course_schedule::CourseSchedule,
    webdynpro::element::{
        sap_table::cell::{SapTableCell, SapTableCells},
        Element, Elements,
    },
};

#[tokio::test]
async fn initial_load() {
    let mut app = CourseSchedule::new().await.unwrap();
    app.load_placeholder().await.unwrap();
}

#[tokio::test]
async fn edu_data() {
    let mut app = CourseSchedule::new().await.unwrap();
    app.load_placeholder().await.unwrap();
    app.load_edu().await.unwrap();
    let table = app.read_edu_raw().unwrap();
    if let Some(table) = table.table() {
        for row in table {
            print!("row: ");
            for col in row {
                match col {
                    SapTableCells::Header(cell) => {
                        let content = cell.content();
                        print!("Header: ");
                        match content {
                            None => {
                                print!("None, ")
                            }
                            Some(Elements::Caption(elem)) => {
                                print!("Caption {{ {:?} }}, ", elem.text());
                            }
                            Some(Elements::TextView(elem)) => {
                                print!("TextView {{ {:?} }}, ", elem.text());
                            }
                            Some(Elements::Unknown(elem)) => {
                                print!("Unknown {{ {:?} }}, ", elem.lsdata());
                            }
                            Some(Elements::Button(elem)) => {
                                print!("Button {{ {:?} }}, ", elem.lsdata());
                            }
                            _ => {}
                        };
                    }
                    SapTableCells::Normal(cell) => {
                        let content = cell.content();
                        match content {
                            None => {
                                print!("None, ")
                            }
                            Some(Elements::Caption(elem)) => {
                                print!("Caption {{ {:?} }},", elem.text());
                            }
                            Some(Elements::TextView(elem)) => {
                                print!("TextView {{ {:?} }},", elem.text());
                            }
                            Some(Elements::Unknown(elem)) => {
                                print!("Unknown {{ {:?} }}, ", elem.lsdata());
                            }
                            Some(Elements::Button(elem)) => {
                                print!("Button {{ {:?} }}, ", elem.lsdata());
                            }
                            _ => {}
                        };
                    }
                    _ => {
                        print!("{:?}, ", col);
                    }
                }
            }
            println!("");
        }
    }
    assert!(false);
}
