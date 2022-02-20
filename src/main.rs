use nxui_template::app;

fn main() {
    let window = app::TemplateApplication::new();
    nxui::nxui::run_app(Box::new(window));
}
