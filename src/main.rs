extern crate chrono;
extern crate plotly;
extern crate serde;

mod data;
mod errors;
mod model;
mod plot;

use crate::data::convert_weather_responses;
use crate::data::create_temperature_timeseries;
use crate::data::reference_timeseries;
use crate::data::DataProvider;
use crate::data::WeatherLocation;
use crate::model::TimeSeries;
use crate::plot::AnimationType;
use chrono::Date;
use chrono::Utc;
use dotenv::dotenv;
use errors::AppError;
use log::info;
use model::{TimeSeriesPoint, WeatherResponse};
use plot::Plot;
use std::result::Result;
use std::sync::Arc;
use std::thread;

const ENV_API_KEY: &str = "API_KEY";

#[derive(serde::Deserialize, Debug)]
struct Settings {
    pub country_code: String,
    pub zip: String,
}

impl Settings {
    pub fn init() -> Result<Self, AppError> {
        let config = config::Config::builder()
            .add_source(config::File::from(std::path::Path::new("config.toml")))
            .build()?;
        let settings = config.try_deserialize::<Settings>()?;
        Ok(settings)
    }
}

fn main() -> Result<(), AppError> {
    env_logger::init();

    info!("Starting fun with plots");
    let settings = Settings::init()?;

    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    let api_key = std::env::var(ENV_API_KEY)?;

    let weather_location = WeatherLocation::new(settings.country_code, settings.zip);
    let data_provider = DataProvider::Weather((api_key, weather_location));

    info!("Check updates for forcast data");
    let today = Utc::now().date();
    data_provider.update_data(&today)?;

    // forecast curves (retrieved as by date) each containing full weather forecast data
    let forecasts: Vec<WeatherResponse> = data_provider.load_data()?;
    info!("loaded {} weather forecasts", forecasts.len());

    // use max-temperature in the following
    let temperature_forecasts: Vec<(Date<Utc>, TimeSeries)> =
        convert_weather_responses(forecasts, create_temperature_timeseries);

    let reference_ts: Vec<TimeSeriesPoint> = reference_timeseries(&temperature_forecasts);

    info!("Creating plots");
    Plot::Chart(&reference_ts).plot(&temperature_forecasts)?;
    Plot::ChartLevels(&reference_ts).plot(&temperature_forecasts)?;
    Plot::ChartLevelTs(&reference_ts).plot(&temperature_forecasts)?;

    info!("Creating animations");
    // Parallelize the animations as each takes a considerate time
    let forecast_ts_arc = Arc::new(temperature_forecasts);

    let abs_fc_ts = forecast_ts_arc.clone();
    let abs_handle = thread::spawn(move || {
        Plot::Animation(AnimationType::Absolute)
            .plot(abs_fc_ts.as_ref())
            .unwrap();
    });

    let rel_handle = thread::spawn(move || {
        Plot::Animation(AnimationType::create_relative(&reference_ts))
            .plot(&forecast_ts_arc)
            .unwrap();
    });

    abs_handle.join().expect("Absolute animation failed");
    rel_handle.join().expect("Relative animation failed");

    info!("Completed");
    Ok(())
}
