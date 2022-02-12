use nxui::Demo::demowindow;
use nxui_template::app;

fn main() {
    let window = app::SampleWindow::new();
    nxui::nxui::run(Box::new(window));
}
