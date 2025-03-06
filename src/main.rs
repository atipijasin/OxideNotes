mod falling_note;
mod keyboard;
mod model;
mod update;
mod view;
mod midi;

use view::view;
use update::update;

fn main() {
    nannou::app(model::model)
        .update(update)
        .simple_window(view)
        .run();
}
