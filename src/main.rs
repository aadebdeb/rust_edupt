extern crate rayon;
#[macro_use] extern crate lazy_static;

mod edupt;

fn main() {
    println!("Path tracing renderer: edupt");
    edupt::render(640, 480, 100, 2);
}
