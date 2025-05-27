use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_notification::NotificationExt;
use tokio::time::sleep;
use std::fs;
use std::path::PathBuf;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    work_duration_minutes: u64,
    short_break_duration_minutes: u64,
    long_break_duration_minutes: u64,
    auto_start_breaks: bool,
    auto_start_work: bool,
    notification_sound: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            work_duration_minutes: 25,
            short_break_duration_minutes: 5,
            long_break_duration_minutes: 15,
            auto_start_breaks: false,
            auto_start_work: false,
            notification_sound: true,
        }
    }
}

#[derive(Debug)]
pub struct TimerManager {
    status: Arc<Mutex<TimerStatus>>,
    start_time: Arc<Mutex<Option<Instant>>>,
    preferences: Arc<Mutex<UserPreferences>>,
}

impl TimerManager {
    pub fn new() -> Self {
        let preferences = Self::load_preferences().unwrap_or_default();
        let work_duration = preferences.work_duration_minutes * 60;
        
        Self {
            status: Arc::new(Mutex::new(TimerStatus {
                state: TimerState::Idle,
                timer_type: TimerType::Work,
                remaining_seconds: work_duration,
                total_seconds: work_duration,
            })),
            start_time: Arc::new(Mutex::new(None)),
            preferences: Arc::new(Mutex::new(preferences)),
        }
    }

    fn get_preferences_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir()
            .ok_or("Could not get config directory")?;
        path.push("pomodoro-timer");
        fs::create_dir_all(&path)?;
        path.push("preferences.json");
        Ok(path)
    }

    fn load_preferences() -> Result<UserPreferences, Box<dyn std::error::Error>> {
        let path = Self::get_preferences_path()?;
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let preferences: UserPreferences = serde_json::from_str(&content)?;
            Ok(preferences)
        } else {
            Ok(UserPreferences::default())
        }
    }

    fn save_preferences(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_preferences_path()?;
        let preferences = self.preferences.lock().unwrap();
        let content = serde_json::to_string_pretty(&*preferences)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_preferences(&self) -> UserPreferences {
        self.preferences.lock().unwrap().clone()
    }

    pub fn update_preferences(&self, new_preferences: UserPreferences) -> Result<(), String> {
        {
            let mut preferences = self.preferences.lock().unwrap();
            *preferences = new_preferences;
        }
        
        // Reset timer if it's idle to use new durations
        {
            let mut status = self.status.lock().unwrap();
            if matches!(status.state, TimerState::Idle) {
                let prefs = self.preferences.lock().unwrap();
                let duration = match status.timer_type {
                    TimerType::Work => prefs.work_duration_minutes * 60,
                    TimerType::ShortBreak => prefs.short_break_duration_minutes * 60,
                    TimerType::LongBreak => prefs.long_break_duration_minutes * 60,
                };
                status.remaining_seconds = duration;
                status.total_seconds = duration;
            }
        }
        
        self.save_preferences().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_status(&self) -> TimerStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn start_timer(&self, app_handle: AppHandle, timer_type: TimerType) {
        let preferences = self.preferences.lock().unwrap();
        let duration_minutes = match timer_type {
            TimerType::Work => preferences.work_duration_minutes,
            TimerType::ShortBreak => preferences.short_break_duration_minutes,
            TimerType::LongBreak => preferences.long_break_duration_minutes,
        };
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
        let _start_time_clone = Arc::clone(&self.start_time);
        
        tauri::async_runtime::spawn(async move {
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
                    
                    let _ = app_handle
                        .notification()
                        .builder()
                        .title("Pomodoro Timer")
                        .body(&format!("{} completed!", timer_type_name))
                        .show();
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
        
        // Reset remaining_seconds to the full duration based on timer type and user preferences
        let preferences = self.preferences.lock().unwrap();
        let full_duration = match status.timer_type {
            TimerType::Work => preferences.work_duration_minutes * 60,
            TimerType::ShortBreak => preferences.short_break_duration_minutes * 60,
            TimerType::LongBreak => preferences.long_break_duration_minutes * 60,
        };
        status.remaining_seconds = full_duration;
        status.total_seconds = full_duration;
        
        *self.start_time.lock().unwrap() = None;
    }
}

#[tauri::command]
fn get_timer_status(timer_manager: State<TimerManager>) -> TimerStatus {
    timer_manager.get_status()
}

#[tauri::command]
fn start_work_timer(app_handle: AppHandle, timer_manager: State<TimerManager>) {
    timer_manager.start_timer(app_handle, TimerType::Work);
}

#[tauri::command]
fn start_short_break(app_handle: AppHandle, timer_manager: State<TimerManager>) {
    timer_manager.start_timer(app_handle, TimerType::ShortBreak);
}

#[tauri::command]
fn start_long_break(app_handle: AppHandle, timer_manager: State<TimerManager>) {
    timer_manager.start_timer(app_handle, TimerType::LongBreak);
}

#[tauri::command]
fn get_preferences(timer_manager: State<TimerManager>) -> UserPreferences {
    timer_manager.get_preferences()
}

#[tauri::command]
fn update_preferences(timer_manager: State<TimerManager>, preferences: UserPreferences) -> Result<(), String> {
    timer_manager.update_preferences(preferences)
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
            stop_timer,
            get_preferences,
            update_preferences
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
