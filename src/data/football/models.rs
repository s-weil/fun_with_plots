use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FootballPlayer {
    #[serde(default)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStats {
    pub games: Option<PlayerStatsGames>,
    pub goals: Option<PlayerStatsGoals>,
    pub cards: Option<PlayerStatsCards>,
    pub duels: Option<PlayerStatsDuels>,
    pub passes: Option<PlayerStatsPasses>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStatsGames {
    pub minutes: Option<i32>,
    pub appearences: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStatsCards {
    pub yellow: Option<i32>,
    pub yellowred: Option<i32>,
    pub red: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStatsDuels {
    pub total: Option<i32>,
    pub won: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStatsPasses {
    pub total: Option<i32>,
    pub accuracy: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStatsGoals {
    pub total: Option<i32>,
    pub conceded: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SeasonPlayerResults {
    pub player: FootballPlayer,
    pub statistics: Vec<PlayerStats>,
}

#[derive(Debug, Deserialize)]
pub struct FootballSeasonResults {
    #[serde(default)]
    pub season: i32,
    #[serde(rename = "playerResults")]
    pub player_results: Vec<SeasonPlayerResults>,
}
