use eframe::egui::{self, Align, Color32, FontId, Layout, RichText, TextStyle};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        default_theme: eframe::Theme::Dark,
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

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Clone the current style to modify it
        let mut style = (*ctx.style()).clone();
        
        // Update the text styles, ensuring all required styles are included
        style.text_styles = [
            (TextStyle::Body, FontId::proportional(18.0)),
            (TextStyle::Heading, FontId::proportional(28.0)),
            (TextStyle::Button, FontId::proportional(22.0)), // Added Button style
            // You can add more styles if needed
            // (TextStyle::Monospace, FontId::monospace(16.0)),
            // (TextStyle::Small, FontId::proportional(12.0)),
        ]
        .iter()
        .cloned()
        .collect();
        
        // Apply the updated style
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(40, 40, 40)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("Rust GUI Calculator")
                            .color(Color32::from_rgb(200, 200, 255)),
                    );
                });

                // Display screen
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.display)
                            .desired_width(300.0),
                    );
                });

                ui.add_space(10.0);

                // Calculator buttons
                let buttons = [
                    ['7', '8', '9', '+'],
                    ['4', '5', '6', '-'],
                    ['1', '2', '3', '*'],
                    ['C', '0', '=', '/'],
                ];

                for row in buttons {
                    ui.horizontal(|ui| {
                        for &ch in &row {
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(ch.to_string())
                                            .size(22.0)
                                            .color(Color32::WHITE),
                                    )
                                    .fill(Color32::from_rgb(60, 60, 90)),
                                )
                                .clicked()
                            {
                                self.process_input(ch);
                            }
                        }
                    });
                }
            });
    }
}


impl CalculatorApp {
    fn process_input(&mut self, input: char) {
        match input {
            '0'..='9' => {
                self.display.push(input);
            }
            '+' | '-' | '*' | '/' => {
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
