extern crate yew;
extern crate search;

use yew::prelude::*;
use search::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
