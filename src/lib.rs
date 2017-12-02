extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod errors;
mod platform;
mod responses;

use std::cell::RefCell;
use futures::{Future, Stream};
use hyper::{Client, Chunk};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use errors::Error;
pub use platform::Platform;
pub use responses::*;

const BASE_URL: &'static str = "https://api.rocketleague.com";

pub struct LeagueClient {
    token: String,
    core: RefCell<Core>,
    client: Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl LeagueClient {
    pub fn new(token: String) -> Result<LeagueClient, Error> {
        let core = Core::new().map_err(|_| Error::Internal)?;
        let handle = core.handle();

        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle)
                           .map_err(|_| Error::Internal)?)
            .build(&handle);

        Ok(LeagueClient {
               token,
               core: RefCell::new(core),
               client,
           })
    }

    pub fn get_player_skills(&self,
                             platform: Platform,
                             player_id: PlayerId)
                             -> Result<Vec<PlayerSkillResponse>, Error> {
        let path = format!("/api/v1/{}/playerskills/{}", platform.code(), player_id);
        self.make_request(path)
    }

    pub fn get_player_titles(&self,
                             platform: Platform,
                             player_id: PlayerId)
                             -> Result<PlayerTitlesResponse, Error> {
        let path = format!("/api/v1/{}/playertitles/{}", platform.code(), player_id);
        self.make_request(path)
    }

    pub fn get_population(&self) -> Result<PopulationResponse, Error> {
        let path = "/api/v1/population".to_string();
        self.make_request(path)
    }

    fn make_request<T>(&self, path: String) -> Result<T, Error>
        where T: serde::de::DeserializeOwned
    {
        let uri = format!("{}{}", BASE_URL, path)
            .parse()
            .map_err(|_| Error::Internal)?;

        let mut request: hyper::Request<hyper::Body> = hyper::Request::new(hyper::Method::Get, uri);
        request
            .headers_mut()
            .set(hyper::header::Authorization(format!("Token {}", self.token)));

        let work = self.client
            .request(request)
            .and_then(|res| res.body().concat2())
            .and_then(move |body: Chunk| {
                          // TODO: Map the error here instead and return it instead of unwrapping
                          let v = serde_json::from_slice(&body).unwrap();
                          Ok(v)
                      });

        self.core
            .borrow_mut()
            .run(work)
            .map_err(|_| Error::Internal)
    }
}
