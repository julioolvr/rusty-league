// TODO: Consider moving this somewhere else
pub type PlayerId = String;
pub type PlayerTitle = String;

#[derive(Deserialize, Debug)]
pub struct PlayerSkillResponse {
    user_id: Option<String>,
    user_name: String,
    player_skills: Vec<PlaylistSkillResponse>,
    // TODO: Add season_rewards field
}

#[derive(Deserialize, Debug)]
struct PlaylistSkillResponse {
    playlist: i64,
    tier: Option<i64>,
    tier_max: Option<i64>,
    division: Option<i64>,
    skill: Option<i64>,
    mu: f64,
    sigma: f64,
    win_streak: Option<i64>,
    matches_played: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct PlayerTitlesResponse {
    titles: Vec<PlayerTitle>,
}

#[derive(Deserialize, Debug)]
pub struct PopulationResponse {
    #[serde(rename = "XboxOne")]
    xbox_one: Vec<PlatformPopulationResponse>,
    #[serde(rename = "Switch")]
    switch: Vec<PlatformPopulationResponse>,
    #[serde(rename = "Steam")]
    steam: Vec<PlatformPopulationResponse>,
    #[serde(rename = "PS4")]
    ps4: Vec<PlatformPopulationResponse>,
}

#[derive(Deserialize, Debug)]
pub struct PlatformPopulationResponse {
    #[serde(rename = "PlaylistID")]
    playlist: i64,
    #[serde(rename = "NumPlayers")]
    num_players: i64,
}

#[derive(Deserialize, Debug)]
pub struct RegionResponse {
    platforms: String,
    region: String, // TODO: Consider an enum
}

#[derive(Deserialize, Debug)]
pub struct SkillLeaderboardResponse {
    user_id: Option<String>,
    user_name: String,
    tier: i64,
    skill: i64,
}

#[derive(Deserialize, Debug)]
pub struct StatLeaderboardResponse {
    stats: Vec<PlayerStatResponse>,
    stat_type: String,
}

#[derive(Deserialize, Debug)]
pub struct PlayerStatResponse {
    user_id: Option<String>,
    user_name: String,
    assists: Option<i64>,
    goals: Option<i64>,
    mvps: Option<i64>,
    saves: Option<i64>,
    shots: Option<i64>,
    wins: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct StatValueForUserResponse {
    user_id: Option<String>,
    user_name: Option<String>,
    stat_type: String,
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct StatValueForUserMultipleResponse {
    user_id: Option<String>,
    user_name: Option<String>,
    stat_type: String,
    value: i64,
}