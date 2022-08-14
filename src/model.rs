use chrono::Date;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

// Cannot impl the From trait due to orphan rule..
fn from(date: &str) -> Date<Utc> {
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let naive_datetime: NaiveDateTime = naive_date.and_hms(0, 0, 0);
    let datetime_utc = DateTime::<Utc>::from_utc(naive_datetime, Utc);
    datetime_utc.date()
}

/*
  "moonrise_ts": 1658709007,
  "wind_cdir": "SW",
  "rh": 60,
  "pres": 943.8,
  "high_temp": 33.6,
  "sunset_ts": 1658776108,
  "ozone": 302.8,
  "moon_phase": 0.0366486,
  "wind_gust_spd": 7.9,
  "snow_depth": 0,
  "clouds": 18,
  "ts": 1658700060,
  "sunrise_ts": 1658721283,
  "app_min_temp": 19.7,
  "wind_spd": 2.6,
  "pop": 10,
  "wind_cdir_full": "southwest",
  "slp": 1014,
  "moon_phase_lunation": 0.91,
  "valid_date": "2022-07-25",
  "app_max_temp": 32.2,
  "vis": 24.127,
  "dewpt": 15.8,
  "snow": 0,
  "uv": 8.5,
  "weather": {
    "icon": "c02d",
    "code": 801,
    "description": "Few clouds"
  },
  "wind_dir": 224,
  "max_dhi": null,
  "clouds_hi": 26,
  "precip": 0.419922,
  "low_temp": 18.8,
  "max_temp": 33.6,
  "moonset_ts": 1658772960,
  "datetime": "2022-07-25",
  "temp": 25.6,
  "min_temp": 19.4,
  "clouds_mid": 21,
  "clouds_low": 7
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherResponse {
    #[serde(rename = "asOfDate")]
    pub as_of_date: String,
    pub forecast: Value, // NOTE: store the full response data for further future plots
}

impl WeatherResponse {
    pub fn date(&self) -> chrono::Date<Utc> {
        from(&self.as_of_date.replace("UTC", ""))
    }
}

// Required for deserializing
pub type ForecastCurve = Vec<ForecastTemperaturePoint>;

#[derive(Debug, Deserialize)]
pub struct ForecastTemperaturePoint {
    pub valid_date: String,
    pub max_temp: f32,
}

impl ForecastTemperaturePoint {
    pub fn date(&self) -> Date<Utc> {
        from(&self.valid_date)
    }
}

pub type TimeSeries = Vec<TimeSeriesPoint>;

#[derive(Debug, Clone)]
pub struct TimeSeriesPoint {
    pub date: Date<Utc>,
    pub value: f32,
}

impl From<&ForecastTemperaturePoint> for TimeSeriesPoint {
    fn from(fc_pt: &ForecastTemperaturePoint) -> TimeSeriesPoint {
        Self {
            date: fc_pt.date(),
            value: fc_pt.max_temp,
        }
    }
}

// pub struct TimeSeriesRef<T> {

// }
