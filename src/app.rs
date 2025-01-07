use crate::keylight::{self, Light};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Gauge},
    DefaultTerminal, Frame,
};
use tempergb::rgb_from_temperature;

#[derive(Debug, Default)]
pub struct App {
    // IP address of the key light
    ip: String,

    /// Is the application running?
    running: bool,

    // Light state
    on: bool,
    brightness: i32,  // 0-100
    temperature: i32, // 143-344   / 0.05 to get kelvin values
}

impl App {
    pub fn new(ip: String, light: Light) -> Self {
        App {
            ip,
            running: false,
            on: if light.on == 1 { true } else { false },
            brightness: light.brightness,
            temperature: light.temperature,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from(" Elgato Key Light ")
            .bold()
            .light_magenta()
            .centered();
        let instructions = Line::from(vec![
            "  Dimmer ".into(),
            "<Left>".light_magenta().bold(),
            "  Brighter ".into(),
            "<Right>".light_magenta().bold(),
            "  Warmer ".into(),
            "<Up>".light_magenta().bold(),
            "  Colder ".into(),
            "<Down>".light_magenta().bold(),
            "  Toggle off/on ".into(),
            "<Space>".light_magenta().bold(),
            "  Quit ".into(),
            "<Q> ".light_magenta().bold(),
        ]);

        let bright_style = if self.on {
            Style::new().white().on_black().italic()
        } else {
            Style::new().dark_gray().on_black().italic()
        };

        let brightness_gauge = Gauge::default()
            .gauge_style(bright_style)
            .percent(self.brightness as u16);

        let ratio = f64::from(self.temperature.clamp(143, 344) - 143) / 201.0;
        let temp_in_kelvin = f64::floor(4100.0 * (1.0 - ratio) + 2900.0);
        let temp_style = if self.on {
            let rgb = rgb_from_temperature(temp_in_kelvin);
            Style::new()
                .fg(Color::Rgb(rgb.r(), rgb.g(), rgb.b()))
                .on_black()
                .italic()
        } else {
            Style::new().dark_gray().on_black().italic()
        };
        let temperature_gauge = Gauge::default()
            .gauge_style(temp_style)
            .ratio(ratio)
            .label(format!("{} K", temp_in_kelvin));

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(100)])
            .split(frame.area());

        let border = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered());

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(border.inner(outer_layout[0]));

        frame.render_widget(border, outer_layout[0]);

        frame.render_widget(brightness_gauge, layout[0]);

        frame.render_widget(temperature_gauge, layout[1])
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            (KeyModifiers::SHIFT, KeyCode::Left) => self.change_brightness(-1),
            (KeyModifiers::SHIFT, KeyCode::Right) => self.change_brightness(1),
            (_, KeyCode::Left) => self.change_brightness(-10),
            (_, KeyCode::Right) => self.change_brightness(10),
            (KeyModifiers::SHIFT, KeyCode::Up) => self.change_temperature(1),
            (KeyModifiers::SHIFT, KeyCode::Down) => self.change_temperature(-1),
            (_, KeyCode::Up) => self.change_temperature(10),
            (_, KeyCode::Down) => self.change_temperature(-10),
            (_, KeyCode::Char(' ') | KeyCode::Enter) => self.set_on(!self.on),
            _ => {}
        }
    }

    fn change_brightness(&mut self, change: i32) {
        self.brightness = (self.brightness + change).clamp(0, 100);
        self.set_light();
    }

    fn change_temperature(&mut self, change: i32) {
        self.temperature = (self.temperature + change).clamp(143, 344);
        self.set_light();
    }

    fn set_on(&mut self, on: bool) {
        self.on = on;
        self.set_light();
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn set_light(&mut self) {
        if self.running {
            let light = Light::new(self.on, self.brightness, self.temperature);
            keylight::set_light(&self.ip, light).expect("failed to set light");
        }
    }
}
