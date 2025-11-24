// UI Layer
use crate::calculator::Calculator;
use crate::operation::Operation;

pub struct CalculatorApp {
    calculator: Calculator,
}

impl CalculatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            calculator: Calculator::new(),
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // Display area with background
                ui.group(|ui| {
                    ui.set_min_width(280.0);
                    ui.set_min_height(60.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(self.calculator.get_display_text())
                                .size(36.0)
                                .monospace()
                        );
                        ui.add_space(10.0);
                    });
                });
                
                ui.add_space(20.0);
                
                // Button grid (4x4)
                egui::Grid::new("calculator_grid")
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        // Row 1: 7, 8, 9, ÷
                        for digit in 7..=9 {
                            if ui.add_sized([65.0, 65.0], 
                                egui::Button::new(egui::RichText::new(digit.to_string()).size(24.0))
                            ).clicked() {
                                self.calculator.input_digit(digit);
                            }
                        }
                        if ui.add_sized([65.0, 65.0], 
                            egui::Button::new(egui::RichText::new("÷").size(24.0))
                        ).clicked() {
                            self.calculator.input_operation(Operation::Divide);
                        }
                        ui.end_row();
                        
                        // Row 2: 4, 5, 6, ×
                        for digit in 4..=6 {
                            if ui.add_sized([65.0, 65.0], 
                                egui::Button::new(egui::RichText::new(digit.to_string()).size(24.0))
                            ).clicked() {
                                self.calculator.input_digit(digit);
                            }
                        }
                        if ui.add_sized([65.0, 65.0], 
                            egui::Button::new(egui::RichText::new("×").size(24.0))
                        ).clicked() {
                            self.calculator.input_operation(Operation::Multiply);
                        }
                        ui.end_row();
                        
                        // Row 3: 1, 2, 3, -
                        for digit in 1..=3 {
                            if ui.add_sized([65.0, 65.0], 
                                egui::Button::new(egui::RichText::new(digit.to_string()).size(24.0))
                            ).clicked() {
                                self.calculator.input_digit(digit);
                            }
                        }
                        if ui.add_sized([65.0, 65.0], 
                            egui::Button::new(egui::RichText::new("-").size(24.0))
                        ).clicked() {
                            self.calculator.input_operation(Operation::Subtract);
                        }
                        ui.end_row();
                        
                        // Row 4: 0, ., =, +
                        if ui.add_sized([65.0, 65.0], 
                            egui::Button::new(egui::RichText::new("0").size(24.0))
                        ).clicked() {
                            self.calculator.input_digit(0);
                        }
                        if ui.add_sized([65.0, 65.0], 
                            egui::Button::new(egui::RichText::new(".").size(24.0))
                        ).clicked() {
                            self.calculator.input_decimal_point();
                        }
                        if ui.add_sized([65.0, 65.0], 
                            egui::Button::new(egui::RichText::new("=").size(24.0))
                        ).clicked() {
                            self.calculator.calculate();
                        }
                        if ui.add_sized([65.0, 65.0], 
                            egui::Button::new(egui::RichText::new("+").size(24.0))
                        ).clicked() {
                            self.calculator.input_operation(Operation::Add);
                        }
                        ui.end_row();
                    });
                
                ui.add_space(15.0);
                
                // Clear button (full width)
                if ui.add_sized([292.0, 50.0], 
                    egui::Button::new(egui::RichText::new("Clear").size(20.0))
                ).clicked() {
                    self.calculator.clear();
                }
            });
        });
    }
}
