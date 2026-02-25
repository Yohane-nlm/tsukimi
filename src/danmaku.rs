#[cfg(not(target_os = "macos"))]
pub use danmakw::{
    Color,
    Danmaku,
    DanmakuMode,
    DanmakwArea,
    Timer,
    init,
};

#[cfg(target_os = "macos")]
mod stub {
    use std::cell::Cell;

    use gtk::{
        glib,
        prelude::*,
        subclass::prelude::*,
    };

    #[derive(Clone, Debug, Default)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    #[derive(Clone, Debug, Default)]
    pub enum DanmakuMode {
        #[default]
        Scroll,
        TopCenter,
        BottomCenter,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Danmaku {
        pub content: String,
        pub start: f64,
        pub color: Color,
        pub mode: DanmakuMode,
    }

    pub trait Timer {
        fn time_milis(&self) -> f64;
    }

    mod imp {
        use super::*;

        #[derive(Default, glib::Properties)]
        #[properties(wrapper_type = super::DanmakwArea)]
        pub struct DanmakwArea {
            #[property(get, set, default = 36.0)]
            pub font_size: Cell<f64>,
            #[property(get, set, default = 0.0)]
            pub top_padding: Cell<f64>,
            #[property(get, set, default = 12.0)]
            pub max_lines: Cell<f64>,
            #[property(get, set, default = 6.0)]
            pub bottom_center_max_lines: Cell<f64>,
            #[property(get, set, default = 6.0)]
            pub top_center_max_lines: Cell<f64>,
            #[property(get, set, default = 1.0)]
            pub speed_factor: Cell<f64>,
            #[property(get, set, default = 0.0)]
            pub row_spacing: Cell<f64>,
            #[property(get, set, default = 1.0)]
            pub opacity: Cell<f64>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for DanmakwArea {
            const NAME: &'static str = "DanmakwArea";
            type Type = super::DanmakwArea;
            type ParentType = gtk::Widget;
        }

        #[glib::derived_properties]
        impl ObjectImpl for DanmakwArea {}

        impl WidgetImpl for DanmakwArea {}
    }

    glib::wrapper! {
        pub struct DanmakwArea(ObjectSubclass<imp::DanmakwArea>)
            @extends gtk::Widget,
            @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
    }

    impl Default for DanmakwArea {
        fn default() -> Self {
            glib::Object::new()
        }
    }

    impl DanmakwArea {
        pub fn set_enable_danmaku(&self, _enabled: bool) {}

        pub fn start_rendering<T: Timer + 'static>(&self, _timer: T) {}

        pub fn stop_rendering(&self) {}

        pub fn set_danmaku(&self, _danmaku: Vec<Danmaku>) {}

        pub fn set_time_milis(&self, _time_milis: f64) {}

        pub fn clear_danmaku(&self) {}
    }

    pub fn init() {
        let _ = DanmakwArea::static_type();
    }
}

#[cfg(target_os = "macos")]
pub use stub::{
    Color,
    Danmaku,
    DanmakuMode,
    DanmakwArea,
    Timer,
    init,
};
