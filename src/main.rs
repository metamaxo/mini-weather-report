use serde::Deserialize;
use std::env;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
mod ascii;
mod config;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Deserialize, Debug)]
pub struct Coord {
    pub lon: Option<f64>,
    pub lat: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: Option<f64>,
    #[serde(rename = "feels_like")]
    pub feels_like: Option<f64>,
    #[serde(rename = "temp_min")]
    pub temp_min: Option<f64>,
    #[serde(rename = "temp_max")]
    pub temp_max: Option<f64>,
    pub pressure: Option<i32>,
    pub humidity: Option<i32>,
    #[serde(rename = "sea_level")]
    pub sea_level: Option<i32>,
    #[serde(rename = "grnd_level")]
    pub grnd_level: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: Option<f64>,
    pub deg: Option<i32>,
    pub gust: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: Option<i32>,
    pub id: i32,
    pub country: Option<String>,
    pub sunrise: i32,
    pub sunset: i32,
}

pub fn fetch_daytime(sunrise: i32, sunset: i32) -> String {
    // get current time
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32;
    // find day time

    if sunrise <= current_time && current_time <= sunset {
        String::from("day")
    } else {
        String::from("night")
    }
}

// Get API response from openweathermap
pub async fn openweathermap(
    key: &str,
    city: &str,
    country_code: &str,
) -> Result<WeatherResponse, anyhow::Error> {
    let body = reqwest::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, key
    ))
    .await?
    .text()
    .await?;
    Ok(serde_json::from_str(&body)?)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Fetch args
    let args: Vec<String> = env::args().collect();
    // Fetch config
    let (key, mut city, mut country) = config::fetch_config()?;
    let mut city_vec = Vec::new();
    // set mode
    let mut home_mode = true;
    // If args is "home", give home weather report, else give weather report for args.
    if !args.contains(&"home".to_string()) && args.len() > 2 {
        home_mode = false;
        for arg in &args[1..(args.len() - 1)] {
            city_vec.push(arg.to_string())
        }
        city = city_vec.join(" ");
        country = args[args.len() - 1].clone();
    }

    // Get weather data, if not succesful, exit.
    let result = match openweathermap(&key, &city, &country).await {
        Ok(response) => response,
        Err(_e) => {
            if home_mode {
                println!("unable to retrieve weather data, please check your configuration");
            } else {
                println!("unable to retrieve weather data, please check your arguments",);
            }
            return Ok(());
        }
    };

    // Parse API results
    let weather_type = result.weather[0].main.clone();
    let weather_subtype = result.weather[0].description.clone();
    let temperature = result.main.temp.unwrap();
    let id = result.weather[0].id;

    // Check if its day or night
    let daytime = fetch_daytime(result.sys.sunrise, result.sys.sunset);
    // Fetch the correct ascii art
    let ascii = ascii::ascii(id, &daytime);

    // Print weather report
    if !home_mode {
        println!("mini weather report for: {} {}", city, country);
    } else {
        println!("mini weather report:");
    }
    println!("{} weather type: {}", ascii[0], weather_type);
    println!("{} subtype: {}", ascii[1], weather_subtype);
    println!("{} temperature: {:?}", ascii[2], temperature);
    Ok(())
}
