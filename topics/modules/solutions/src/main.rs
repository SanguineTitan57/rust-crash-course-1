use my_modules_solution::{math, util};

fn main() {
    util::log::debug(&format!("min: {}", math::min(1, 2)));
    util::log::debug(&format!("max: {}", math::max(1, 2)));
}
