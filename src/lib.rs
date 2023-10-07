#[cfg(feature = "application")]
pub mod application;
#[cfg(feature = "application")]
mod error;
#[cfg(feature = "application")]
pub use error::RusaintError;
#[cfg(feature = "application")]
pub use error::SsuSsoError;
#[cfg(feature = "application")]
mod session;

#[cfg(feature = "application")]
pub use session::obtain_ssu_sso_token;
#[cfg(feature = "application")]
pub use session::USaintSession;

#[cfg(feature = "model")]
pub mod model;

mod utils;
pub mod webdynpro;
