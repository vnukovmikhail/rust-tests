mod app;
mod platform;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
