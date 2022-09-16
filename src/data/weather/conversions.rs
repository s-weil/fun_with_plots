use crate::data::models::{TimeSeries, TimeSeriesPoint};
use crate::data::weather::models::{ForecastCurve, WeatherResponse};
use chrono::Utc;
use chrono::{Date, Duration};
use std::collections::HashMap;

// TODO: refactor to make it more clear what and where the conversions happen
pub fn convert_weather_responses<T: serde::de::DeserializeOwned>(
    forecasts: Vec<WeatherResponse>,
    create_time_series: impl Fn(T) -> TimeSeries,
) -> Vec<(Date<Utc>, TimeSeries)> {
    forecasts
        .into_iter()
        .flat_map(|fc| {
            let date = fc.date();
            serde_json::from_value(fc.forecast)
                .map(|forecast_curve: T| (date, create_time_series(forecast_curve)))
        })
        .collect()
}

pub fn create_temperature_timeseries(forecast_curve: ForecastCurve) -> TimeSeries {
    forecast_curve.iter().map(|fc_pt| fc_pt.into()).collect()
}

pub fn reference_timeseries(forecast_ts: &[(Date<Utc>, TimeSeries)]) -> Vec<TimeSeriesPoint> {
    forecast_ts.iter().map(|(_, ts)| ts[0].clone()).collect()
}

pub fn percentile_timeseries(forecast_ts: &[(Date<Utc>, TimeSeries)]) -> Vec<(usize, TimeSeries)> {
    let mut grouped_by_date: HashMap<Date<Utc>, Vec<f32>> = HashMap::new();

    for (_, ts) in forecast_ts.iter() {
        for tsp in ts.iter() {
            let vs = grouped_by_date.entry(tsp.date).or_insert(Vec::new());
            vs.push(tsp.value);
        }
    }

    grouped_by_date
        .iter_mut()
        .for_each(|(_, vs)| vs.sort_by(|a, b| a.partial_cmp(b).unwrap()));

    let max_idx = 5;
    let mut level_map = Vec::with_capacity(4);

    for level in (20..=80).step_by(20) {
        let mut level_curve = Vec::new();
        for (&date, vs) in grouped_by_date.iter() {
            if vs.len() >= max_idx {
                let idx = ((vs.len() as f32) * (level as f32 / 100.0)).floor() as usize;
                level_curve.push(TimeSeriesPoint {
                    date,
                    value: vs[idx],
                });
            }
        }

        level_curve.sort_by_key(|d| d.date);
        level_map.push((level, level_curve));
    }

    level_map
}

pub fn percentiles(
    ref_curve: &[TimeSeriesPoint],
    forecast_ts: &[(Date<Utc>, TimeSeries)],
) -> HashMap<usize, Vec<(Duration, f32)>> {
    let ref_by_date: HashMap<Date<Utc>, f32> =
        ref_curve.iter().map(|tsp| (tsp.date, tsp.value)).collect();

    let mut grouped_by_d: HashMap<Duration, Vec<f32>> = HashMap::new();

    for (as_of_date, ts) in forecast_ts.iter() {
        for tsp in ts.iter() {
            if let Some(ref_v) = ref_by_date.get(&tsp.date) {
                let days_ahead = tsp.date - *as_of_date;
                let vs = grouped_by_d.entry(days_ahead).or_insert(Vec::new());
                vs.push(tsp.value - ref_v);
            }
        }
    }

    grouped_by_d
        .iter_mut()
        .for_each(|(_, vs)| vs.sort_by(|a, b| a.partial_cmp(b).unwrap()));

    // TODO: do only the levels 20,40,60,80 for now until more data is available
    let max_idx = 5;
    let mut level_map = HashMap::new();

    for level in (20..=80).step_by(20) {
        let mut level_curve = Vec::new();
        for (duration, vs) in grouped_by_d.iter() {
            if vs.len() >= max_idx {
                let idx = ((vs.len() as f32) * (level as f32 / 100.0)).floor() as usize;
                level_curve.push((*duration, vs[idx]));
            }
        }

        level_curve.sort_by(|(d1, _), (d2, _)| d1.cmp(d2));
        level_map.insert(level, level_curve);
    }

    level_map
}
