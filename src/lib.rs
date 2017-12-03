extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod platform;
pub mod api;

pub use platform::Platform;
pub use api::client::LeagueClient;
