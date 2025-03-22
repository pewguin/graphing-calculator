mod parsing {
    pub mod function;
    pub mod ast;
}

use std::{collections::HashMap, ops::Range};

use egui::{style, widget_text, Color32, ComboBox, Painter, Pos2, Rect, RichText, Stroke, Style, Vec2, WidgetText};

use parsing::function::Function;

fn main() {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("graphing_calc_icon.png")).expect("Icon image is invalid.");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_icon(icon),
        ..Default::default()
    };
    match eframe::run_native(
        "Graphing Calculator", 
        options, 
        Box::new(|_ccl| {
            Ok(Box::<GraphApp>::default())
        })
    ) {
        Ok(window) => window,
        Err(error) => {
            println!("{}", error.to_string());
        }
    }
}

struct GraphApp {
    background_color: Color32,
    grid_color: Color32,
    grid_thickness: f32,
    grid_scaling: Vec2,

    mini_lines: i16,

    origin_ui: Pos2,

    equation: String,

    drag_offset: Pos2,
    origin_offset: Pos2,

    debug_values: HashMap<String, Vec2>,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            background_color: Color32::BLACK,
            grid_color: Color32::from_gray(100),
            grid_thickness: 0.75,
            grid_scaling: Vec2::new(80.0, 80.0),

            mini_lines: 5,

            origin_ui: Pos2::new(0.0, 0.0),

            equation: "signum(x)sin(X)-x".to_owned(),

            drag_offset: Pos2::ZERO,
            origin_offset: Pos2::ZERO,

            debug_values: HashMap::new(),
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.0);
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Graphing Calculator");
                let mut selected_option = -1;
                ComboBox::from_label("").selected_text("Graph").show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected_option, 0, "Reset Position");
                    ui.selectable_value(&mut selected_option, 1, "Reset Scale");
                });
                match selected_option {
                    -1 => {},
                    0 => {
                        self.origin_ui = Pos2::ZERO;
                    },
                    1 => {
                        self.grid_scaling = Vec2::new(80.0, 80.0);
                    },
                    _ => {
                        println!("Selected option from Graph dropdown is not valid!");
                    }
                }
            });
        });
        egui::Window::new("Equations")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let equation_label = ui.label("y=");
                    ui.text_edit_singleline(&mut self.equation)
                        .labelled_by(equation_label.id);
                })
            });
        egui::Window::new("Debug Values")
            .resizable(true)
            .show(ctx, |ui| {
                for dbg in self.debug_values.iter() {
                    ui.collapsing(dbg.0, |ui| {
                        ui.label(format!("{}", dbg.1));
                    });
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input(|i| {
                i.pointer.button_pressed(egui::PointerButton::Middle) ||
                i.pointer.button_pressed(egui::PointerButton::Secondary)
            }) {
                self.origin_offset = self.origin_ui;
                self.drag_offset = ctx.input(|i| { i.pointer.latest_pos().unwrap_or(Pos2::ZERO) });
            }

            if ctx.input(|i| {
                i.pointer.button_down(egui::PointerButton::Middle) ||
                i.pointer.button_down(egui::PointerButton::Secondary)
            }) {
                self.origin_ui = ctx.input(|i| { i.pointer.latest_pos().unwrap_or(Pos2::ZERO) }) - self.drag_offset.to_vec2() + self.origin_offset.to_vec2();
            }
            let scroll = ctx.input(|i| i.raw_scroll_delta.y);
            
            if scroll > 0.0 {
                self.grid_scaling *= scroll.powf(1.0/23.0);
            }
            else if scroll < 0.0 {
                self.grid_scaling *= 1.0/scroll.abs().powf(1.0/23.0);
            }

            let size = ui.available_size();
            let painter = ui.painter();

            painter.rect_filled(Rect::from_two_pos(Pos2::ZERO, size.to_pos2()), 0.0, self.background_color);

            self.draw_grid(painter, size);
            let top_left_gridspace = self.ui_to_grid(Pos2::new(0.0, 0.0));
            let bottom_right_gridspace = self.ui_to_grid(Pos2::new(size.x, size.y));
            self.draw_curve(painter, self.get_function_points(Function::new(&self.equation), top_left_gridspace.x..bottom_right_gridspace.x, 1000.0), (2.0, Color32::RED).into());
        });
    }
}

