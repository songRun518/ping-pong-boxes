use crate::App;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

pub fn on_moves(app: &App) {
    moves_collect(app);
    moves_follow(app);
}

fn moves_collect(app: &App) {
    let ui = app.clone_strong();
    app.on_moves_collect(move |mx, my| {
        let boxes = ui.get_boxes();

        let newboxes = boxes
            .iter()
            .map(|mut abox| {
                abox.x = mx;
                abox.y = my;

                abox
            })
            .collect::<VecModel<_>>();

        ui.set_boxes(ModelRc::new(newboxes));
    });
}

fn moves_follow(app: &App) {
    let ui = app.clone_strong();
    app.on_moves_follow(move |mx, my| {
        let boxes = ui.get_boxes();

        let newboxes = boxes
            .iter()
            .map(|mut abox| {
                let mag = (abox.speed.vx.powi(2) + abox.speed.vy.powi(2)).sqrt();
                let (dx, dy) = normalize(mx - abox.x, my - abox.y);

                abox.speed.vx = mag * dx;
                abox.speed.vy = mag * dy;

                abox
            })
            .collect::<VecModel<_>>();

        ui.set_boxes(ModelRc::new(newboxes));
    });

    fn normalize(vx: f32, vy: f32) -> (f32, f32) {
        let mag = (vx.powi(2) + vy.powi(2)).sqrt();
        (vx / mag, vy / mag)
    }
}
