import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./Settings.css";

export interface UserPreferences {
  work_duration_minutes: number;
  short_break_duration_minutes: number;
  long_break_duration_minutes: number;
  auto_start_breaks: boolean;
  auto_start_work: boolean;
  notification_sound: boolean;
}

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  onPreferencesUpdated: () => void;
}

export default function Settings(props: SettingsProps) {
  const [preferences, setPreferences] = createSignal<UserPreferences>({
    work_duration_minutes: 25,
    short_break_duration_minutes: 5,
    long_break_duration_minutes: 15,
    auto_start_breaks: false,
    auto_start_work: false,
    notification_sound: true,
  });

  const [isSaving, setIsSaving] = createSignal(false);

  const loadPreferences = async () => {
    try {
      const prefs = await invoke<UserPreferences>("get_preferences");
      setPreferences(prefs);
    } catch (error) {
      console.error("Failed to load preferences:", error);
    }
  };

  const savePreferences = async () => {
    setIsSaving(true);
    try {
      await invoke("update_preferences", { preferences: preferences() });
      props.onPreferencesUpdated();
      props.onClose();
    } catch (error) {
      console.error("Failed to save preferences:", error);
    } finally {
      setIsSaving(false);
    }
  };

  const updatePreference = <K extends keyof UserPreferences>(
    key: K,
    value: UserPreferences[K]
  ) => {
    setPreferences((prev) => ({ ...prev, [key]: value }));
  };

  const resetToDefaults = () => {
    setPreferences({
      work_duration_minutes: 25,
      short_break_duration_minutes: 5,
      long_break_duration_minutes: 15,
      auto_start_breaks: false,
      auto_start_work: false,
      notification_sound: true,
    });
  };

  onMount(() => {
    if (props.isOpen) {
      loadPreferences();
    }
  });

  // Load preferences when modal opens
  const handleModalOpen = () => {
    if (props.isOpen) {
      loadPreferences();
    }
  };

  // Watch for isOpen changes
  onMount(() => {
    handleModalOpen();
  });

  if (!props.isOpen) return null;

  return (
    <div class="settings-overlay" onClick={props.onClose}>
      <div class="settings-modal" onClick={(e) => e.stopPropagation()}>
        <div class="settings-header">
          <h2>Settings</h2>
          <button class="close-btn" onClick={props.onClose}>
            ×
          </button>
        </div>

        <div class="settings-content">
          <div class="settings-section">
            <h3>Timer Durations</h3>
            
            <div class="setting-item">
              <label for="work-duration">Work Session (minutes)</label>
              <input
                id="work-duration"
                type="number"
                min="1"
                max="60"
                value={preferences().work_duration_minutes}
                onInput={(e) =>
                  updatePreference("work_duration_minutes", parseInt(e.currentTarget.value) || 25)
                }
              />
            </div>

            <div class="setting-item">
              <label for="short-break-duration">Short Break (minutes)</label>
              <input
                id="short-break-duration"
                type="number"
                min="1"
                max="30"
                value={preferences().short_break_duration_minutes}
                onInput={(e) =>
                  updatePreference("short_break_duration_minutes", parseInt(e.currentTarget.value) || 5)
                }
              />
            </div>

            <div class="setting-item">
              <label for="long-break-duration">Long Break (minutes)</label>
              <input
                id="long-break-duration"
                type="number"
                min="1"
                max="60"
                value={preferences().long_break_duration_minutes}
                onInput={(e) =>
                  updatePreference("long_break_duration_minutes", parseInt(e.currentTarget.value) || 15)
                }
              />
            </div>
          </div>

          <div class="settings-section">
            <h3>Auto-Start Options</h3>
            
            <div class="setting-item">
              <label class="checkbox-label">
                <input
                  type="checkbox"
                  checked={preferences().auto_start_breaks}
                  onChange={(e) =>
                    updatePreference("auto_start_breaks", e.currentTarget.checked)
                  }
                />
                <span class="checkmark"></span>
                Auto-start breaks after work sessions
              </label>
            </div>

            <div class="setting-item">
              <label class="checkbox-label">
                <input
                  type="checkbox"
                  checked={preferences().auto_start_work}
                  onChange={(e) =>
                    updatePreference("auto_start_work", e.currentTarget.checked)
                  }
                />
                <span class="checkmark"></span>
                Auto-start work sessions after breaks
              </label>
            </div>
          </div>

          <div class="settings-section">
            <h3>Notifications</h3>
            
            <div class="setting-item">
              <label class="checkbox-label">
                <input
                  type="checkbox"
                  checked={preferences().notification_sound}
                  onChange={(e) =>
                    updatePreference("notification_sound", e.currentTarget.checked)
                  }
                />
                <span class="checkmark"></span>
                Enable notification sounds
              </label>
            </div>
          </div>
        </div>

        <div class="settings-footer">
          <button class="reset-btn" onClick={resetToDefaults}>
            Reset to Defaults
          </button>
          <div class="action-buttons">
            <button class="cancel-btn" onClick={props.onClose}>
              Cancel
            </button>
            <button
              class="save-btn"
              onClick={savePreferences}
              disabled={isSaving()}
            >
              {isSaving() ? "Saving..." : "Save"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}