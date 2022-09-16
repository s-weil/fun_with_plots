import pandas as pd
import json
import requests
import os

headers = {
    'x-rapidapi-host': "v3.football.api-sports.io",
    'x-rapidapi-key': "XXX"
}

def json_request(route):
    resp = requests.get(f"https://v3.football.api-sports.io/{route}", headers=headers)
    print(resp.status_code)    
    return resp.json()

def dump_json(json_content, file_name):
    json_string = json.dumps(json_content)
    dir = os.path.dirname(__file__)
    file = os.path.join(dir, f'{file_name}.json')
    with open(file, 'w') as outfile:
        outfile.write(json_string)

def read_json_source(file_name):
    dir = os.path.dirname(__file__)
    file = os.path.join(dir, f'{file_name}.json')
    with open(file) as json_file:
        data = json.load(json_file)
        return data



# countries = read_json_source("json/countries")
# countries = json_request("/countries")
# dump_json(countries['response'], "json/countries")
# print(countries)

# leagues = read_json_source("json/leagues")
# leagues = json_request("/leagues")
# dump_json(leagues['response'], "json/leagues")
# print(leagues)
# leagues_df = pd.read_json(leagues)

# SPANISH ligue: id 140
# L.Messi: id 154
# C.Ronaldo: id 874

# top_scorers_spain_2018 = json_request("/players/topscorers?season=2018&league=140")
# dump_json(top_scorers_spain_2018['response'], "json/top_scorers_spain_2018")
# top_scorers_spain_2018 = read_json_source("json/top_scorers_spain_2018")
# print(top_scorers_spain_2018)

# top_scorers_spain = json_request("/players/topscorers?season=2020&league=140")
# dump_json(top_scorers_spain['response'], "json/top_scorers_spain_2020")

# for season_year in range(2000, 2021):
#     top_scorers_spain = json_request(f"/players/topscorers?season={season_year}&league=140")
#     dump_json(top_scorers_spain['response'], f"json/top_scorers_spain_{season_year}")

# season_dict = {}
# for season_year in range(2010, 2021):
#     season_results = read_json_source(f"json/top_scorers_spain_{season_year}")
#     df = pd.json_normalize(season_results, record_path=['statistics'], meta=[['player', 'id'], ['player', 'name']]) #meta=['player', 'statistics']
#     df['season'] = 2015
#     season_dict[season_year] = df

season_list = []
for season_year in range(2010, 2021):
    season_results = read_json_source(f"json/top_scorers_spain_{season_year}")
    df = pd.json_normalize(season_results, record_path=['statistics'], meta=[['player', 'id'], ['player', 'name']]) #meta: "['player', 'id'] <> player -> id"
    df['season'] = season_year
    season_list.append(df)

# df_seasons = pd.concat(season_list, keys=['season'])
df_seasons = pd.concat(season_list, axis=0)
print(df_seasons.head())
# print(df_seasons.columns())

# ronaldo = df_seasons.query('"player.id"==874')
# ronaldo = df_seasons[df_seasons['player.id'] == 874]

# id -> birth year
birthyear_by_playerid = { 874: 1985, 154 : 1987 }

player_ids = [874, 154]
player_stats = df_seasons[df_seasons['player.id'].isin(player_ids)]
print(player_stats.head())
player_stats_selected = player_stats[['season', 'player.id', 'player.name', 'games.minutes', 'goals.total', 'games.appearences', 'shots.total', 'passes.total']]
player_stats_selected['goals.per_minute'] = player_stats_selected.apply(lambda row: row[4] / row[3], axis=1)
player_stats_selected['player.age'] = player_stats_selected.apply(lambda row: row[0] - birthyear_by_playerid[row[1]], axis=1) # rough age of course ;-)
print(player_stats_selected)

print(player_stats_selected.dtypes)

####### PLOTS #######
import matplotlib.pyplot as plt

print("Who is the best football player? Let the stats decide!")
ronaldo = player_stats_selected.query('`player.id` == 874')
messi   = player_stats_selected.query('`player.id` == 154')

merged_by_season = messi.merge(ronaldo, left_on='season', right_on='season', suffixes=['.messi', '.cr7'])

ax = plt.gca() # have both lines in the plot
merged_by_season.plot(x = 'season', y = 'goals.per_minute.cr7', label = 'C. Ronaldo', ax = ax)
merged_by_season.plot(x = 'season', y = 'goals.per_minute.messi', label = 'L. Messi', ax = ax)
plt.legend(loc = 'lower right')
plt.show()

merged_by_age = messi.merge(ronaldo, left_on='player.age', right_on='player.age', suffixes=['.messi', '.cr7'])
ax = plt.gca() # have both lines in the plot
merged_by_age.plot(x = 'player.age', y = 'goals.per_minute.cr7', label = 'C. Ronaldo', ax = ax)
merged_by_age.plot(x = 'player.age', y = 'goals.per_minute.messi', label = 'L. Messi', ax = ax)
plt.legend(loc = 'lower right')
plt.show()
