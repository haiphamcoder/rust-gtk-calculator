mod config;
mod app;

use app::CalculateApp;

fn main() {
    let app = CalculateApp::new("org.gtk3-rs.calculator", "config.cfg");
    app.run();
}
