use rusaint::{application::USaintApplication, define_usaint_application};

define_usaint_application!(pub(crate) struct EventTestSuite<"WDR_TEST_EVENTS">);

mod button;
