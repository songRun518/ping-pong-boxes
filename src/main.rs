use rand::Rng;
use slint::{ModelRc, VecModel};

slint::include_modules!();

const WIDTH: i32 = 10;
const HEIGHT: i32 = 10;

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    init(&app);

    app.run()
}

fn init(app: &App) {
    let mut rng = rand::thread_rng();

    let win_width = app.get_win_width().ceil() as i32;
    let win_height = app.get_win_height().ceil() as i32;

    let v = (0..10)
        .map(|_| {
            let x = rng.gen_range(0..win_width - WIDTH) as i32;
            let y = rng.gen_range(0..win_height - HEIGHT) as i32;
            let vec2 = Vec2 { x, y };
            Boxs {
                vector: vec2,
                speed: 0,
                posx: x,
                posy: y,
            }
        })
        .collect::<Vec<_>>();

    app.set_boxs(ModelRc::new(VecModel::from(v)));
}
