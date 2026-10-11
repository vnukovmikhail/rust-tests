mod app;
mod platform;
mod render;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
