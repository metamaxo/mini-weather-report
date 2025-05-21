use crate::utils::fetch_daytime;
use crate::weather_response::openweathermap;
use std::env;

mod ascii;
mod config;
mod utils;
mod weather_response;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Fetch args
    let args: Vec<String> = env::args().collect();

    // Fetch config
    let (key, mut city, mut country) = config::fetch_config()?;

    // Check if location is specified
    if args.len() > 2 {
        // If city name is multiple words, collect into a vec. Country can only have 1 word.
        // Print city name and country if location is specified
        city = args[1..(args.len() - 1)].join(" ");
        country = args[args.len() - 1].clone();
        println!("weather report for: {city}, {country}")
    }

    // Get weather data, if not succesful, exit.
    let result = match openweathermap(&key, &city, &country).await {
        Ok(response) => response,
        Err(_e) => {
            if args.len() < 2 {
                println!("unable to retrieve weather data, please check your configuration");
            } else {
                println!("unable to retrieve weather data, please check your arguments",);
            }
            return Ok(());
        }
    };

    // Parse API results
    let weather_type = &result.weather[0].main;
    let weather_subtype = &result.weather[0].description;
    let temperature = result.main.temp.unwrap_or_default();
    let id = result.weather[0].id;

    // Check if its day or night
    let daytime = fetch_daytime(result.sys.sunrise, result.sys.sunset);

    // Fetch the correct ascii art
    let ascii = ascii::ascii(id, &daytime);

    // Print weather report
    println!("{} weather type: {}", ascii[0], weather_type);
    println!("{} subtype: {}", ascii[1], weather_subtype);
    println!("{} temperature: {:?}", ascii[2], temperature);
    Ok(())
}
