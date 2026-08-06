#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::Arc;
use loglens_core::models::{EventGroup, LogEvent, LogSource, QueryFilter};
use loglens_core::StreamingLogReader;
use loglens_storage::{Database, EventRepository, GroupRepository, SourceRepository};
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

struct AppState {
    db: Database,
}

#[tauri::command]
async fn list_sources(state: State<'_, Arc<Mutex<AppState>>>) -> Result<Vec<LogSource>, String> {
    let state_guard = state.lock().await;
    let repo = SourceRepository::new(state_guard.db.pool());
    repo.list_sources(Uuid::nil()).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn import_file(path: String, state: State<'_, Arc<Mutex<AppState>>>) -> Result<LogSource, String> {
    let state_guard = state.lock().await;
    let file_path = PathBuf::from(path);
    let src_id = Uuid::new_v4();

    let reader = StreamingLogReader::new(Default::default());
    let (source, events) = reader
        .process_file(&file_path, Uuid::nil(), src_id, None)
        .await
        .map_err(|e| e.to_string())?;

    let src_repo = SourceRepository::new(state_guard.db.pool());
    src_repo.create_source(&source).await.map_err(|e| e.to_string())?;

    let event_repo = EventRepository::new(state_guard.db.pool());
    event_repo.insert_events_batch(&events).await.map_err(|e| e.to_string())?;

    Ok(source)
}

#[tauri::command]
async fn delete_source(source_id: String, state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
    let state_guard = state.lock().await;
    let id = Uuid::parse_str(&source_id).map_err(|e| e.to_string())?;
    let repo = SourceRepository::new(state_guard.db.pool());
    repo.delete_source(id, Uuid::nil()).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn query_events(filter: QueryFilter, state: State<'_, Arc<Mutex<AppState>>>) -> Result<Vec<LogEvent>, String> {
    let state_guard = state.lock().await;
    let repo = EventRepository::new(state_guard.db.pool());
    repo.query_events(Uuid::nil(), &filter).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_groups(state: State<'_, Arc<Mutex<AppState>>>) -> Result<Vec<EventGroup>, String> {
    let state_guard = state.lock().await;
    let repo = GroupRepository::new(state_guard.db.pool());
    repo.list_groups(Uuid::nil()).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn export_events(_format: String, _query: Option<String>, state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    let state_guard = state.lock().await;
    let repo = EventRepository::new(state_guard.db.pool());
    let events = repo.query_events(Uuid::nil(), &Default::default()).await.map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&events).map_err(|e| e.to_string())
}

fn main() {
    let db_path = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("LogLens")
        .join("loglens_desktop.db");

    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    let rt = tokio::runtime::Runtime::new().unwrap();
    let db = rt.block_on(async { Database::new_sqlite(&db_url).await.unwrap() });

    let app_state = Arc::new(Mutex::new(AppState { db }));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            list_sources,
            import_file,
            delete_source,
            query_events,
            list_groups,
            export_events
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
