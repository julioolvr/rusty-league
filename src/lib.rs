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

#[derive(Serialize)]
pub struct PlayersQueryBody {
    player_ids: Vec<PlayerId>,
}

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
        self.make_get_request(path)
    }

    pub fn get_players_skills(&self,
                              platform: Platform,
                              player_ids: Vec<PlayerId>)
                              -> Result<Vec<PlayerSkillResponse>, Error> {
        let path = format!("/api/v1/{}/playerskills", platform.code());
        let query = PlayersQueryBody { player_ids };
        self.make_post_request(path, serde_json::to_string(&query).unwrap())
    }

    pub fn get_player_titles(&self,
                             platform: Platform,
                             player_id: PlayerId)
                             -> Result<PlayerTitlesResponse, Error> {
        let path = format!("/api/v1/{}/playertitles/{}", platform.code(), player_id);
        self.make_get_request(path)
    }

    pub fn get_population(&self) -> Result<PopulationResponse, Error> {
        let path = "/api/v1/population".to_string();
        self.make_get_request(path)
    }

    pub fn get_regions(&self) -> Result<Vec<RegionResponse>, Error> {
        let path = "/api/v1/regions".to_string();
        self.make_get_request(path)
    }

    pub fn get_skill_leaderboard(&self,
                                 platform: Platform,
                                 playlist: i64)
                                 -> Result<Vec<SkillLeaderboardResponse>, Error> {
        let path = format!("/api/v1/{}/leaderboard/skills/{}",
                           platform.code(),
                           playlist);
        self.make_get_request(path)
    }

    pub fn get_stats_leaderboard(&self,
                                 platform: Platform)
                                 -> Result<Vec<StatLeaderboardResponse>, Error> {
        let path = format!("/api/v1/{}/leaderboard/stats", platform.code());
        self.make_get_request(path)
    }

    pub fn get_stat_leaderboard(&self,
                                platform: Platform,
                                stat: &str)
                                -> Result<Vec<StatLeaderboardResponse>, Error> {
        let path = format!("/api/v1/{}/leaderboard/stats/{}", platform.code(), stat);
        self.make_get_request(path)
    }

    pub fn get_stat_value_for_user(&self,
                                   platform: Platform,
                                   stat: &str,
                                   player_id: PlayerId)
                                   -> Result<Vec<StatValueForUserResponse>, Error> {
        let path = format!("/api/v1/{}/leaderboard/stats/{}/{}",
                           platform.code(),
                           stat,
                           player_id);
        self.make_get_request(path)
    }

    fn make_get_request<T>(&self, path: String) -> Result<T, Error>
        where T: serde::de::DeserializeOwned
    {
        self.make_request(path, hyper::Method::Get, None)
    }

    fn make_post_request<T>(&self, path: String, body: String) -> Result<T, Error>
        where T: serde::de::DeserializeOwned
    {
        self.make_request(path, hyper::Method::Post, Some(body))
    }

    fn make_request<T>(&self,
                       path: String,
                       method: hyper::Method,
                       body: Option<String>)
                       -> Result<T, Error>
        where T: serde::de::DeserializeOwned
    {
        let uri = format!("{}{}", BASE_URL, path)
            .parse()
            .map_err(|_| Error::Internal)?;

        let mut request: hyper::Request<hyper::Body> = hyper::Request::new(method, uri);
        request
            .headers_mut()
            .set(hyper::header::Authorization(format!("Token {}", self.token)));

        if let Some(body) = body {
            request.set_body(body)
        }

        let work =
            self.client
                .request(request)
                .and_then(|res| res.body().concat2())
                .map(move |body: Chunk| serde_json::from_slice(&body).map_err(|_| Error::Internal));

        self.core
            .borrow_mut()
            .run(work)
            .map_err(|_| Error::Internal)?
    }
}
