use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerStatus {
    state: TimerState,
    timer_type: TimerType,
    remaining_seconds: u64,
    total_seconds: u64,
}

#[derive(Debug)]
pub struct TimerManager {
    status: Arc<Mutex<TimerStatus>>,
    start_time: Arc<Mutex<Option<Instant>>>,
}

impl TimerManager {
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(TimerStatus {
                state: TimerState::Idle,
                timer_type: TimerType::Work,
                remaining_seconds: 25 * 60,
                total_seconds: 25 * 60,
            })),
            start_time: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_status(&self) -> TimerStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn start_timer(&self, app_handle: AppHandle, timer_type: TimerType, duration_minutes: u64) {
        let duration_seconds = duration_minutes * 60;
        
        {
            let mut status = self.status.lock().unwrap();
            status.state = TimerState::Running;
            status.timer_type = timer_type.clone();
            status.remaining_seconds = duration_seconds;
            status.total_seconds = duration_seconds;
        }
        
        *self.start_time.lock().unwrap() = Some(Instant::now());
        
        let status_clone = Arc::clone(&self.status);
        let start_time_clone = Arc::clone(&self.start_time);
        
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                
                let mut should_complete = false;
                {
                    let mut status = status_clone.lock().unwrap();
                    if matches!(status.state, TimerState::Running) {
                        if status.remaining_seconds > 0 {
                            status.remaining_seconds -= 1;
                        } else {
                            status.state = TimerState::Completed;
                            should_complete = true;
                        }
                    }
                    
                    if !matches!(status.state, TimerState::Running) && !should_complete {
                        break;
                    }
                    
                    let _ = app_handle.emit("timer-tick", status.clone());
                }
                
                if should_complete {
                    let timer_type_name = match timer_type {
                        TimerType::Work => "Work session",
                        TimerType::ShortBreak => "Short break",
                        TimerType::LongBreak => "Long break",
                    };
                    
                    let _ = app_handle.emit("timer-completed", timer_type.clone());
                    
                    if let Ok(notification) = tauri_plugin_notification::NotificationBuilder::new()
                        .title("Pomodoro Timer")
                        .body(&format!("{} completed!", timer_type_name))
                        .build()
                    {
                        let _ = notification.show();
                    }
                    break;
                }
            }
        });
    }
    
    pub fn pause_timer(&self) {
        let mut status = self.status.lock().unwrap();
        if matches!(status.state, TimerState::Running) {
            status.state = TimerState::Paused;
        }
    }
    
    pub fn resume_timer(&self) {
        let mut status = self.status.lock().unwrap();
        if matches!(status.state, TimerState::Paused) {
            status.state = TimerState::Running;
        }
    }
    
    pub fn stop_timer(&self) {
        let mut status = self.status.lock().unwrap();
        status.state = TimerState::Idle;
        *self.start_time.lock().unwrap() = None;
    }
}

#[tauri::command]
fn get_timer_status(timer_manager: State<TimerManager>) -> TimerStatus {
    timer_manager.get_status()
}

#[tauri::command]
fn start_work_timer(app_handle: AppHandle, timer_manager: State<TimerManager>) {
    timer_manager.start_timer(app_handle, TimerType::Work, 25);
}

#[tauri::command]
fn start_short_break(app_handle: AppHandle, timer_manager: State<TimerManager>) {
    timer_manager.start_timer(app_handle, TimerType::ShortBreak, 5);
}

#[tauri::command]
fn start_long_break(app_handle: AppHandle, timer_manager: State<TimerManager>) {
    timer_manager.start_timer(app_handle, TimerType::LongBreak, 15);
}

#[tauri::command]
fn pause_timer(timer_manager: State<TimerManager>) {
    timer_manager.pause_timer();
}

#[tauri::command]
fn resume_timer(timer_manager: State<TimerManager>) {
    timer_manager.resume_timer();
}

#[tauri::command]
fn stop_timer(timer_manager: State<TimerManager>) {
    timer_manager.stop_timer();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let timer_manager = TimerManager::new();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .manage(timer_manager)
        .invoke_handler(tauri::generate_handler![
            get_timer_status,
            start_work_timer,
            start_short_break,
            start_long_break,
            pause_timer,
            resume_timer,
            stop_timer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
