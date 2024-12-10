mod cmd;

use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tauri::{
    webview::{PageLoadEvent, WebviewWindowBuilder},
    App, AppHandle, Emitter, Listener, RunEvent, WebviewUrl,
};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;
use std::fs;

#[derive(Serialize)]
struct CustomResponse {
    text: String,
}

#[derive(Deserialize)]
struct DiffParams {
    dirPath: String, // 사용자가 선택한 Git 리포지토리 경로
    outputFile: Option<String>, // diff를 저장할 파일명 (예: "diff.patch")
}

#[derive(Serialize)]
struct DiffResult {
    success: bool,
    diff_content: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
 struct ReviewParams {
     dirPath: String,
     outputFile: Option<String>,
    branchName: String,
}

#[derive(Serialize)]
 struct ReviewResult {
     success: bool,
     review: Option<String>,
     error: Option<String>,
}

#[tauri::command]
async fn infer_from_model(window: tauri::Window, prompt: String) -> Result<CustomResponse, String> {
    println!("Called from {}", window.label());
    let ollama = Ollama::default();
    let model = "qwen2.5-coder:latest".to_string();

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


#[tauri::command]
async fn generate_code_review(params: ReviewParams) -> Result<ReviewResult, String> {
    let repo_path = PathBuf::from(&params.dirPath);

    if !repo_path.exists() {
        return Ok(ReviewResult {
            success: false,
            review: None,
            error: Some("Repository path does not exist.".to_string()),
        });
    }

    // 1. Git Diff 얻기
    let diff_output = Command::new("git")
        .args(["diff", "main", &params.branchName])
        .current_dir(&repo_path)
        .output();

    let diff_str = match diff_output {
        Ok(output) => {
            if !output.status.success() {
                let err_str = String::from_utf8_lossy(&output.stderr).to_string();
                return Ok(ReviewResult {
                    success: false,
                    review: None,
                    error: Some(format!("Failed to run git diff: {}", err_str)),
                });
            }
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Err(e) => {
            return Ok(ReviewResult {
                success: false,
                review: None,
                error: Some(format!("Error executing git command: {}", e)),
            });
        }
    };

    // option: export diff_output as diff.txt file.
    fs::write("diff.txt", &diff_str).expect("Unable to write file");

    // diff가 비어있으면 리뷰할 변경 사항이 없는 것.
    if diff_str.trim().is_empty() {
        return Ok(ReviewResult {
            success: true,
            review: Some("No changes found between main and HEAD.".to_string()),
            error: None,
        });
    }

    // 파일로 저장 (선택사항)
    if let Some(file_name) = &params.outputFile {
        let output_path = repo_path.join(file_name);
        if let Err(e) = fs::write(&output_path, &diff_str) {
            return Ok(ReviewResult {
                success: false,
                review: None,
                error: Some(format!("Failed to write diff to file: {}", e)),
            });
        }
    }

    // 2. Ollama 모델에 Diff 전달하여 코드 리뷰 생성
    let ollama = Ollama::default();
    // 프롬프트 템플릿에 diff 삽입
    const prompt_template: &str = "Please answer in Korean only. 아래에 코드 변경사항이 있어. 프론트엔드 팀원으로서 여기에 대해 코드리뷰를 제안해줘. :\n{{diff}}";
    let prompt = prompt_template.replace("{{diff}}", &diff_str);

    let model: String = "qwen2.5-coder:latest".to_string();

    let res= ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(response) = res {
        Ok(ReviewResult {
            success: true,
            review: Some(response.response),
            error: None,
        })
    } else {
        Ok(ReviewResult {
            success: false,
            review: None,
            error: Some("Error".to_string()),
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![generate_code_review, infer_from_model])
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
