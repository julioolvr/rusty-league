// TODO: Consider moving this somewhere else
pub type PlayerId = String;
pub type PlayerTitle = String;

#[derive(Deserialize, Debug)]
pub struct PlayerSkillResponse {
    pub user_id: Option<String>,
    pub user_name: String,
    pub player_skills: Vec<PlaylistSkillResponse>,
    // TODO: Add season_rewards field
}

#[derive(Deserialize, Debug)]
pub struct PlaylistSkillResponse {
    pub playlist: i64,
    pub tier: Option<i64>,
    pub tier_max: Option<i64>,
    pub division: Option<i64>,
    pub skill: Option<i64>,
    pub mu: f64,
    pub sigma: f64,
    pub win_streak: Option<i64>,
    pub matches_played: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct PlayerTitlesResponse {
    pub titles: Vec<PlayerTitle>,
}

#[derive(Deserialize, Debug)]
pub struct PopulationResponse {
    #[serde(rename = "XboxOne")]
    pub xbox_one: Vec<PlatformPopulationResponse>,
    #[serde(rename = "Switch")]
    pub switch: Vec<PlatformPopulationResponse>,
    #[serde(rename = "Steam")]
    pub steam: Vec<PlatformPopulationResponse>,
    #[serde(rename = "PS4")]
    pub ps4: Vec<PlatformPopulationResponse>,
}

#[derive(Deserialize, Debug)]
pub struct PlatformPopulationResponse {
    #[serde(rename = "PlaylistID")]
    pub playlist: i64,
    #[serde(rename = "NumPlayers")]
    pub num_players: i64,
}

#[derive(Deserialize, Debug)]
pub struct RegionResponse {
    pub platforms: String,
    pub region: String, // TODO: Consider an enum
}

#[derive(Deserialize, Debug)]
pub struct SkillLeaderboardResponse {
    pub user_id: Option<String>,
    pub user_name: String,
    pub tier: i64,
    pub skill: i64,
}

#[derive(Deserialize, Debug)]
pub struct StatLeaderboardResponse {
    pub stats: Vec<PlayerStatResponse>,
    pub stat_type: String,
}

#[derive(Deserialize, Debug)]
pub struct PlayerStatResponse {
    pub user_id: Option<String>,
    pub user_name: String,
    pub assists: Option<i64>,
    pub goals: Option<i64>,
    pub mvps: Option<i64>,
    pub saves: Option<i64>,
    pub shots: Option<i64>,
    pub wins: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct StatValueForUserResponse {
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub stat_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct StatValueForUserMultipleResponse {
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub stat_type: String,
    pub value: i64,
}