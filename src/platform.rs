pub enum Platform {
    Steam,
    Xbox,
    Playstation,
}

impl Platform {
    pub fn code(&self) -> String {
        match self {
                &Platform::Steam => "steam",
                &Platform::Xbox => "xboxone",
                &Platform::Playstation => "ps4",
            }
            .to_string()
    }
}