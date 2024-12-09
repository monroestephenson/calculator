use eframe::egui::{
    self, Align, Color32, FontId, Layout, RichText, TextStyle, Rounding, Stroke, Vec2,
};
use eframe::egui::Frame;
use eframe::{App, NativeOptions};

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        // Updated field name from `initial_window_size` to `window_size`
        viewport: Some(Vec2::new(400.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rust GUI Calculator",
        options,
        Box::new(|_cc| Box::new(CalculatorApp::default())),
    )
}

#[derive(Default)]
struct CalculatorApp {
    display: String,
    operation: Option<char>,
    operand: Option<f64>,
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Clone and modify the current style
        let mut style = (*ctx.style()).clone();

        // Start with dark visuals
        style.visuals = egui::Visuals::dark();

        // Update the text styles with larger, more readable fonts
        style.text_styles = [
            (TextStyle::Body, FontId::proportional(24.0)),
            (TextStyle::Heading, FontId::proportional(32.0)),
            (TextStyle::Button, FontId::proportional(24.0)),
            (TextStyle::Monospace, FontId::monospace(20.0)),
        ]
        .iter()
        .cloned()
        .collect();

        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(
                        RichText::new("Rust GUI Calculator")
                            .color(Color32::from_rgb(200, 200, 255)),
                    );
                });

                // Display screen with customized frame
                ui.add_space(20.0);
                egui::Frame::none()
                    .fill(Color32::from_rgb(50, 50, 50))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(70, 130, 180)))
                    .rounding(Rounding::same(10.0))
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.display)
                                .font(FontId::monospace(32.0))
                                .desired_width(f32::INFINITY)
                                .text_color(Color32::WHITE)
                                .frame(false)
                                .margin(Vec2::splat(10.0)),
                        );
                    });

                ui.add_space(20.0);

                // Calculator buttons with improved styling
                let buttons = [
                    ['7', '8', '9', '/'],
                    ['4', '5', '6', '*'],
                    ['1', '2', '3', '-'],
                    ['C', '0', '=', '+'],
                ];

                for row in buttons.iter() {
                    ui.horizontal(|ui| {
                        for &ch in row.iter() {
                            // Determine button color based on its function
                            let button_color = match ch {
                                'C' => Color32::from_rgb(220, 20, 60),   // Crimson
                                '=' => Color32::from_rgb(34, 139, 34),  // Forest Green
                                _ => Color32::from_rgb(70, 130, 180),  // Steel Blue
                            };
                            let text_color = Color32::WHITE;

                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(ch.to_string())
                                            .size(28.0)
                                            .color(text_color),
                                    )
                                    .fill(button_color)
                                    .min_size(Vec2::new(80.0, 60.0))
                                    .rounding(Rounding::same(10.0)),
                                )
                                .clicked()
                            {
                                self.process_input(ch);
                            }
                        }
                    });
                    ui.add_space(10.0);
                }

                // Add some footer or additional info if desired
                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("Made with ❤️ using Rust and eframe/egui")
                            .color(Color32::LIGHT_GRAY),
                    );
                });
            });
    }
}

impl CalculatorApp {
    fn process_input(&mut self, input: char) {
        match input {
            '0'..='9' => {
                self.display.push(input);
            }
            '.' => {
                if !self.display.contains('.') {
                    self.display.push(input);
                }
            }
            '+' | '-' | '*' | '/' => {
                if let Some(last_char) = self.display.chars().last() {
                    if "+-*/".contains(last_char) {
                        self.display.pop(); // Replace the operator
                    }
                }
                if let Ok(num) = self.display.parse::<f64>() {
                    self.operand = Some(num);
                    self.operation = Some(input);
                    self.display.clear();
                }
            }
            '=' => {
                if let (Some(op), Some(operand)) = (self.operation, self.operand) {
                    if let Ok(second_operand) = self.display.parse::<f64>() {
                        let result = match op {
                            '+' => operand + second_operand,
                            '-' => operand - second_operand,
                            '*' => operand * second_operand,
                            '/' => {
                                if second_operand != 0.0 {
                                    operand / second_operand
                                } else {
                                    self.display = "Error".to_string();
                                    self.operand = None;
                                    self.operation = None;
                                    return;
                                }
                            }
                            _ => return,
                        };
                        self.display = result.to_string();
                        self.operand = None;
                        self.operation = None;
                    }
                }
            }
            'C' => {
                self.display.clear();
                self.operation = None;
                self.operand = None;
            }
            _ => {}
        }
    }
}
