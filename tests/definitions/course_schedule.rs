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
async fn examine_elements() {
    let mut app = CourseSchedule::new().await.unwrap();
    app.load_placeholder().await.unwrap();
    let ct_selector = scraper::Selector::parse("[ct]").unwrap();
    for elem_ref in app.body().document().select(&ct_selector) {
        let elem = Elements::dyn_elem(elem_ref);
        if let Ok(elem) = elem {
            println!("{:?}", elem);
        }
    }
    assert!(false);
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
                        if let Some(elem) = content { print!("{:?}, ", elem); }
                    }
                    SapTableCells::Normal(cell) => {
                        let content = cell.content();
                        if let Some(elem) = content { print!("{:?}, ", elem); }
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
