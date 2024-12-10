mod cmd;

use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tauri::{
    webview::{PageLoadEvent, WebviewWindowBuilder},
    App, AppHandle, Emitter, Listener, RunEvent, WebviewUrl,
};

#[derive(serde::Serialize)]
struct CustomResponse {
    text: String,
}

#[tauri::command]
async fn infer_from_model(window: tauri::Window, prompt: String) -> Result<CustomResponse, String> {
    println!("Called from {}", window.label());
    let ollama: Ollama = Ollama::default();
    let model: String = "llama3.2:latest".to_string();

    println!("I was invoked from JavaScript! {}", prompt);

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    println!("Response: {:?}", res);

    if let Ok(response) = res {
        Ok(CustomResponse {
            text: response.response,
        })
    } else {
        Err("Error".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![infer_from_model])
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {

            Ok(())
        });

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(tauri::menu::Menu::default);
    }

    #[allow(unused_mut)]
    let mut app = builder
        .invoke_handler(tauri::generate_handler![
            cmd::log_operation,
            cmd::perform_request,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Regular);

    app.run(move |_app_handle, _event| {
        #[cfg(desktop)]
        if let RunEvent::ExitRequested { code, api, .. } = &_event {
            if code.is_none() {
                // Keep the event loop running even if all windows are closed
                // This allow us to catch system tray events when there is no window
                api.prevent_exit();
            }
        }
    })
}
