# Mini Weather Report

A small command-line tool for retrieving local or specified weather reports.

![Image](https://github.com/user-attachments/assets/853993c7-c98b-4cb8-8a99-4d759fa35952)

## Features

* Retrieves current weather conditions.
* Supports fetching weather data for a specified city and country.
* Uses the OpenWeatherMap API.
* Configurable home location.

## Getting Started

### Prerequisites

* An OpenWeatherMap API key. You can obtain a free key at <https://openweathermap.org>.

### Installation

1.  **Clone the repository:**

    ```
    git clone https://github.com/metamaxo/mini-weather-report
    cd mini-weather-report
    ```

2.  **Install the executable:**

    ```
    sudo mv ./mini-weather-report/mini-weather-report /usr/bin/
    ```

    * This command moves the compiled binary to `/usr/local/bin/` so that it can be accessed from anywhere. You may need to adjust the destination directory based on your system and preferences.

4.  **Configure the application:**

    * Create the configuration directory:

        ```
        mkdir -p ~/.config/mini-weather-report
        ```

    * Copy the default configuration file:

        ```
        cp mini-weather-report/config.ini ~/.config/mini-weather-report/config.ini
        ```

    * Edit the `config.ini` file at `~/.config/mini-weather-report/config.ini` to set your OpenWeatherMap API key, home city, and home country.

5.  **Clean up(optional)**

    * Remove unnecessary files

        ```
        sudo rm -rf ~/mini-weather-report
        ```

## Usage

Open a terminal and run `mini-weather-report`.

* To get the weather report for your configured home location, run:

    ```
    mini-weather-report
    ```

* To get the weather report for a specific location, provide the city name and country code (e.g., DE, UK, ES) as command-line arguments:

    ```
    mini-weather-report <city-name> <country-code>
    ```

## Configuration

The `config.ini` file is located in `~/.config/mini-weather-report/`. The following options are available:

* `key`: Your OpenWeatherMap API key (required).
* `city`: Your home city (optional, used for the "home" location).
* `country`: Your home country code (optional, used for the "home" location).
