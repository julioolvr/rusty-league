//! Main entrypoint for querying the Rocket League API.
//! The purpose of this module is to query and return the responses
//! with as little handling as possible, without working around
//! some of the API inconsistencies, nor providing any higher-level
//! models as a response.

pub mod responses;
pub mod client;
pub mod errors;
