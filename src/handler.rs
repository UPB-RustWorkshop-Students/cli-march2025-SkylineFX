use crate::app::{App, AppResult};
use crossterm::event::{KeyEvent, KeyCode};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('q') => {
            app.running = false; // Quit the application when 'q' is pressed
        }
        KeyCode::Up => {
            // Move up in the list of cities
            app.scroll_up();
        }
        KeyCode::Down => {
            // Move down in the list of cities
            app.scroll_down();
        }
        KeyCode::Enter => {
            // Select the current city
            app.select_city().await;
        }
        _ => {}
    }
    Ok(())
}