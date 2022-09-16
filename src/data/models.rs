use super::weather::models::ForecastTemperaturePoint;
use chrono::Date;
use chrono::Utc;

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
