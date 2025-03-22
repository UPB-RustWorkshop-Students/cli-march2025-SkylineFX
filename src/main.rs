use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventsPublisher};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Setup the terminal
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    // Create the events publisher
    let events = EventsPublisher::new(60);

    // Init the terminal user interface
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface
        tui.draw(&mut app)?;

        // Handle the events
        match tui.events.next().await {
            Ok(event) => match event {
                Event::Key(key_event) => {
                    handle_key_events(key_event, &mut app).await?;
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                app.running = false;
            }
        }
    }
    
    // Reset the terminal if the app has been terminated
    let _ = tui.exit();

    Ok(())
}
