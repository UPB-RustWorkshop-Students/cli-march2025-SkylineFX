use reqwest::Client;

pub struct CityInfo {
    pub name: String,
    pub temperature: f64,
    pub description: String,
}

// Returns weather details about a certain city
pub async fn get_data(city: &str) -> Result<CityInfo, Box<dyn std::error::Error>> {
    let api_key = "27b65c23de2d23cb0d71a8aafd988903"; // Replace with your actual API key
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let city_info = CityInfo {
            name: json["name"].as_str().unwrap_or_default().to_string(),
            temperature: json["main"]["temp"].as_f64().unwrap_or_default(),
            description: json["weather"][0]["description"].as_str().unwrap_or_default().to_string(),
        };
        Ok(city_info)
    } else {
        Err(format!("Failed to get data: {}", response.status()).into())
    }
}