use rand::Rng;
use slint::{Color, Model, ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    init(&app);

    let ui = app.clone_strong();
    app.on_update(move || {
        let boxs = ui.get_boxs();
        let side_len = ui.get_side_len();
        let width = ui.get_win_width();
        let height = ui.get_win_height();

        let newboxs = boxs
            .iter()
            .map(|mut abox| {
                abox.x += abox.speed.vx;
                abox.y += abox.speed.vy;
                collision(&mut abox.speed, abox.x, abox.y, width, height, side_len);

                abox
            })
            .collect::<VecModel<_>>();

        ui.set_boxs(ModelRc::new(newboxs));
    });

    app.run()
}

fn collision(Vec2 { vx, vy }: &mut Vec2, x: f32, y: f32, width: f32, height: f32, side_len: f32) {
    if x <= 0.0 || x >= width - side_len {
        *vx = -*vx;
        // println!("vx change");
    }
    if y <= 0.0 || y >= height - side_len {
        *vy = -*vy;
        // println!("vy change");
    }
}

fn init(app: &App) {
    let width = 500.0;
    let height = 500.0;
    let side_len = 50.0;
    let mut rng = rand::thread_rng();

    let model = (0..10)
        .map(|_| {
            let vx = rng.gen_range(-5.0..=5.0);
            let vy = rng.gen_range(-5.0..=5.0);
            let speed = Vec2 { vx, vy };
            let x = rng.gen_range(0.0..width - side_len);
            let y = rng.gen_range(0.0..height - side_len);
            let color = Color::from_rgb_u8(255, 128, 0);

            // dbg!(vx, vy);
            Data { speed, x, y, color }
        })
        .collect::<VecModel<_>>();

    app.set_win_width(width);
    app.set_win_height(height);
    app.set_side_len(side_len);
    app.set_boxs(ModelRc::new(model));
}
