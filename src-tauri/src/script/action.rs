//! Action definitions for automation scripts

use serde::{Deserialize, Serialize};

/// Sound types for notification sounds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SoundType {
    Alert,
    Success,
    Warning,
}

/// Actions that can be executed when a trigger fires
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// Write a specific value to a register
    WriteValue {
        register: String,
        value: u16,
    },
    
    /// Write 0xFF00 (ON) to a coil/register
    WriteOn {
        register: String,
    },
    
    /// Write 0x0000 (OFF) to a coil/register
    WriteOff {
        register: String,
    },
    
    /// Toggle a coil (ON->OFF or OFF->ON)
    Toggle {
        register: String,
    },
    
    /// Show a desktop notification
    ShowNotification {
        title: String,
        message: String,
    },
    
    /// Play a sound
    PlaySound {
        sound: SoundType,
    },
    
    /// Log a message
    Log {
        message: String,
    },
    
    /// Run another script by ID
    RunScript {
        script_id: String,
    },
    
    /// Stop a running script by ID
    StopScript {
        script_id: String,
    },
    
    /// Delay execution for N seconds
    Delay {
        seconds: u64,
    },
}

impl Action {
    /// Human-readable description of the action
    pub fn description(&self) -> String {
        match self {
            Action::WriteValue { register, value } => {
                format!("Write {} to {}", value, register)
            }
            Action::WriteOn { register } => {
                format!("Turn ON {}", register)
            }
            Action::WriteOff { register } => {
                format!("Turn OFF {}", register)
            }
            Action::Toggle { register } => {
                format!("Toggle {}", register)
            }
            Action::ShowNotification { title, message } => {
                format!("Notify: {} - {}", title, message)
            }
            Action::PlaySound { sound } => {
                format!("Play sound: {:?}", sound)
            }
            Action::Log { message } => {
                format!("Log: {}", message)
            }
            Action::RunScript { script_id } => {
                format!("Run script: {}", script_id)
            }
            Action::StopScript { script_id } => {
                format!("Stop script: {}", script_id)
            }
            Action::Delay { seconds } => {
                format!("Wait {}s", seconds)
            }
        }
    }
    
    /// Check if this action requires a target register
    pub fn has_register(&self) -> bool {
        matches!(
            self,
            Action::WriteValue { .. }
                | Action::WriteOn { .. }
                | Action::WriteOff { .. }
                | Action::Toggle { .. }
        )
    }
    
    /// Get the target register if any
    pub fn register(&self) -> Option<&str> {
        match self {
            Action::WriteValue { register, .. } => Some(register),
            Action::WriteOn { register } => Some(register),
            Action::WriteOff { register } => Some(register),
            Action::Toggle { register } => Some(register),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_action_description() {
        let a1 = Action::WriteValue {
            register: "40001".to_string(),
            value: 100,
        };
        assert_eq!(a1.description(), "Write 100 to 40001");
        
        let a2 = Action::ShowNotification {
            title: "Alert".to_string(),
            message: "Temperature high!".to_string(),
        };
        assert_eq!(a2.description(), "Notify: Alert - Temperature high!");
    }
    
    #[test]
    fn test_action_register() {
        let a1 = Action::WriteOn { register: "00001".to_string() };
        assert_eq!(a1.register(), Some("00001"));
        
        let a2 = Action::Log { message: "Test".to_string() };
        assert_eq!(a2.register(), None);
    }
}
