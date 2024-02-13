use reqwest;
use serde::Deserialize;
use tokio;

#[derive(Deserialize)]
struct WeatherData {
    main : Main,
    weather : Vec<Weather>,
}
#[derive(Deserialize)]
struct Main {
    temp : f64,
}
#[derive(Deserialize)]
struct Weather {
    description : String,
}

async fn fetch_weather(api_key : &str , city : String) -> Result<(), reqwest::Error>{

    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city.trim(),
                                api_key);

    let response = reqwest::get(url).await?;

    if response.status().is_success() {

        let weather_data : WeatherData = response.json().await?;

        let temperature = weather_data.main.temp;
        let temperature_celsius =temperature - 273.15;

        let description = &weather_data.weather[0].description;
        println!("Weather in city: {}, {:.2}°C , {}" , city.trim(), temperature_celsius, description);

    }
    else {
        println!("Error : {}", response.status());
    }
    Ok(())

}
#[tokio::main]
async fn main() {
    let api_key = "<your_api_key>";
    let mut city = String::new();
    println!("Enter the city");

    std::io::stdin().read_line(&mut city).expect("Failed to read the line");

    tokio::spawn(fetch_weather(api_key, city));

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
 
}
