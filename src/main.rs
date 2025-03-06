mod model;
mod view;
mod keyboard;

use view::view;

fn main() {
    nannou::app(model::model).simple_window(view).run();
}