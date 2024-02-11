use crate::{
  conf::AppConf,
  utils::{self, load_script},
};
use tauri::{utils::config::WindowUrl, window::WindowBuilder, Manager};

pub fn tray_window(handle: &tauri::AppHandle) {
  let app_conf = AppConf::read();
  let theme = AppConf::theme_mode();
  let app = handle.clone();

  tauri::async_runtime::spawn(async move {
    let link = if app_conf.tray_dashboard {
      "index.html"
    } else {
      &app_conf.tray_origin
    };
    let mut tray_win = WindowBuilder::new(&app, "tray", WindowUrl::App(link.into()))
      .title("ChatGPT")
      .resizable(false)
      .fullscreen(false)
      .inner_size(app_conf.tray_width, app_conf.tray_height)
      .decorations(false)
      .always_on_top(true)
      .theme(Some(theme))
      .initialization_script(&utils::user_script())
      .initialization_script(&load_script("core.js"))
      .user_agent(&app_conf.ua_tray);

    if app_conf.tray_origin == "https://chat.openai.com" && !app_conf.tray_dashboard {
      tray_win = tray_win
        .initialization_script(include_str!("../vendors/floating-ui-core.js"))
        .initialization_script(include_str!("../vendors/floating-ui-dom.js"))
        .initialization_script(&load_script("cmd.js"))
        .initialization_script(&load_script("chat.js"))
    }

    tray_win.build().unwrap().hide().unwrap();
  });
}

pub fn sponsor_window(handle: tauri::AppHandle) {
  tauri::async_runtime::spawn(async move {
    if let Some(win) = handle.get_window("sponsor") {
      win.show().unwrap()
    } else {
      WindowBuilder::new(&handle, "sponsor", WindowUrl::App("sponsor.html".into()))
        .title("Sponsor")
        .resizable(true)
        .fullscreen(false)
        .inner_size(600.0, 600.0)
        .min_inner_size(600.0, 600.0)
        .build()
        .unwrap();
    }
  });
}

pub mod cmd {
  use super::*;
  use log::info;
  use tauri::{command, utils::config::WindowUrl, window::WindowBuilder, Manager};

  #[tauri::command]
  pub fn control_window(handle: tauri::AppHandle, win_type: String) {
    tauri::async_runtime::spawn(async move {
      if handle.get_window("main").is_none() {
        WindowBuilder::new(
          &handle,
          "main",
          WindowUrl::App(format!("index.html?type={}", win_type).into()),
        )
        .title("Control Center")
        .resizable(true)
        .fullscreen(false)
        .inner_size(1200.0, 700.0)
        .min_inner_size(1000.0, 600.0)
        .build()
        .unwrap();
      } else {
        let main_win = handle.get_window("main").unwrap();
        main_win.show().unwrap();
        main_win.set_focus().unwrap();
      }
    });
  }

  #[command]
  pub fn wa_window(
    app: tauri::AppHandle,
    label: String,
    title: String,
    url: String,
    script: Option<String>,
  ) {
    info!("wa_window: {} :=> {}", title, url);
    let win = app.get_window(&label);
    if win.is_none() {
      tauri::async_runtime::spawn(async move {
        tauri::WindowBuilder::new(&app, label, tauri::WindowUrl::App(url.parse().unwrap()))
          .initialization_script(&script.unwrap_or_default())
          .initialization_script(&load_script("core.js"))
          .title(title)
          .inner_size(960.0, 700.0)
          .resizable(true)
          .build()
          .unwrap();
      });
    } else if let Some(v) = win {
      if !v.is_visible().unwrap() {
        v.show().unwrap();
      }
      v.eval("window.location.reload()").unwrap();
      v.set_focus().unwrap();
    }
  }

  #[command]
  pub fn window_reload(app: tauri::AppHandle, label: &str) {
    app
      .app_handle()
      .get_window(label)
      .unwrap()
      .eval("window.location.reload()")
      .unwrap();
  }
}
