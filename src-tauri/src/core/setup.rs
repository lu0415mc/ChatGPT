use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{
    webview::DownloadEvent, App, LogicalPosition, Manager, PhysicalSize, TitleBarStyle,
    WebviewBuilder, WebviewUrl, WindowBuilder, WindowEvent,
};
use tauri_plugin_shell::ShellExt;

use crate::core::{
    conf::AppConf,
    constant::{ASK_HEIGHT, INIT_SCRIPT, TITLEBAR_HEIGHT},
    template,
};

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    let conf = &AppConf::load(handle).unwrap();
    let ask_mode_height = if conf.ask_mode { ASK_HEIGHT } else { 0.0 };

    template::Template::new(AppConf::get_scripts_path(handle)?);

    tauri::async_runtime::spawn({
        let handle = handle.clone();
        async move {
            let core_window = WindowBuilder::new(&handle, "core")
                .hidden_title(true)
                .title_bar_style(TitleBarStyle::Overlay)
                .resizable(true)
                .inner_size(800.0, 600.0)
                .min_inner_size(300.0, 200.0)
                .theme(Some(AppConf::get_theme(&handle)))
                .build()
                .expect("Failed to create window");

            let win_size = core_window.inner_size().expect("Failed to get window size");
            let window = Arc::new(Mutex::new(core_window)); // Wrap the window in Arc<Mutex<_>> to manage ownership across threads

            let main_view =
                WebviewBuilder::new("main", WebviewUrl::App("https://chatgpt.com".into()))
                    .auto_resize()
                    .on_download({
                        let app_handle = handle.clone();
                        let download_path = Arc::new(Mutex::new(PathBuf::new()));
                        move |_, event| {
                            match event {
                                DownloadEvent::Requested { destination, .. } => {
                                    let download_dir = app_handle
                                        .path()
                                        .download_dir()
                                        .expect("[download] Failed to get download directory");
                                    let mut locked_path = download_path
                                        .lock()
                                        .expect("[download] Failed to lock download path");
                                    *locked_path = download_dir.join(&destination);
                                    *destination = locked_path.clone();
                                }
                                DownloadEvent::Finished { success, .. } => {
                                    let final_path = download_path
                                        .lock()
                                        .expect("[download] Failed to lock download path")
                                        .clone();

                                    if success {
                                        app_handle
                                            .shell()
                                            .open(final_path.to_string_lossy(), None)
                                            .expect("[download] Failed to open file");
                                    }
                                }
                                _ => (),
                            }
                            true
                        }
                    })
                    .initialization_script("console.log('Hello from the initialization script!');")
                    .initialization_script(&AppConf::load_script(&handle, "ask.js"))
                    .initialization_script(INIT_SCRIPT);

            let titlebar_view = WebviewBuilder::new(
                "titlebar",
                WebviewUrl::App("index.html?type=titlebar".into()),
            )
            .auto_resize();

            let ask_view =
                WebviewBuilder::new("ask", WebviewUrl::App("index.html?type=ask".into()))
                    .auto_resize();

            let window_clone = Arc::clone(&window);
            let win = window.lock().unwrap();
            let scale_factor = win.scale_factor().unwrap();
            let titlebar_height = (scale_factor * TITLEBAR_HEIGHT).round() as u32;
            let ask_height = (scale_factor * ask_mode_height).round() as u32;
            let main_area_height = win_size.height - titlebar_height;

            win.add_child(
                titlebar_view,
                LogicalPosition::new(0, 0),
                PhysicalSize::new(win_size.width, titlebar_height),
            )
            .unwrap();
            win.add_child(
                ask_view,
                LogicalPosition::new(
                    0.0,
                    (win_size.height as f64 / scale_factor) - ask_mode_height,
                ),
                PhysicalSize::new(win_size.width, ask_height),
            )
            .unwrap();
            win.add_child(
                main_view,
                LogicalPosition::new(0.0, TITLEBAR_HEIGHT),
                PhysicalSize::new(win_size.width, main_area_height - ask_height),
            )
            .unwrap();

            // {
            //     let get_ask_view = win
            //         .get_webview("ask")
            //         .expect("Failed to get webview window");
            //     get_ask_view.open_devtools();
            // }
            // {
            //     let get_main_view = win
            //         .get_webview("main")
            //         .expect("Failed to get webview window");
            //     // dbg!(get_main_view);
            //     // get_main_view.open_devtools();
            // }

            win.on_window_event(move |event| {
                let conf = &AppConf::load(&handle).unwrap();
                let ask_mode_height = if conf.ask_mode { ASK_HEIGHT } else { 0.0 };
                let ask_height = (scale_factor * ask_mode_height).round() as u32;

                if let WindowEvent::Resized(size) = event {
                    let win = window_clone.lock().unwrap();
                    let main_area_height = win.inner_size().unwrap().height - titlebar_height;

                    let main_view = win
                        .get_webview("main")
                        .expect("[view:main] Failed to get webview window");
                    let titlebar_view = win
                        .get_webview("titlebar")
                        .expect("[view:titlebar] Failed to get webview window");
                    let ask_view = win
                        .get_webview("ask")
                        .expect("[view:ask] Failed to get webview window");

                    main_view
                        .set_position(LogicalPosition::new(0.0, TITLEBAR_HEIGHT))
                        .unwrap();
                    main_view
                        .set_size(PhysicalSize::new(
                            win.inner_size().unwrap().width,
                            main_area_height - ask_height,
                        ))
                        .unwrap();

                    titlebar_view
                        .set_position(LogicalPosition::new(0, 0))
                        .unwrap();
                    titlebar_view
                        .set_size(PhysicalSize::new(
                            win.inner_size().unwrap().width,
                            titlebar_height,
                        ))
                        .unwrap();

                    ask_view
                        .set_position(LogicalPosition::new(
                            0.0,
                            (size.height as f64 / scale_factor) - ask_mode_height,
                        ))
                        .unwrap();
                    ask_view
                        .set_size(PhysicalSize::new(
                            win.inner_size().unwrap().width,
                            ask_height,
                        ))
                        .unwrap();
                }
            });
        }
    });

    Ok(())
}
