use iced::{
    alignment, executor, keyboard, theme,
    widget::{button, column, container, row, text, text_input},
    Alignment, Application, Color, Command, Element, Event, Length, Settings, Subscription,
};

fn main() -> iced::Result {
    Calculator::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Calculator {
    display: String,
    last_valid_expr: String,
    buttons: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed(char),
    KeyPressed(keyboard::KeyCode),
    Ignore,
}

impl Application for Calculator {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let buttons = vec![
            vec!['7', '8', '9', '/'],
            vec!['4', '5', '6', '*'],
            vec!['1', '2', '3', '-'],
            vec!['C', '0', '=', '+'],
        ];

        (
            Calculator {
                display: String::new(),
                last_valid_expr: String::new(),
                buttons,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Iced Calculator")
    }

    fn theme(&self) -> theme::Theme {
        // A dark theme
        let theme = iced::Theme::custom(iced::theme::Palette {
            background: iced::Color::from_rgb(0.1, 0.1, 0.1),
            text: iced::Color::WHITE,
            primary: iced::Color::from_rgb(0.2, 0.2, 0.3),
            success: iced::Color::from_rgb(0.1, 0.6, 0.1),
            danger: iced::Color::from_rgb(0.8, 0.1, 0.1),
        });
        theme
    
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(new_value) => {
                self.display = new_value.clone();
                // Try to evaluate as user types
                if let Some(result) = evaluate_expression(&new_value) {
                    self.last_valid_expr = result.to_string();
                }
            }
            Message::ButtonPressed(ch) => {
                self.process_input(ch);
            }
            Message::KeyPressed(key) => {
                // Handle keyboard events: numbers, operators, etc.
                match key {
                    keyboard::KeyCode::Key0 => self.process_input('0'),
                    keyboard::KeyCode::Key1 => self.process_input('1'),
                    keyboard::KeyCode::Key2 => self.process_input('2'),
                    keyboard::KeyCode::Key3 => self.process_input('3'),
                    keyboard::KeyCode::Key4 => self.process_input('4'),
                    keyboard::KeyCode::Key5 => self.process_input('5'),
                    keyboard::KeyCode::Key6 => self.process_input('6'),
                    keyboard::KeyCode::Key7 => self.process_input('7'),
                    keyboard::KeyCode::Key8 => self.process_input('8'),
                    keyboard::KeyCode::Key9 => self.process_input('9'),
                    keyboard::KeyCode::Plus => self.process_input('+'),
                    keyboard::KeyCode::Minus => self.process_input('-'),
                    keyboard::KeyCode::Asterisk => self.process_input('*'),
                    keyboard::KeyCode::Slash => self.process_input('/'),
                    keyboard::KeyCode::Delete | keyboard::KeyCode::Backspace => {
                        self.display.pop();
                        if let Some(result) = evaluate_expression(&self.display) {
                            self.last_valid_expr = result.to_string();
                        }
                    }
                    keyboard::KeyCode::Enter | keyboard::KeyCode::NumpadEnter => {
                        // On Enter, finalize calculation
                        if let Some(val) = evaluate_expression(&self.display) {
                            self.display = val.to_string();
                            self.last_valid_expr = self.display.clone();
                        }
                    }
                    keyboard::KeyCode::Escape => {
                        self.display.clear();
                        self.last_valid_expr.clear();
                    }
                    keyboard::KeyCode::Period => {
                        if !self.display.contains('.') {
                            self.display.push('.');
                        }
                    }
                    _ => {}
                }
            }
            Message::Ignore => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::subscription::events().map(|event| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                Message::KeyPressed(key_code)
            }
            _ => Message::Ignore,
        })
    }

    fn view(&self) -> Element<Message> {
        // Create a styled container with a column layout
        let title = text("Rust Iced Calculator")
            .size(40)
            .horizontal_alignment(alignment::Horizontal::Center);

        let input = text_input("Type expression...", &self.display)
            .on_input(Message::InputChanged)
            .size(32)
            .padding(10)
            .width(Length::Fill);
        

        let mut rows = column![].spacing(10);

        for row_buttons in &self.buttons {
            let mut button_row = row![];
            for &ch in row_buttons {
                let (bg, txt_color) = match ch {
                    'C' => (Color::from_rgb(0.8, 0.1, 0.1), Color::WHITE),
                    '=' => (Color::from_rgb(0.1, 0.6, 0.1), Color::WHITE),
                    '+' | '-' | '*' | '/' => (Color::from_rgb(0.2, 0.2, 0.5), Color::WHITE),
                    _ => (Color::from_rgb(0.2, 0.2, 0.3), Color::WHITE),
                };

                let btn = button(
                    text(ch.to_string())
                        .size(28)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .width(Length::Fixed(80.0))
                .height(Length::Fixed(60.0))                
                .on_press(Message::ButtonPressed(ch))
                .style(theme::Button::Custom(Box::new(ButtonStyle { bg, txt_color })));

                button_row = button_row.push(btn);
            }
            rows = rows.push(button_row.spacing(10));
        }

        // Footer
        let footer = text("Made with ❤️ using Rust and Iced")
            .size(16)
            .horizontal_alignment(alignment::Horizontal::Center)
            .style(theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7)));



        let content = column![
            title,
            input,
            rows,
            footer
        ]
        .align_items(Alignment::Center)
        .padding(20)
        .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(ContainerStyle) // Pass the struct implementing StyleSheet
            .into()
    }
}

impl Calculator {
    fn process_input(&mut self, ch: char) {
        match ch {
            '0'..='9' => {
                self.display.push(ch);
                if let Some(result) = evaluate_expression(&self.display) {
                    self.last_valid_expr = result.to_string();
                }
            }
            '.' => {
                if !self.display.contains('.') {
                    self.display.push(ch);
                }
            }
            '+' | '-' | '*' | '/' => {
                // Ensure expression is valid before adding operator
                if !self.display.is_empty() {
                    let last_char = self.display.chars().last().unwrap();
                    if "+-*/".contains(last_char) {
                        // Replace the operator
                        self.display.pop();
                    }
                    self.display.push(ch);
                }
            }
            '=' => {
                // Evaluate the expression
                if let Some(val) = evaluate_expression(&self.display) {
                    self.display = val.to_string();
                    self.last_valid_expr = self.display.clone();
                }
            }
            'C' => {
                self.display.clear();
                self.last_valid_expr.clear();
            }
            _ => {}
        }
    }
}

fn evaluate_expression(expr: &str) -> Option<f64> {
    meval::eval_str(expr).ok()
}


// --- Styling ---

struct ContainerStyle;

impl iced::widget::container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, theme: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(theme.palette().background)),
            text_color: Some(theme.palette().text),
            border_radius: 10.0.into(),
            ..Default::default()
        }
    }
}





struct ButtonStyle {
    bg: Color,
    txt_color: Color,
}

impl iced::widget::button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(self.bg)),
            border_radius: 10.0.into(),
            text_color: self.txt_color,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(iced::Color {
                r: (self.bg.r + 0.1).min(1.0),
                g: (self.bg.g + 0.1).min(1.0),
                b: (self.bg.b + 0.1).min(1.0),
                a: self.bg.a,
            })),
            border_radius: 10.0.into(),
            text_color: self.txt_color,
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(iced::Color {
                r: (self.bg.r - 0.1).max(0.0),
                g: (self.bg.g - 0.1).max(0.0),
                b: (self.bg.b - 0.1).max(0.0),
                a: self.bg.a,
            })),
            border_radius: 10.0.into(),
            text_color: self.txt_color,
            ..Default::default()
        }
    }
}

