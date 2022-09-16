mod data_provider;
pub mod football;
pub mod models;
pub mod weather;

pub use data_provider::{DataProvider, FootballProvider, WeatherProvider};
pub use football::FootballLeague;
pub use weather::WeatherLocation;