impl GraphApp {
    fn grid_to_ui(&self, grid_point: Pos2) -> Pos2 {
        Pos2::new(
            (grid_point.x * self.grid_scaling.x) + self.origin_ui.x,
            (-grid_point.y * self.grid_scaling.y) + self.origin_ui.y
        )
    }

    fn ui_to_grid(&self, ui_point: Pos2) -> Pos2 {
        Pos2::new(
            (ui_point.x - self.origin_ui.x) / self.grid_scaling.x,
            (ui_point.y - self.origin_ui.y) / -self.grid_scaling.y,
        )
    }

    fn draw_curve(&self, painter: &Painter, points: Vec<Pos2>, stroke: Stroke) {
        painter.line(points.iter().map(|point| {
            self.grid_to_ui(*point)
        }).collect(), stroke);
    }

    fn get_units_per_line(&mut self, screen_size: Vec2) -> Vec2 {
        let units_on_screen = Vec2::new(
            screen_size.x / self.grid_scaling.x,
            screen_size.y / self.grid_scaling.y
        );
        let mut lines_on_screen = Vec2::new(
            (self.mini_lines as f32).powf(units_on_screen.x.log(self.mini_lines as f32).floor()),
            (self.mini_lines as f32).powf(units_on_screen.y.log(self.mini_lines as f32).floor())
        );
        lines_on_screen = Vec2::new(1.0, 1.0) / lines_on_screen;
        lines_on_screen = lines_on_screen * Vec2::new(self.mini_lines as f32, self.mini_lines as f32);
        let size = Vec2::new(
            self.grid_scaling.x / lines_on_screen.x,
            self.grid_scaling.y / lines_on_screen.y
        );
        self.debug_values.insert(
            "Screen size".to_string(),
            size
        );
        self.debug_values.insert(
            "Scaling".to_string(),
            self.grid_scaling
        );
        self.debug_values.insert(
            "Lines count".to_string(),
            lines_on_screen
        );
        self.debug_values.insert(
            "Pixels between lines".to_string(),
            size
        );
        size
    }

    fn draw_grid(&mut self, painter: &Painter, size: Vec2) {
        let spacing = self.get_units_per_line(size);

        let mut x = self.origin_ui.x;
        let mut x_count = 0;

        while x <= size.x {
            let mut thickness = self.grid_thickness;
            if x_count == 0 {
                thickness = 5.0;
            }
            else if x_count % self.mini_lines == 0 {
                thickness = 2.0;
            }
            painter.line_segment([Pos2::new(x, 0.0), Pos2::new(x, size.y)], (thickness, self.grid_color));
            
            x += spacing.x;
            x_count += 1;
        }
        x = self.origin_ui.x - spacing.x;
        x_count = 1;
        while x >= 0.0 {
            let mut thickness = self.grid_thickness;
            if x_count % self.mini_lines == 0 {
                thickness = 2.0;
            }
            painter.line_segment([Pos2::new(x, 0.0), Pos2::new(x, size.y)], (thickness, self.grid_color));
            
            x -= spacing.x;
            x_count += 1;
        }

        let mut y = self.origin_ui.y;
        let mut y_count = 0;

        while y <= size.y {
            let mut thickness = self.grid_thickness;
            if y_count == 0 {
                thickness = 5.0;
            }
            else if y_count % self.mini_lines == 0 {
                thickness = 2.0;
            }
            painter.line_segment([Pos2::new(0.0, y), Pos2::new(size.x, y)], (thickness, self.grid_color));
            
            y += spacing.y;
            y_count += 1;
        }
        y = self.origin_ui.y - spacing.y;
        y_count = 1;
        while y >= 0.0 {
            let mut thickness = self.grid_thickness;
            if y_count % self.mini_lines == 0 {
                thickness = 2.0;
            }
            painter.line_segment([Pos2::new(0.0, y), Pos2::new(size.x, y)], (thickness, self.grid_color));
            y -= spacing.y;
            y_count += 1;
        }
    }

    fn get_function_points(&self, func: Function, domain: Range<f32>, resolution: f32) -> Vec<Pos2> {
        let mut points: Vec<Pos2> = Vec::new();
        let addend = (domain.end - domain.start) / resolution;

        let mut x = domain.start;
        while x < domain.end {
            points.push(Pos2::new(
                x, 
                func.eval(x)
            ));
            x += addend;
        }
        points
    }
}