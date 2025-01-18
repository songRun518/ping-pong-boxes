#![windows_subsystem = "windows"]

use rand::Rng;
use slint::{Color, Model, ModelRc, VecModel};

slint::include_modules!();

const COLORS: [(u8, u8, u8); 24] = [
    (0, 0, 128),
    (0, 0, 255),
    (0, 128, 0),
    (0, 128, 128),
    (0, 128, 255),
    (0, 255, 0),
    (0, 255, 128),
    (0, 255, 255),
    (128, 0, 0),
    (128, 0, 128),
    (128, 0, 255),
    (128, 128, 0),
    (128, 128, 255),
    (128, 255, 0),
    (128, 255, 128),
    (128, 255, 255),
    (255, 0, 0),
    (255, 0, 128),
    (255, 0, 255),
    (255, 128, 0),
    (255, 128, 128),
    (255, 128, 255),
    (255, 255, 0),
    (255, 255, 128),
];

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    init(&app);

    let ui = app.clone_strong();
    app.on_update(move || {
        let boxes = ui.get_boxes();
        let side_len = ui.get_side_len();
        let width = ui.get_win_width();
        let height = ui.get_win_height();

        let newboxes = boxes
            .iter()
            .map(|mut abox| {
                abox.x += abox.speed.vx;
                abox.y += abox.speed.vy;

                if abox.x <= 0.0 || abox.x >= width - side_len {
                    abox.speed.vx = -abox.speed.vx;
                }
                if abox.y <= 0.0 || abox.y >= height - side_len {
                    abox.speed.vy = -abox.speed.vy;
                }

                abox
            })
            .collect::<VecModel<_>>();

        ui.set_boxes(ModelRc::new(newboxes));
    });

    app.run()
}

fn init(app: &App) {
    let width = 500.0;
    let height = 500.0;
    let side_len = 50.0;
    let mut rng = rand::thread_rng();

    let model = (0..24)
        .map(|idx| {
            let vx = rng.gen_range(-5.0..=5.0);
            let vy = rng.gen_range(-5.0..=5.0);
            let speed = Vec2 { vx, vy };
            let x = rng.gen_range(0.0..width - side_len);
            let y = rng.gen_range(0.0..height - side_len);
            let (r, g, b) = COLORS[idx];
            let color = Color::from_rgb_u8(r, g, b);

            Data { speed, x, y, color }
        })
        .collect::<VecModel<_>>();

    app.set_win_width(width);
    app.set_win_height(height);
    app.set_side_len(side_len);
    app.set_boxes(ModelRc::new(model));
}
