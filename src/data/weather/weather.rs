use crate::data::weather::models::WeatherResponse;
use chrono::Utc;
use serde_json::Value;
use std::error::Error as std_error;
use std::result::Result;

pub struct WeatherLocation {
    pub country_code: String,
    pub zip: String,
}

impl std::fmt::Display for WeatherLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", self.country_code, self.zip)
    }
}

impl WeatherLocation {
    pub fn new(country_code: String, zip: String) -> Self {
        Self { country_code, zip }
    }

    /// Loads the most recent weather forecast from the API
    pub fn load_weather_data(&self, api_key: &str) -> Result<WeatherResponse, Box<dyn std_error>> {
        let url = format!(
            "https://api.weatherbit.io/v2.0/forecast/daily?postal_code={}&country={}&key={}",
            self.zip, self.country_code, api_key
        );
        let resp_json = reqwest::blocking::get(url)?.json::<Value>()?;

        // NOTE: we dump the whole response data for now, so that we can analyze further graphs later
        let response = WeatherResponse {
            as_of_date: Utc::now().date().to_string(),
            forecast: (&resp_json["data"]).clone(),
        };

        Ok(response)
    }
}
