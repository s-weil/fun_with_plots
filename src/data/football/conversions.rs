use crate::data::football::models::FootballSeasonResults;
use polars::df;
use polars::prelude::*;

pub fn column_vec(df: &polars::prelude::DataFrame, col_name: &str) -> Vec<Option<f32>> {
    let chunked_arr = df.column(col_name).unwrap().f32().unwrap();
    Vec::from(chunked_arr)
}

pub fn join_players(
    df1: polars::prelude::DataFrame,
    df2: &polars::prelude::DataFrame,
) -> polars::prelude::DataFrame {
    df1.join(
        df2,
        ["season"],
        ["season"],
        JoinType::Inner,
        Some(".other".to_string()),
    )
    .unwrap()
}

// TODO: there must be a nicer way!
pub fn filter_players(
    df: &polars::prelude::DataFrame,
    player_id: i32,
) -> polars::prelude::DataFrame {
    let filtered_ids: Vec<i32> = df
        .column("player_id")
        .unwrap()
        .i32()
        .unwrap()
        .into_iter()
        .map(|id| match id {
            Some(i) if i == player_id => player_id,
            _ => -1,
        })
        .collect();

    let s0 = Series::new("player_id", &filtered_ids);

    df.filter(&df.column("player_id").unwrap().equal(&s0).unwrap())
        .unwrap()
}

// TODO: add more metrics and stats
pub fn convert_data_frame(
    season_results: Vec<FootballSeasonResults>,
) -> polars::prelude::DataFrame {
    let mut player_ids = Vec::new();
    let mut season = Vec::new();
    let mut goals = Vec::new();
    let mut minutes = Vec::new();
    let mut cards_weighted = Vec::new();
    let mut passes_total = Vec::new();
    let mut goals_per_minute: Vec<f32> = Vec::new();
    let mut fairness_per_minute: Vec<f32> = Vec::new();
    let mut passes_per_minute: Vec<f32> = Vec::new();

    let add_per_minute = |(container, v, nr_mins): (&mut Vec<f32>, i32, i32)| {
        container.push(if nr_mins == 0 {
            0.0
        } else {
            v as f32 / nr_mins as f32
        })
    };

    for season_result in season_results {
        for player_results in season_result.player_results {
            player_ids.push(player_results.player.id);
            player_results.statistics.first().map(|res| {
                let gs = res
                    .goals
                    .as_ref()
                    .and_then(|r| r.total)
                    .unwrap_or_else(|| 0);

                let mins = res
                    .games
                    .as_ref()
                    .and_then(|r| r.minutes)
                    .unwrap_or_else(|| 0);

                let cards_total_weighted = res
                    .cards
                    .as_ref()
                    .map(|cards| {
                        cards.red.unwrap_or(0) * 3
                            + cards.yellowred.unwrap_or(0) * 2
                            + cards.yellow.unwrap_or(0)
                    })
                    .unwrap_or_else(|| 0);

                let passes = res
                    .passes
                    .as_ref()
                    .and_then(|p| p.total)
                    .unwrap_or_else(|| 0);

                minutes.push(mins);
                goals.push(gs);
                passes_total.push(passes);
                cards_weighted.push(cards_total_weighted);

                add_per_minute((&mut goals_per_minute, gs, mins));
                add_per_minute((&mut fairness_per_minute, cards_total_weighted, mins));
                add_per_minute((&mut passes_per_minute, passes, mins));
            });
            season.push(season_result.season as f32);
        }
    }

    let data_frame = df![
        "season" => &season,
        "player_id" => &player_ids,
        "goals" => &goals,
        "minutes" => &minutes,
        "passes" => &passes_total,
        "cards_weighted" => &cards_weighted,
        "goals_per_minute" => &goals_per_minute,
        "fairness_per_minute" => &fairness_per_minute,
        "passes_per_minute" => &passes_per_minute
    ];
    // let season_series = Series::new("season", &season);
    // let player_ids_series = Series::new("player_ids", &player_ids);
    // let df = DataFrame::new(vec![season_series, player_ids_series]);
    data_frame.unwrap()
}
