use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub sys: Sys,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    pub sunrise: i32,
    pub sunset: i32,
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
