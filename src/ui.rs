use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::Frame;
use crate::app::App;

/// Renders the user interface widgets.
pub fn render(frame: &mut Frame, app: &mut App) {
    // Split the layout into two areas
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(frame.size());

    // Get the list of cities
    let cities: Vec<ListItem> = app.cities.iter().map(|city| {
        ListItem::new(city.clone())
    }).collect();
    let list_component = List::new(cities)
        .block(Block::default().borders(Borders::ALL).title("Cities"))
        .highlight_symbol(">>")
        .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));

    // Render the list of cities
    frame.render_stateful_widget(list_component, chunks[0], &mut app.list_state);

    // Create the weather info component
    let weather_info = Paragraph::new(app.weather_info.clone())
        .block(Block::default().borders(Borders::ALL).title("Weather Info"));

    // Render the weather info component
    frame.render_widget(weather_info, chunks[1]);
}
