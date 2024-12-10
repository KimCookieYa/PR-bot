use ollama_rs::{
    generation::completion::{
        request::GenerationRequest,
    },
    Ollama,
};

#[derive(serde::Serialize)]
struct CustomResponse {
    text: String,
}

#[tauri::command]
async fn infer_from_model(
  window: tauri::Window, prompt: String) -> Result<CustomResponse, String> {
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
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![infer_from_model])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
