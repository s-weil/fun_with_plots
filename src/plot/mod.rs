mod animation;
mod chart;
pub use animation::AnimationType;

use crate::errors::AppError;
use crate::model::TimeSeriesPoint;
use chrono::Date;
use chrono::Utc;

pub enum Plot<'a> {
    Chart(&'a [TimeSeriesPoint]),
    ChartLevels(&'a [TimeSeriesPoint]),  // TODO: unionize
    ChartLevelTs(&'a [TimeSeriesPoint]), // TODO: unionize
    Animation(AnimationType),
}

impl<'a> Plot<'a> {
    pub fn plot(
        self,
        forecast_timeseries: &[(Date<Utc>, Vec<TimeSeriesPoint>)],
    ) -> Result<(), AppError> {
        match self {
            Plot::Chart(ref_ts) => chart::plot_time_series(ref_ts, forecast_timeseries),
            Plot::ChartLevelTs(ref_ts) => {
                let percentile_timeseries = crate::data::percentile_timeseries(forecast_timeseries);
                chart::plot_time_series(ref_ts, &percentile_timeseries)
            }
            Plot::ChartLevels(ref_ts) => {
                let percentiles = crate::data::percentiles(ref_ts, forecast_timeseries);
                chart::plot_level_curves(&percentiles);
            }
            Plot::Animation(animation_type) => {
                animation::plot_time_series_animation(animation_type, forecast_timeseries)?
            }
        }
        Ok(())
    }
}
