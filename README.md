# FUN WITH PLOTS

This is <b>fun with plots</b> where we experiment with data and visualizations in rust.
For example the 

- weather data, a popular data source for forecasts and which are hard to predict and deviate the more days ahead of the reference date
- football metrics which compares stats and metrics of several players

Below are visualizations of weather data

<img src="./plots/CH__8001/forecast_relative_animation.gif" width="500" height="500" />
<img src="./plots/CH__8001/forecast_animation.gif" width="500" height="500" />

![Forecasts absolute](./plots/CH__8001/daily_forecast_curves.png)
![Forecasts absolute](./plots/CH__8001/relative_percentile_curves.png)
![Forecasts absolute](./plots/CH__8001/daily_percentile_curves.png)


Below are visualizations of the football data 

Ronaldo<br>
![Forecasts absolute](./plots/spain/ronaldo.png)<br>
Messi<br>
![Forecasts absolute](./plots/spain/messi.png)<br>
Ronaldo and Messi per season (GPM - goals per minute,, scaled by 100.0; PPM - passes per minute)<br>
![Forecasts absolute](./plots/spain/Ronaldo_vs_Messi.png)

## Disclaimer

This is a <i>fun</i> project with many open todo's and possible improvements, i.e. due to lack of time many parts are simple and not meant for any productive setup.

The data is licensed to weatherbit.io resp. football.api-sports.io and is hence not provided here, i.e. not under source control.
For the former, you need to run the app on a daily basis for several days in order to gather the data required for creating plots similar to the ones above.

## Setup

Rename the `.env_exampe` file to `.env` and paste in your API Key from weatherbit.io resp. football.api-sports.io
Also adjust the `country` and `zip` code in the `config.toml` file for the weather data and `football_country` for the football data.

Run 
```
cargo r weather
cargo r football
```

### TODOs and ideas

    - comments
    - add more weather graphs like pressure and more football metrics and players
    - further data sources: add corona, census or stock data
    - add (mongo) DB for data management
