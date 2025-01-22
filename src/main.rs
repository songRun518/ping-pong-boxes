#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fmt::Display, ops::RangeInclusive};

use eframe::egui;
use rand::Rng;

const COLORS: [(u8, u8, u8); 7] = [
    (255, 0, 0),
    (255, 127, 0),
    (255, 255, 0),
    (0, 255, 0),
    (0, 0, 255),
    (75, 0, 130),
    (238, 130, 238),
];

fn main() -> eframe::Result {
    let option = eframe::NativeOptions {
        run_and_return: false,
        ..Default::default()
    };

    eframe::run_native(
        "Ping Pong Boxes",
        option,
        Box::new(|_| Ok(<Box<MyApp>>::default())),
    )
}

#[derive(Debug, Default, PartialEq)]
enum DragMode {
    #[default]
    Collect,
    Follow,
}

impl Display for DragMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Self::Collect => "Collect",
            Self::Follow => "Follow",
        };
        write!(f, "{txt}")
    }
}

#[derive(Debug)]
struct MyApp {
    setup: bool,

    number: usize,
    box_size: f32,
    boxes: Vec<egui::Shape>,
    boxes_speeds: Vec<egui::Vec2>,
    speed_range: RangeInclusive<f32>,

    t_number: String,
    t_boxes_size: String,
    t_speed_range: String,

    draggable: bool,
    drag_mode: DragMode,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            setup: false,

            number: 10,
            box_size: 10.0,
            boxes: vec![],
            boxes_speeds: vec![],
            speed_range: -1.0..=1.0,

            t_number: "10".to_string(),
            t_boxes_size: "10.0".to_string(),
            t_speed_range: "1.0".to_string(),

            draggable: true,
            drag_mode: DragMode::default(),
        }
    }
}

fn build_sqare(size: f32, pos: (f32, f32), fill_color: egui::Color32) -> egui::Shape {
    let x_range = pos.0..=pos.0 + size;
    let y_range = pos.1..=pos.1 + size;
    let rect = egui::Rect::from_x_y_ranges(x_range, y_range);
    let rounding = egui::Rounding::ZERO;
    egui::Shape::rect_filled(rect, rounding, fill_color)
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let screen = ctx.input(|i| i.screen_rect);

        //setup
        if !self.setup {
            let mut rng = rand::thread_rng();

            self.boxes_speeds.clear();
            self.boxes = (0..self.number)
                .map(|idx| {
                    let xs = rng.gen_range(self.speed_range.clone());
                    let ys = rng.gen_range(self.speed_range.clone());
                    self.boxes_speeds.push(egui::vec2(xs, ys));

                    //build boxes
                    let x = rng.gen_range(0.0..=screen.width() - self.box_size);
                    let y = rng.gen_range(0.0..=screen.height() - self.box_size);
                    let (r, g, b) = COLORS[idx % COLORS.len()];
                    let fill_color = egui::Color32::from_rgb(r, g, b);

                    build_sqare(self.box_size, (x, y), fill_color)
                })
                .collect();

            self.setup = true;
            return;
        }

        //normal update
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.interact(ui.max_rect(), ui.id(), egui::Sense::drag());
            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Key {
                        key,
                        pressed,
                        repeat,
                        physical_key: _,
                        modifiers: _,
                    } = event
                    {
                        if *key == egui::Key::D && *pressed && !*repeat {
                            self.draggable = !self.draggable;
                        }
                    }
                }
            });

            //ui
            egui::SidePanel::left(egui::Id::new("new session"))
                .show_separator_line(false)
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.spacing_mut().text_edit_width = 80.0;
                    ui.spacing_mut().button_padding.x = 15.0;

                    ui.label("number:");
                    let _ = ui.text_edit_singleline(&mut self.t_number);
                    ui.label("box size:");
                    let _ = ui.text_edit_singleline(&mut self.t_boxes_size);
                    ui.label("speed:");
                    let _ = ui.text_edit_singleline(&mut self.t_speed_range);
                    ui.add_space(5.0);
                    let btn = ui.button("new");

                    if btn.clicked() {
                        if let Ok(n) = self.t_number.trim().parse::<usize>() {
                            self.number = n;
                        }
                        if let Ok(s) = self.t_boxes_size.trim().parse::<f32>() {
                            self.box_size = s;
                        }
                        if let Ok(s) = self.t_speed_range.trim().parse::<f32>() {
                            self.speed_range = -s..=s;
                        }
                        self.setup = false;
                    }
                });
            egui::SidePanel::right(egui::Id::new("control drag"))
                .show_separator_line(false)
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.checkbox(&mut self.draggable, "enable drag");
                    egui::ComboBox::from_id_salt("drag mode")
                        .selected_text(self.drag_mode.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.drag_mode,
                                DragMode::Collect,
                                DragMode::Collect.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.drag_mode,
                                DragMode::Follow,
                                DragMode::Follow.to_string(),
                            );
                        });
                });

            //normal moves
            for (idx, abox) in self.boxes.iter_mut().enumerate() {
                if !response.dragged() || !self.draggable {
                    abox.translate(self.boxes_speeds[idx]);

                    if let egui::Shape::Rect(rect) = abox {
                        let x = (rect.rect.max.x + rect.rect.min.x) / 2.0;
                        let y = (rect.rect.max.y + rect.rect.min.y) / 2.0;

                        if x <= 0.0 || x >= screen.width() {
                            self.boxes_speeds[idx].x *= -1.0;
                        }
                        if y <= 0.0 || y >= screen.height() {
                            self.boxes_speeds[idx].y *= -1.0;
                        }
                    }

                    continue;
                }

                //drag mode
                if let egui::Shape::Rect(rect) = abox {
                    match self.drag_mode {
                        DragMode::Collect => {
                            let scale = rect.rect.max - rect.rect.min;
                            rect.rect.min = response.interact_pointer_pos().unwrap();
                            rect.rect.max = rect.rect.min + scale;
                        }
                        DragMode::Follow => {
                            let delta = response.interact_pointer_pos().unwrap() - rect.rect.min;
                            self.boxes_speeds[idx] =
                                self.boxes_speeds[idx].length() * delta.normalized();
                            abox.translate(self.boxes_speeds[idx]);
                        }
                    }
                }
            }

            ui.painter().extend(self.boxes.clone());
        });
    }
}
