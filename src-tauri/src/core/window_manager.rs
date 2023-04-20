pub enum WindowType {
    // TODO: set system config move to config window
    Config,
    Main,
}
pub struct WindowInfo {
    pub label: String,
    pub title: String,
    pub url: String,
    pub width: f64,
    pub height: f64,
    pub resizable: bool,
    pub maximized: bool,
    pub fullscreenable: bool,
    pub always_on_top: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub skip_taskbar: bool,
    pub hidden_title: bool,
}

impl WindowInfo {
    pub fn main() -> Self {
        WindowInfo {
            label: "main".into(),
            title: "".into(),
            url: "/".into(),
            width: 1060.0,
            height: 600.0,
            resizable: true,
            maximized: false,
            fullscreenable: false,
            always_on_top: false,
            transparent: false,
            decorations: true,
            skip_taskbar: false,
            hidden_title: false,
        }
    }
    pub fn config() -> Self {
        WindowInfo {
            label: "config".into(),
            title: "".into(),
            url: "/config".into(),
            width: 1060.0,
            height: 600.0,
            resizable: true,
            maximized: false,
            // minimizable: false,
            fullscreenable: false,
            always_on_top: false,
            transparent: false,
            decorations: true,
            skip_taskbar: false,
            hidden_title: false,
        }
    }
}
