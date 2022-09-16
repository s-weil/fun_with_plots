use std::collections::HashMap;

use crate::data::models::{TimeSeries, TimeSeriesPoint};
use plotly::common::{DashType, Line, Marker, Mode, Title};
use plotly::layout::{
    Axis, Layout, Legend, RangeSelector, RangeSlider, SelectorButton, SelectorStep, StepMode,
    TicksDirection,
};
use plotly::{Plot, Rgb, Scatter};

fn unzip(time_series: &[TimeSeriesPoint]) -> (Vec<String>, Vec<f32>) {
    let mut dates = Vec::new();
    let mut values = Vec::new();
    // skip the first value which is the forecast of today
    for tsp in time_series[1..].iter() {
        dates.push(tsp.date.to_string());
        values.push(tsp.value);
    }
    (dates, values)
}

pub fn plot_time_series<T: std::fmt::Display>(
    reference: &[TimeSeriesPoint],
    timeseries_collection: &[(T, TimeSeries)],
) {
    let mut plot = Plot::new();

    let (dates, values) = unzip(reference);
    let trace = Scatter::new(dates, values).name("reference (forecast today)");
    plot.add_trace(trace);

    for (name, ts) in timeseries_collection {
        let (dates, values) = unzip(ts);
        let trace = Scatter::new(dates, values)
            .name(&name.to_string())
            .line(Line::new().dash(DashType::Dot));
        plot.add_trace(trace);
    }

    let layout = Layout::new()
        .title(Title::new(
            "Weather forcast curves - 16days, Celsius: max-temperature for Zurich, CH",
        ))
        .legend(Legend::new().title(Title::new("Forecast curve as of date")))
        .paper_background_color(Rgb::new(255, 255, 255))
        .plot_background_color(Rgb::new(229, 229, 229))
        .x_axis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .range(vec![1.0, 10.0])
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false)
                .range_slider(RangeSlider::new().visible(true))
                .range_selector(RangeSelector::new().buttons(vec![
                        SelectorButton::new()
                            .count(1)
                            .label("1d")
                            .step(SelectorStep::Day)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(1)
                            .label("1m")
                            .step(SelectorStep::Month)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(1)
                            .label("YTD")
                            .step(SelectorStep::Year)
                            .step_mode(StepMode::ToDate),
                        SelectorButton::new()
                            .count(1)
                            .label("1y")
                            .step(SelectorStep::Year)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new().step(SelectorStep::All),
                    ])),
        )
        .y_axis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        );
    plot.set_layout(layout);

    plot.show();
    plot.to_inline_html(Some("time_series_with_range_selector_buttons"));
}

fn unzip_level_curve(curve: &[(chrono::Duration, f32)]) -> (Vec<f32>, Vec<f32>) {
    let mut days_ahead = Vec::new();
    let mut values = Vec::new();
    // skip the first value which is the forecast of today
    for (duration, v) in curve.iter() {
        days_ahead.push(duration.num_days() as f32);
        values.push(*v);
    }
    (days_ahead, values)
}

pub fn plot_level_curves(curve_by_level: &HashMap<usize, Vec<(chrono::Duration, f32)>>) {
    let layout = Layout::new().title(Title::new("Percentile Level curves"));
    let mut plot = Plot::new();

    for (level, level_curve) in curve_by_level.iter() {
        let (days_ahead, vs) = unzip_level_curve(level_curve);
        let trace = Scatter::new(days_ahead, vs)
            .mode(Mode::LinesMarkers)
            .name(&format!("Level {}%", level))
            .marker(
                Marker::new()
                    .color(Rgb::new(3 * (*level as u8), 64, 82))
                    .size(12),
            );
        plot.add_trace(trace);
    }

    plot.set_layout(layout);
    plot.show();
    println!("{}", plot.to_inline_html(Some("line_and_scatter_styling")));
}

pub fn plot_metric_curves(base_cuve: &Vec<f32>, metric_curves: &HashMap<&str, Vec<f32>>) {
    let layout = Layout::new().title(Title::new("Metric curves"));
    let mut plot = Plot::new();

    for (metric_name, metric_curve) in metric_curves.iter() {
        let trace = Scatter::new(base_cuve.clone(), metric_curve.clone())
            .mode(Mode::LinesMarkers)
            .name(metric_name);

        plot.add_trace(trace);
    }

    plot.set_layout(layout);
    plot.show();
    println!("{}", plot.to_inline_html(Some("line_and_scatter_styling")));
}
