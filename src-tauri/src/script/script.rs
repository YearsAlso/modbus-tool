//! Script definition and serialization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Action, Trigger};

/// Automation script definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Script {
    /// Unique identifier
    pub id: Uuid,
    
    /// Display name
    pub name: String,
    
    /// Optional description
    pub description: String,
    
    /// Trigger condition
    pub trigger: Trigger,
    
    /// Actions to execute when triggered
    pub actions: Vec<Action>,
    
    /// Whether the script is enabled
    pub enabled: bool,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Script {
    /// Create a new script with the given trigger
    pub fn new(name: String, trigger: Trigger) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            trigger,
            actions: Vec::new(),
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Add an action to the script
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
        self.updated_at = Utc::now();
    }
    
    /// Remove an action by index
    pub fn remove_action(&mut self, index: usize) -> Option<Action> {
        if index < self.actions.len() {
            self.updated_at = Utc::now();
            Some(self.actions.remove(index))
        } else {
            None
        }
    }
    
    /// Set the script description
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
        self.updated_at = Utc::now();
    }
    
    /// Enable or disable the script
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.updated_at = Utc::now();
    }
    
    /// Check if the script is valid (has at least one action)
    pub fn is_valid(&self) -> bool {
        !self.actions.is_empty()
    }
}

impl Default for Script {
    fn default() -> Self {
        Self::new("New Script".to_string(), Trigger::Changed {
            register: "40001".to_string(),
        })
    }
}

/// Script execution status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptStatus {
    pub script_id: Uuid,
    pub running: bool,
    pub last_triggered: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

impl ScriptStatus {
    pub fn new(script_id: Uuid) -> Self {
        Self {
            script_id,
            running: false,
            last_triggered: None,
            last_error: None,
        }
    }
    
    pub fn mark_triggered(&mut self) {
        self.last_triggered = Some(Utc::now());
        self.last_error = None;
    }
    
    pub fn mark_error(&mut self, error: impl Into<String>) {
        self.last_error = Some(error.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_script_new() {
        let trigger = Trigger::Changed {
            register: "40001".to_string(),
        };
        let script = Script::new("Test Script".to_string(), trigger.clone());
        
        assert_eq!(script.name, "Test Script");
        assert_eq!(script.trigger, trigger);
        assert!(script.enabled);
        assert!(script.actions.is_empty());
    }
    
    #[test]
    fn test_script_add_action() {
        let mut script = Script::default();
        script.add_action(Action::WriteOn {
            register: "00001".to_string(),
        });
        
        assert_eq!(script.actions.len(), 1);
        assert!(script.is_valid());
    }
    
    #[test]
    fn test_script_remove_action() {
        let mut script = Script::default();
        script.add_action(Action::WriteOn {
            register: "00001".to_string(),
        });
        
        let action = script.remove_action(0);
        assert!(action.is_some());
        assert!(script.actions.is_empty());
        assert!(!script.is_valid());
    }
}
