/// Represents the platforms that the API can be queried for.
pub enum Platform {
    Steam,
    Xbox,
    Playstation,
}

impl Platform {
    /// Get the id that has to be used when querying for each platform.
    pub fn code(&self) -> String {
        match self {
                &Platform::Steam => "steam",
                &Platform::Xbox => "xboxone",
                &Platform::Playstation => "ps4",
            }
            .to_string()
    }
}