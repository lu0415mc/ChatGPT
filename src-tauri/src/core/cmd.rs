use tauri::{command, AppHandle, LogicalPosition, Manager, PhysicalSize};

use crate::core::{
    conf::AppConf,
    constant::{ASK_HEIGHT, TITLEBAR_HEIGHT},
};

#[command]
pub fn view_reload(app: AppHandle) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval("window.location.reload()")
        .unwrap();
}

#[command]
pub fn view_url(app: AppHandle) -> tauri::Url {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .url()
}

#[command]
pub fn view_go_forward(app: AppHandle) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval("window.history.forward()")
        .unwrap();
}

#[command]
pub fn view_go_back(app: AppHandle) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval("window.history.back()")
        .unwrap();
}

#[command]
pub fn window_pin(app: AppHandle, pin: bool) {
    let conf = AppConf::load(&app).unwrap();
    conf.amend(serde_json::json!({"stay_on_top": pin}))
        .unwrap()
        .save(&app)
        .unwrap();

    app.get_window("core")
        .unwrap()
        .set_always_on_top(pin)
        .unwrap();
}

#[command]
pub fn ask_sync(app: AppHandle, message: String) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval(&format!("ChatAsk.sync({})", message))
        .unwrap();
}

#[command]
pub fn ask_send(app: AppHandle) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval("ChatAsk.submit()")
        .unwrap();
}

#[command]
pub fn set_theme(app: AppHandle, theme: String) {
    let conf = AppConf::load(&app).unwrap();
    conf.amend(serde_json::json!({"theme": theme}))
        .unwrap()
        .save(&app)
        .unwrap();

    app.restart();
}

#[command]
pub fn get_app_conf(app: AppHandle) -> AppConf {
    AppConf::load(&app).unwrap()
}

#[command]
pub fn set_view_ask(app: AppHandle, enabled: bool) {
    let conf = AppConf::load(&app).unwrap();
    conf.amend(serde_json::json!({"ask_mode": enabled}))
        .unwrap()
        .save(&app)
        .unwrap();

    let core_window = app.get_window("core").unwrap();
    let ask_mode_height = if enabled { ASK_HEIGHT } else { 0.0 };
    let scale_factor = core_window.scale_factor().unwrap();
    let titlebar_height = (scale_factor * TITLEBAR_HEIGHT).round() as u32;
    let ask_height = (scale_factor * ask_mode_height).round() as u32;
    let win_size = core_window.inner_size().expect("Failed to get window size");
    let main_area_height = core_window.inner_size().unwrap().height - titlebar_height;

    let main_view = core_window
        .get_webview("main")
        .expect("[view:main] Failed to get webview window");
    let titlebar_view = core_window
        .get_webview("titlebar")
        .expect("[view:titlebar] Failed to get webview window");
    let ask_view = core_window
        .get_webview("ask")
        .expect("[view:ask] Failed to get webview window");

    if enabled {
        ask_view.set_focus().unwrap();
    } else {
        main_view.set_focus().unwrap();
    }

    main_view
        .set_position(LogicalPosition::new(0.0, TITLEBAR_HEIGHT))
        .unwrap();
    main_view
        .set_size(PhysicalSize::new(
            core_window.inner_size().unwrap().width,
            main_area_height - ask_height,
        ))
        .unwrap();

    titlebar_view
        .set_position(LogicalPosition::new(0, 0))
        .unwrap();
    titlebar_view
        .set_size(PhysicalSize::new(
            core_window.inner_size().unwrap().width,
            titlebar_height,
        ))
        .unwrap();

    ask_view
        .set_position(LogicalPosition::new(
            0.0,
            (win_size.height as f64 / scale_factor) - ask_mode_height,
        ))
        .unwrap();
    ask_view
        .set_size(PhysicalSize::new(
            core_window.inner_size().unwrap().width,
            ask_height,
        ))
        .unwrap();
}
