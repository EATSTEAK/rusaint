use rusaint::define_usaint_application;

define_usaint_application!(pub(crate) struct EventTestSuite<"WDR_TEST_EVENTS">; pub(crate) type EventTestSuiteBuilder;);

mod button;
