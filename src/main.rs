use std::ops::Range;

use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2, Widget};
use regex::{Captures, Regex};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
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

    origin_ui: Pos2,

    equation: String,

    drag_offset: Pos2,
    origin_offset: Pos2,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            background_color: Color32::BLACK,
            grid_color: Color32::from_gray(100),
            grid_thickness: 0.75,
            grid_scaling: Vec2::new(80.0, 40.0),

            origin_ui: Pos2::new(0.0, 0.0),

            equation: "3*x".to_owned(),

            drag_offset: Pos2::ZERO,
            origin_offset: Pos2::ZERO,
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.0);
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.heading("Graphing Calculator");
        });
        egui::Window::new("Equations")
            .resizable(true)
            .show(ctx, |ui| {
                let equation_label = ui.label("");
                ui.text_edit_singleline(&mut self.equation)
                    .labelled_by(equation_label.id);
            });
        egui::Window::new("Debug Values")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(format!("{}", self.origin_ui));
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input(|i| {
                i.pointer.button_pressed(egui::PointerButton::Middle)
            }) {
                self.origin_offset = self.origin_ui;
                self.drag_offset = ctx.input(|i| { i.pointer.latest_pos().unwrap_or(Pos2::ZERO) });
            }

            if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Middle)) {
                self.origin_ui = ctx.input(|i| { i.pointer.latest_pos().unwrap_or(Pos2::ZERO) }) - self.drag_offset.to_vec2() + self.origin_offset.to_vec2();
            }
            self.grid_scaling += Vec2::new(1.0, 1.0) * ctx.input(|i| i.smooth_scroll_delta.y) * 0.05;

            let rect = ui.available_rect_before_wrap();
            let painter = ui.painter();

            painter.rect_filled(rect, 0.0, self.background_color);

            self.draw_grid(painter, rect);
            self.draw_curve(painter, rect, self.get_function_points(Function::new(), 0.0..10.0, 0.01), (2.0, Color32::RED).into());
            painter.arrow(self.origin_ui, Vec2::new(0.0, -self.grid_scaling.y), (3.0, Color32::BLUE));
            painter.arrow(self.origin_ui, Vec2::new(self.grid_scaling.x, 0.0), (3.0, Color32::BLUE));
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

    fn draw_curve(&self, painter: &Painter, rect: Rect, points: Vec<Pos2>, stroke: Stroke) {
        let mut prev: Option<Pos2> = Option::None;
        for point in points {
            let gui_point = self.grid_to_ui(point);
            match prev {
                Some(prev) => {
                    painter.line_segment([gui_point, prev], stroke);
                },
                _ => {}
            }
            prev = Some(gui_point);
        }
    }

    fn draw_grid(&self, painter: &Painter, rect: Rect) {
        let mut x = self.origin_ui.x % self.grid_scaling.x;
        while x <= rect.right() {
            let mut thickness = self.grid_thickness;
            if f32::abs(x - self.origin_ui.x) < 1.0 {
                thickness = 5.0;
            }
            painter.line_segment([Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())], (thickness, self.grid_color));
            x += self.grid_scaling.x;
        }

        let mut y = self.origin_ui.y % self.grid_scaling.y;
        while y <= rect.bottom() {
            let mut thickness = self.grid_thickness;
            if f32::abs(y - self.origin_ui.y) < 1.0 {
                thickness = 5.0;
            }
            painter.line_segment([Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)], (thickness, self.grid_color));
            y += self.grid_scaling.y;
        }
    }

    fn get_function_points(&self, func: Function, domain: Range<f32>, resolution: f32) -> Vec<Pos2> {
        let mut points: Vec<Pos2> = Vec::new();

        let mut x = domain.start;
        while x < domain.end {
            points.push(Pos2::new(
                x, 
                func.eval(x)
            ));
            x += resolution;
        }
        points
    }
}

struct Function {
    color: Color32,
    width: f32,
}

impl Function {
    pub fn new() -> Self {
        Self {
            color: Color32::RED,
            width: 4.0,
        }
    }

    pub fn eval(&self, x: f32)  -> f32 {
        return f32::sin(x);
    }
}