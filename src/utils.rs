use std::time::SystemTime;
use std::time::UNIX_EPOCH;

// Check if day or night using sunset and sunrise times.
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
