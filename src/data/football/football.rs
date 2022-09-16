pub struct FootballLeague {
    pub country: String,
}

impl std::fmt::Display for FootballLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.country)
    }
}
