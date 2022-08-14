use crate::model::TimeSeriesPoint;
use chrono::{Date, Utc};
use plotters::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub enum AnimationType {
    Absolute,
    Relative(HashMap<Date<Utc>, f32>), // containing the reference curve
}

impl AnimationType {
    pub fn create_relative(ref_curve: &[TimeSeriesPoint]) -> AnimationType {
        let ref_by_date: HashMap<Date<Utc>, f32> =
            ref_curve.iter().map(|tsp| (tsp.date, tsp.value)).collect();
        AnimationType::Relative(ref_by_date)
    }

    fn output_file_name(&self, base_dir: &Path) -> PathBuf {
        match self {
            AnimationType::Absolute => base_dir.join("forecast_animation.gif"),
            AnimationType::Relative(_) => base_dir.join("forecast_relative_animation.gif"),
        }
    }

    fn y_axis_range(&self) -> std::ops::Range<f32> {
        match self {
            AnimationType::Absolute => -20.0..45.0,
            AnimationType::Relative(_) => -20.0..20.0,
        }
    }

    fn caption(&self) -> String {
        match self {
            AnimationType::Absolute =>  "Weather forcast curve for next 16 days in Celsius: max-temperature for Zurich, CH'".to_string(),
            AnimationType::Relative(_) =>  "Weather forcast difference (relative to reference) curve for next 16 days in Celsius: max-temperature for Zurich, CH'".to_string(),

        }
    }

    fn chart_point(&self, time_series_point: &TimeSeriesPoint) -> Option<f32> {
        match self {
            AnimationType::Absolute => Some(time_series_point.value),
            AnimationType::Relative(ref_by_date) => ref_by_date
                .get(&time_series_point.date)
                .map(|ref_v| time_series_point.value - ref_v),
        }
    }
}

pub fn plot_time_series_animation(
    animation_type: AnimationType,
    forecast_timeseries: &[(Date<Utc>, Vec<TimeSeriesPoint>)],
) -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new("plots").join("CH__8001"); // TODO
    let output_path = animation_type.output_file_name(&base_dir);

    let delay = 1_000;
    let root = BitMapBackend::gif(&output_path, (800, 600), delay)?.into_drawing_area();

    let caption = animation_type.caption();
    let y_axis_range = animation_type.y_axis_range();

    for (as_of_date, ts) in forecast_timeseries {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(caption.clone(), ("sans-serif", 15))
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .build_cartesian_2d::<std::ops::Range<f32>, std::ops::Range<f32>>(
                0.0..17.0,
                y_axis_range.clone(),
            )?;

        chart
            .configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()?;

        let points: Vec<(f32, f32)> = ts
            .iter()
            .enumerate()
            .flat_map(|(idx, tsp)| {
                animation_type
                    .chart_point(tsp)
                    .map(|v| (idx as f32, v as f32))
            })
            .collect();
        // TODO: in absolute case, keep always the previous one and add new one in new color (but only show 2 at the time)

        let series = LineSeries::new(points, &RED);

        chart
            .draw_series(series)?
            .label(&as_of_date.to_string().replace("UTC", ""))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
    }

    println!("Result has been saved to {:?}", output_path);

    Ok(())
}
