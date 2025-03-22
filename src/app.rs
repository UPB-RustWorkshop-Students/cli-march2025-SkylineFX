use crate::connection::get_data;
use ratatui::widgets::ListState;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct App {
    pub running: bool,
    pub cities: Vec<String>,
    pub list_state: ListState,
    pub weather_info: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            running: true,
            cities: vec!["Bucharest".to_string(), "London".to_string(), "Lisbon".to_string()],
            list_state,
            weather_info: String::new(),
            // Initialize other fields...
        }
    }

    pub fn scroll_up(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected > 0 {
                self.list_state.select(Some(selected - 1));
            }
        }
    }

    pub fn scroll_down(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.cities.len() - 1 {
                self.list_state.select(Some(selected + 1));
            }
        }
    }

    pub async fn select_city(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let city = &self.cities[selected];
            match get_data(city).await {
                Ok(city_info) => {
                    self.weather_info = format!(
                        "City: {}\nTemperature: {}°C\nDescription: {}\n",
                        city_info.name, city_info.temperature, city_info.description
                    );
                }
                Err(e) => {
                    self.weather_info = format!("Failed to fetch weather info: {}", e);
                }
            }
        }
    }
}
