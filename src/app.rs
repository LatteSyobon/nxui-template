use nxui::NativesAndMessaging::WINDOWSTYLE_OVERLAPPED;
use nxui::window::{Window, WindowAttributes};
use nxui::windowclass::WindowClass;

pub struct SampleWindow {

}

impl SampleWindow {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Window for SampleWindow {
    fn attributes(&self) -> WindowAttributes {
        let class = WindowClass::new("window".to_string(), "org.example.demo".to_string());
        let attributes = WindowAttributes::new(
            class,
            WINDOWSTYLE_OVERLAPPED,
            "NXUI Template App".to_string(),
            1280,
            750,
            500,
            500,
        );
        attributes
    }

    // fn startup(&self) {
    //     todo!()
    // }
    //
    // fn ui(&self) {
    //     todo!()
    // }
    //
    // fn run(&self, attributes: WindowAttributes) {
    //     todo!()
    // }
    //
    // fn exit(&self) {
    //     todo!()
    // }
}