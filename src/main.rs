extern crate chrono;
extern crate clap;
extern crate plotly;
extern crate serde;

mod data;
mod errors;
mod plot;

use crate::data::football::{
    column_vec, convert_data_frame, filter_players, join_players, FootballSeasonResults,
};
use crate::data::models::{TimeSeries, TimeSeriesPoint};
use crate::data::weather::{
    convert_weather_responses, create_temperature_timeseries, models::WeatherResponse,
    reference_timeseries,
};
use crate::data::{
    DataProvider, FootballLeague, FootballProvider, WeatherLocation, WeatherProvider,
};
use crate::plot::AnimationType;
use crate::plot::Plot;
use chrono::Date;
use chrono::Utc;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use env_logger::Env;
use errors::AppError;
use log::info;
use std::collections::HashMap;
use std::result::Result;
use std::sync::Arc;
use std::thread;

// TODO / ideas:
// - add comments
// - migrate to public "fun with plots / graphs"
// - add Readme with gifs and chart
// - add more graphs like pressure
// - add corona data / different data source
// - add (mongo) DB if data gets bigger?
// - github actions

const ENV_API_KEY: &str = "API_KEY";
const LOG_LEVEL: &str = "LOG_LEVEL";

// TODO: structure into weather and football
#[derive(serde::Deserialize, Debug)]
struct Settings {
    pub country_code: String,
    pub zip: String,
    pub football_country: String,
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

#[derive(Subcommand, Debug)]
enum JobArgument {
    Weather,
    Football,
}

/// CLI to run the different data jobs.
/// Examples:
/// - cargo r weather: load latest data, plot graphs and animations
/// - cargo r football: load data, plot graphs
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    #[clap(subcommand)]
    data_source: JobArgument,
}

fn run_football(settings: &Settings) -> Result<(), AppError> {
    let league = FootballLeague {
        country: settings.football_country.to_lowercase(),
    };
    let data_provider = FootballProvider::new("NOT_SET".to_string(), league);
    let results_by_season: Vec<FootballSeasonResults> = data_provider.load_timeseries_data()?;
    let season_resuls_df = convert_data_frame(results_by_season);

    // # SPANISH ligue: id 140
    // # L.Messi: id 154
    // # C.Ronaldo: id 874
    let messi = filter_players(&season_resuls_df, 154);
    dbg!(&messi);
    let ronaldo = filter_players(&season_resuls_df, 874);
    dbg!(&ronaldo);

    let common_stats = join_players(messi, &ronaldo);
    dbg!(&common_stats);

    fn filter_map(v: Vec<Option<f32>>, factor: f32) -> Vec<f32> {
        v.into_iter()
            .map(|x| x.map(|z| z * factor).unwrap_or_else(|| 0.0))
            .collect()
    }

    //TODO: should rather have curvees like timeseries each with season and values
    let mut metrics = HashMap::new();
    metrics.insert(
        "Messi.GPM",
        filter_map(column_vec(&common_stats, "goals_per_minute"), 100.0),
    );
    metrics.insert(
        "Ronaldo.GPM",
        filter_map(column_vec(&common_stats, "goals_per_minute.other"), 100.0),
    );
    metrics.insert(
        "Messi.PPM",
        filter_map(column_vec(&common_stats, "passes_per_minute"), 1.0),
    );
    metrics.insert(
        "Ronaldo.PPM",
        filter_map(column_vec(&common_stats, "passes_per_minute.other"), 1.0),
    );

    let seasons = filter_map(column_vec(&common_stats, "season"), 1.0);

    crate::plot::plot_metric_curves(&seasons, &metrics);

    Ok(())
}

fn run_weather(settings: &Settings) -> Result<(), AppError> {
    let api_key = std::env::var(ENV_API_KEY)?;

    let weather_location =
        WeatherLocation::new(settings.country_code.clone(), settings.zip.clone());
    let data_provider = WeatherProvider::new(api_key, weather_location);

    info!("Check updates for forcast data");
    let today = Utc::now().date();
    data_provider.update_data(&today)?;

    // forecast curves (retrieved as by date) each containing full weather forecast data
    let forecasts: Vec<WeatherResponse> = data_provider.load_timeseries_data()?;
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

    Ok(())
}

fn main() -> Result<(), AppError> {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let args = CliArgs::parse();

    let log_level = std::env::var(LOG_LEVEL)?;
    env_logger::Builder::from_env(Env::default().default_filter_or(&log_level)).init();

    info!("Starting fun with plots");
    let settings = Settings::init()?;

    match args.data_source {
        JobArgument::Football => run_football(&settings)?,
        JobArgument::Weather => run_weather(&settings)?,
    };

    info!("Completed");
    Ok(())
}
