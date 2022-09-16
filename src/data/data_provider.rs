use crate::data::football::models::FootballSeasonResults;
use crate::data::weather::models::WeatherResponse;
use crate::data::FootballLeague;
use crate::data::WeatherLocation;
use crate::errors::AppError;
use chrono::Date;
use chrono::NaiveDate;
use chrono::Utc;
use std::fs::create_dir;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;

type ApiKey = String;

const DATA_PATH: &str = "data";
const WEATHER: &str = "weather";
const FOOTBALL: &str = "football";

pub trait TimeSeriesSortKey {
    fn date(&self) -> NaiveDate;
}

impl TimeSeriesSortKey for WeatherResponse {
    fn date(&self) -> NaiveDate {
        self.date().naive_utc()
    }
}

impl TimeSeriesSortKey for FootballSeasonResults {
    fn date(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.season, 1, 1)
    }
}

pub trait DataProvider {
    fn data_path(&self) -> PathBuf;

    fn load_timeseries_data<T: serde::de::DeserializeOwned + TimeSeriesSortKey>(
        &self,
    ) -> Result<Vec<T>, AppError> {
        let data_path = self.data_path();
        let mut data: Vec<T> = utils::read_files(&data_path)?;
        data.sort_by_key(|d| d.date());
        Ok(data)
    }
}

pub struct WeatherProvider {
    api_key: ApiKey,
    location: WeatherLocation,
}

impl DataProvider for WeatherProvider {
    fn data_path(&self) -> PathBuf {
        Path::new(DATA_PATH)
            .join(WEATHER)
            .join(&self.location.to_string())
    }
}

impl WeatherProvider {
    pub fn new(api_key: ApiKey, location: WeatherLocation) -> Self {
        Self { api_key, location }
    }

    pub fn update_data(&self, as_of: &Date<Utc>) -> Result<(), AppError> {
        let zip_country_path = self.data_path();

        if !zip_country_path.exists() {
            create_dir("/some/dir")?;
        }

        let file_name = format!("{}.json", as_of);
        let file = Path::new(&file_name);
        let file_path = zip_country_path.join(file);

        // update the data
        if !Path::new(&file_path).exists() {
            println!("Requesting weather forecast");
            let weather_forecast = self.location.load_weather_data(&self.api_key)?;

            println!("Saving Weather forecast to {:?}", &file_path);
            utils::save_file(&file_path, weather_forecast)?;

            println!("Successfully saved weather forecast");
        }

        Ok(())
    }
}

pub struct FootballProvider {
    api_key: ApiKey, // TODO: load the data
    league: FootballLeague,
}

impl FootballProvider {
    pub fn new(api_key: ApiKey, league: FootballLeague) -> Self {
        Self { api_key, league }
    }
}

impl DataProvider for FootballProvider {
    fn data_path(&self) -> PathBuf {
        Path::new(DATA_PATH)
            .join(FOOTBALL)
            .join(&self.league.to_string())
    }
}

mod utils {
    use log::{error, trace};
    use std::fs::{read_dir, read_to_string, File};
    use std::io::Write;
    use std::path::Path;
    use std::path::PathBuf;
    use std::result::Result;

    use crate::errors::AppError;

    /// Save the serialized content to the specified file.
    pub fn save_file<T: serde::Serialize + std::fmt::Debug>(
        file_path: &Path,
        content: T,
    ) -> Result<(), AppError> {
        let mut output = File::create(file_path)?;
        let serialized: String = serde_json::to_string(&content)?;
        write!(output, "{}", serialized)?;
        Ok(())
    }

    fn read_file<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T, AppError> {
        let content = read_to_string(&path)?;
        let t: T = serde_json::from_str(&content).map_err(|e| {
            error!("cannot deserialize file '{:?}'", path);
            e
        })?;
        Ok(t)
    }

    pub fn read_files<T: serde::de::DeserializeOwned>(dir: &Path) -> Result<Vec<T>, AppError> {
        let mut files: Vec<T> = Vec::new();
        for entry in read_dir(dir)? {
            let file = entry?;
            let path = file.path();
            trace!("reading file '{:?}", path);
            let t: T = read_file(&path)?;
            files.push(t);
        }

        Ok(files)
    }
}
