use crate::errors::AppError;
use crate::model::WeatherResponse;

use super::WeatherLocation;
use chrono::Date;
use chrono::Utc;
use std::fs::create_dir;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;

type ApiKey = String;

const DATA_PATH: &str = "data";
const WEATHER: &str = "weather";

pub trait TimeSeriesSortKey {
    fn date(&self) -> Date<Utc>;
}

impl TimeSeriesSortKey for WeatherResponse {
    fn date(&self) -> Date<Utc> {
        self.date()
    }
}

pub enum DataProvider {
    Weather((ApiKey, WeatherLocation)),
    // further to come... possibly model with traits
}

impl DataProvider {
    fn data_path(&self) -> PathBuf {
        match self {
            DataProvider::Weather((_, weather_location)) => Path::new(DATA_PATH)
                .join(WEATHER)
                .join(&weather_location.to_string()),
        }
    }

    pub fn update_data(&self, today: &Date<Utc>) -> Result<(), AppError> {
        match self {
            DataProvider::Weather((api_key, weather_geo_spec)) => {
                let zip_country_path = self.data_path();

                if !zip_country_path.exists() {
                    create_dir("/some/dir")?;
                }

                let file_name = format!("{}.json", today);
                let file = Path::new(&file_name);
                let file_path = zip_country_path.join(file);

                // update the data
                if !Path::new(&file_path).exists() {
                    println!("Requesting weather forecast");
                    let weather_forecast = weather_geo_spec.load_weather_data(api_key)?;

                    println!("Saving Weather forecast to {:?}", &file_path);
                    utils::save_file(&file_path, weather_forecast)?;

                    println!("Successfully saved weather forecast");
                }
            }
        }

        Ok(())
    }

    pub fn load_data<T: serde::de::DeserializeOwned + TimeSeriesSortKey>(
        &self,
    ) -> Result<Vec<T>, AppError> {
        match self {
            DataProvider::Weather(_) => {
                let zip_country_path = self.data_path();
                let mut data: Vec<T> = utils::read_files(&zip_country_path)?;
                data.sort_by_key(|d| d.date());
                Ok(data)
            }
        }
    }
}

mod utils {
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
        let t: T = serde_json::from_str(&content)?;
        Ok(t)
    }

    pub fn read_files<T: serde::de::DeserializeOwned>(dir: &Path) -> Result<Vec<T>, AppError> {
        let mut files: Vec<T> = Vec::new();
        for entry in read_dir(dir)? {
            let file = entry?;
            let path = file.path();
            let t: T = read_file(&path)?;
            files.push(t);
        }

        Ok(files)
    }
}
