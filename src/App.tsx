import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface TimerStatus {
  state: "Idle" | "Running" | "Paused" | "Completed";
  timer_type: "Work" | "ShortBreak" | "LongBreak";
  remaining_seconds: number;
  total_seconds: number;
}

function App() {
  const [timerStatus, setTimerStatus] = createSignal<TimerStatus>({
    state: "Idle",
    timer_type: "Work",
    remaining_seconds: 25 * 60,
    total_seconds: 25 * 60,
  });

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  };

  const getProgress = (): number => {
    const status = timerStatus();
    return (
      ((status.total_seconds - status.remaining_seconds) /
        status.total_seconds) *
      100
    );
  };

  const updateStatus = async () => {
    try {
      const status = await invoke<TimerStatus>("get_timer_status");
      setTimerStatus(status);
    } catch (error) {
      console.error("Failed to get timer status:", error);
    }
  };

  const startWorkTimer = async () => {
    try {
      await invoke("start_work_timer");
      await updateStatus();
    } catch (error) {
      console.error("Failed to start work timer:", error);
    }
  };

  const startShortBreak = async () => {
    try {
      await invoke("start_short_break");
      await updateStatus();
    } catch (error) {
      console.error("Failed to start short break:", error);
    }
  };

  const startLongBreak = async () => {
    try {
      await invoke("start_long_break");
      await updateStatus();
    } catch (error) {
      console.error("Failed to start long break:", error);
    }
  };

  const pauseTimer = async () => {
    try {
      await invoke("pause_timer");
      await updateStatus();
    } catch (error) {
      console.error("Failed to pause timer:", error);
    }
  };

  const resumeTimer = async () => {
    try {
      await invoke("resume_timer");
      await updateStatus();
    } catch (error) {
      console.error("Failed to resume timer:", error);
    }
  };

  const stopTimer = async () => {
    try {
      await invoke("stop_timer");
      await updateStatus();
    } catch (error) {
      console.error("Failed to stop timer:", error);
    }
  };

  onMount(async () => {
    await updateStatus();

    await listen("timer-tick", (event) => {
      setTimerStatus(event.payload as TimerStatus);
    });

    await listen("timer-completed", () => {
      updateStatus();
    });
  });

  return (
    <main class="container">
      <h1>Pomodoro Timer</h1>

      <div class="timer-display">
        <div class="timer-type">
          {timerStatus().timer_type === "Work" && "Work Session"}
          {timerStatus().timer_type === "ShortBreak" && "Short Break"}
          {timerStatus().timer_type === "LongBreak" && "Long Break"}
        </div>

        <div class="timer-circle">
          <svg class="progress-ring" width="200" height="200">
            <circle
              class="progress-ring-circle-bg"
              stroke="#e0e0e0"
              stroke-width="8"
              fill="transparent"
              r="88"
              cx="100"
              cy="100"
            />
            <circle
              class="progress-ring-circle"
              stroke={
                timerStatus().timer_type === "Work" ? "#ff6b6b" : "#4ecdc4"
              }
              stroke-width="8"
              fill="transparent"
              r="88"
              cx="100"
              cy="100"
              stroke-dasharray={`${2 * Math.PI * 88}`}
              stroke-dashoffset={`${2 * Math.PI * 88 * (1 - getProgress() / 100)}`}
              stroke-linecap="round"
              transform="rotate(-90 100 100)"
            />
          </svg>
          <div class="timer-time">
            {formatTime(timerStatus().remaining_seconds)}
          </div>
        </div>

        <div class="timer-state">Status: {timerStatus().state}</div>
      </div>

      <div class="controls">
        <div class="timer-buttons">
          <button
            onClick={startWorkTimer}
            disabled={timerStatus().state === "Running"}
            class="work-btn"
          >
            Work (25m)
          </button>
          <button
            onClick={startShortBreak}
            disabled={timerStatus().state === "Running"}
            class="break-btn"
          >
            Short Break (5m)
          </button>
          <button
            onClick={startLongBreak}
            disabled={timerStatus().state === "Running"}
            class="break-btn"
          >
            Long Break (15m)
          </button>
        </div>

        <div class="control-buttons">
          {timerStatus().state === "Running" && (
            <button onClick={pauseTimer} class="pause-btn">
              Pause
            </button>
          )}
          {timerStatus().state === "Paused" && (
            <button onClick={resumeTimer} class="resume-btn">
              Resume
            </button>
          )}
          {(timerStatus().state === "Running" ||
            timerStatus().state === "Paused") && (
            <button onClick={stopTimer} class="stop-btn">
              Stop
            </button>
          )}
        </div>
      </div>
    </main>
  );
}

export default App;
