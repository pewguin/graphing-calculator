#![allow(unused)]

mod parsing {
    pub mod ast;
    pub mod parser;
}

use std::{collections::HashMap, ops::Range};

use egui::{style, widget_text, Color32, ComboBox, Painter, Pos2, Rect, RichText, Sense, Stroke, Style, Vec2, WidgetText};
use crate::parsing::ast::Value;
use crate::parsing::parser::{parse, tokenize};

fn main() {
    // println!("{}", parse(tokenize(&"(5+4)*3".to_string())).unwrap());
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("graphing_calc_icon.png")).expect("Icon image is invalid.");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "Graphing Calculator",
        options,
        Box::new(|_ccl| {
            Ok(Box::<GraphApp>::default())
        })
    ).unwrap_or_else(|error| {
        println!("{}", error.to_string());
    });
}

struct GraphApp {
    background_color: Color32,
    grid_origin: Vec2,
    grid_size: Vec2,

    grid_color: Color32,
    grid_thickness: f32,
    grid_scaling: Vec2,

    mini_lines: i16,

    origin_ui: Pos2,

    function_value: Box<Value>,
    function_str: String,

    drag_offset: Pos2,
    origin_offset: Pos2,

    debug_values: HashMap<String, Vec2>,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            background_color: Color32::BLACK,
            grid_origin: Vec2::ZERO,
            grid_size: Vec2::ZERO,

            grid_color: Color32::from_gray(100),
            grid_thickness: 0.75,
            grid_scaling: Vec2::new(80.0, 80.0),

            mini_lines: 5,

            origin_ui: Pos2::new(0.0, 0.0),

            function_value: Box::new(Value::Variable(0)),
            function_str: String::from("x*x"),

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
                        self.origin_ui = (self.grid_size.to_pos2() + self.grid_origin) / 2.0;
                        self.debug_values.insert(
                            "Reset Pos".to_string(),
                            self.origin_ui.to_vec2(),
                        );
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
                    if ui.text_edit_singleline(&mut self.function_str)
                        .labelled_by(equation_label.id)
                        .changed() {
                        let new_func = parse(tokenize(&self.function_str));
                        match new_func {
                            Ok(func) => {
                                println!("{:?}", func);
                                self.function_value = func
                            },
                            Err(e) => ()//println!("{}", e)
                        }
                    }
                })
            });
        egui::Window::new("Debug")
            .resizable(true)
            .show(ctx, |ui| {
                if ui.button("Log function information").clicked() {
                    println!();
                }
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

            let response = ui.allocate_response(ui.available_size(), Sense::all());
            let rect = response.rect;
            let painter = ui.painter();

            self.grid_origin = rect.min.to_vec2();
            self.grid_size = rect.size();

            painter.rect_filled(Rect::from_two_pos(rect.min, rect.max), 0.0, self.background_color);

            self.draw_grid(painter, rect);
            let top_left_gridspace = self.ui_to_grid(rect.min);
            let bottom_right_gridspace = self.ui_to_grid(rect.max);
            self.draw_curve(painter, self.get_function_points(&*self.function_value, top_left_gridspace.x..bottom_right_gridspace.x, 1000.0), (2.0, Color32::RED).into());
        });
    }
}

impl GraphApp {
    fn grid_to_ui(&self, grid_point: Pos2) -> Pos2 {
        Pos2::new(
            (grid_point.x * self.grid_scaling.x) + self.origin_ui.x + self.grid_origin.x,
            (-grid_point.y * self.grid_scaling.y) + self.origin_ui.y + self.grid_origin.y
        )
    }

    fn ui_to_grid(&self, ui_point: Pos2) -> Pos2 {
        Pos2::new(
            (ui_point.x - self.origin_ui.x - self.grid_origin.x) / self.grid_scaling.x,
            (ui_point.y - self.origin_ui.y - self.grid_origin.y) / -self.grid_scaling.y,
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

    fn draw_grid(&mut self, painter: &Painter, rect: Rect) {
        let spacing = self.get_units_per_line(rect.size());
        let ui_origin = self.origin_ui + rect.min.to_vec2();

        let mut x = rect.min.x + self.origin_ui.x.rem_euclid(spacing.x);
        let mini_lines_start = rect.min.x + self.origin_ui.x % (spacing.x * self.mini_lines as f32);
        let mut x_count = ((x - mini_lines_start) / spacing.x) as i32;

        while x <= rect.max.x {
            let mut thickness = self.grid_thickness;
            if (x_count >= 0 && x_count % self.mini_lines as i32 == 0) {
                thickness = 2.0;
            }

            painter.line_segment(
                [Pos2::new(x, rect.min.y), 
                    Pos2::new(x, rect.max.y)],
                (thickness, self.grid_color));
            
            x += spacing.x;
            x_count += 1;
        }

        if (0.0 < self.origin_ui.x && self.origin_ui.x < rect.size().x) {
            let x = self.origin_ui.x + rect.min.x;
            painter.line_segment(
                [Pos2::new(x, rect.min.y), 
                Pos2::new(x, rect.max.y)], 
                (5.0, self.grid_color));
        }

        let mut y = rect.min.y + self.origin_ui.y.rem_euclid(spacing.y);
        let mini_lines_start = rect.min.y + self.origin_ui.y % (spacing.y * self.mini_lines as f32);
        let mut y_count = ((y - mini_lines_start) / spacing.y) as i32;

        while y <= rect.max.y {
            let mut thickness = self.grid_thickness;
            if (y_count >= 0 && y_count % self.mini_lines as i32 == 0) {
                thickness = 2.0;
            }

            painter.line_segment(
                [Pos2::new(rect.min.x, y), 
                    Pos2::new(rect.max.x, y)],
                (thickness, self.grid_color));
            
            y += spacing.y;
            y_count += 1;
        }

        if (0.0 < self.origin_ui.y && self.origin_ui.y < rect.size().y) {
            let y = self.origin_ui.y + rect.min.y;
            painter.line_segment(
                [Pos2::new(rect.min.x, y ), 
                Pos2::new(rect.max.x, y)], 
                (5.0, self.grid_color));
        }
    }

    fn get_function_points(&self, func: &Value, domain: Range<f32>, resolution: f32) -> Vec<Pos2> {
        let mut points: Vec<Pos2> = Vec::new();
        let addend = (domain.end - domain.start) / resolution;

        let mut x = domain.start;
        while x < domain.end {
            points.push(Pos2::new(
                x, 
                func.evaluate(&vec![x]),
            ));
            x += addend;
        }
        points
    }
}