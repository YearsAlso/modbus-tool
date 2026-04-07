//! Script execution engine

use std::collections::HashMap;
use uuid::Uuid;

use super::trigger::CompareOp;
use super::{Action, Script, ScriptStatus, Trigger};

pub struct ScriptEngine {
    scripts: HashMap<Uuid, Script>,
    statuses: HashMap<Uuid, ScriptStatus>,
    last_values: HashMap<String, u16>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        Self {
            scripts: HashMap::new(),
            statuses: HashMap::new(),
            last_values: HashMap::new(),
        }
    }
    
    pub fn add_script(&mut self, script: Script) {
        let id = script.id;
        self.scripts.insert(id, script);
        self.statuses.insert(id, ScriptStatus::new(id));
    }
    
    pub fn remove_script(&mut self, id: &Uuid) -> Option<Script> {
        self.statuses.remove(id);
        self.scripts.remove(id)
    }
    
    pub fn get_script(&self, id: &Uuid) -> Option<&Script> {
        self.scripts.get(id)
    }
    
    pub fn get_script_mut(&mut self, id: &Uuid) -> Option<&mut Script> {
        self.scripts.get_mut(id)
    }
    
    pub fn get_all_scripts(&self) -> Vec<&Script> {
        self.scripts.values().collect()
    }
    
    pub fn get_status(&self, id: &Uuid) -> Option<&ScriptStatus> {
        self.statuses.get(id)
    }
    
    /// Evaluate all scripts with current register values
    /// Returns IDs of triggered scripts
    pub fn evaluate(&mut self, registers: &HashMap<String, u16>) -> Vec<Uuid> {
        let mut triggered = Vec::new();
        
        // Collect IDs first to avoid borrowing issues
        let script_ids: Vec<Uuid> = self.scripts.keys().cloned().collect();
        
        for id in script_ids {
            // Get immutable references
            let trigger = {
                let script = match self.scripts.get(&id) {
                    Some(s) => s,
                    None => continue,
                };
                if !script.enabled {
                    continue;
                }
                script.trigger.clone()
            };
            
            // Check trigger
            if self.check_trigger(&trigger, registers) {
                // Mark as triggered
                if let Some(status) = self.statuses.get_mut(&id) {
                    status.mark_triggered();
                }
                triggered.push(id);
            }
        }
        
        // Update last values for change detection
        for (addr, value) in registers {
            self.last_values.insert(addr.clone(), *value);
        }
        
        triggered
    }
    
    /// Check if a trigger condition is met
    fn check_trigger(&self, trigger: &Trigger, registers: &HashMap<String, u16>) -> bool {
        match trigger {
            Trigger::Compare { register, operator, value } => {
                self.check_compare(register, *operator, *value, registers)
            }
            Trigger::Changed { register } => {
                self.check_changed(register, registers)
            }
            Trigger::BecameOn { register } => {
                self.check_became(register, true, registers)
            }
            Trigger::BecameOff { register } => {
                self.check_became(register, false, registers)
            }
            Trigger::Stable { register, .. } => {
                // Simplified: just check if register exists
                registers.contains_key(register)
            }
        }
    }
    
    fn check_compare(&self, register: &str, operator: CompareOp, expected: i64, registers: &HashMap<String, u16>) -> bool {
        let Some(value) = registers.get(register) else { return false; };
        let actual = *value as i64;
        operator.evaluate(actual, expected)
    }
    
    fn check_changed(&self, register: &str, registers: &HashMap<String, u16>) -> bool {
        let Some(current) = registers.get(register) else { return false; };
        let last = self.last_values.get(register);
        last != Some(current)
    }
    
    fn check_became(&self, register: &str, became_on: bool, registers: &HashMap<String, u16>) -> bool {
        let Some(current) = registers.get(register) else { return false; };
        let last = self.last_values.get(register);
        let became_state = if became_on { *current != 0 } else { *current == 0 };
        became_state && last != Some(current)
    }
    
    /// Execute actions for a triggered script
    /// Returns list of action descriptions that were executed
    pub fn execute_script(&mut self, id: &Uuid) -> Result<Vec<String>, String> {
        let script = match self.scripts.get(id) {
            Some(s) => s,
            None => return Err("Script not found".to_string()),
        };
        
        if !script.enabled {
            return Err("Script is disabled".to_string());
        }
        
        // Mark as running
        if let Some(status) = self.statuses.get_mut(id) {
            status.running = true;
        }
        
        let mut executed = Vec::new();
        for action in &script.actions {
            // Log the action execution
            let desc = action.description();
            executed.push(desc.clone());
            tracing::info!("Script '{}' executing action: {}", script.name, desc);
        }
        
        // Mark as finished (in real impl, this would be async)
        if let Some(status) = self.statuses.get_mut(id) {
            status.running = false;
            status.last_triggered = Some(chrono::Utc::now());
        }
        
        Ok(executed)
    }
    
    /// Get the action queue for a script (for UI display)
    pub fn get_script_actions(&self, id: &Uuid) -> Option<Vec<String>> {
        self.scripts.get(id).map(|s| {
            s.actions.iter().map(|a| a.description()).collect()
        })
    }
}

impl Default for ScriptEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_engine_add_script() {
        let mut engine = ScriptEngine::new();
        let script = Script::new("Test".to_string(), Trigger::Changed {
            register: "40001".to_string(),
        });
        engine.add_script(script);
        assert_eq!(engine.get_all_scripts().len(), 1);
    }
    
    #[test]
    fn test_check_compare() {
        let engine = ScriptEngine::new();
        let mut registers = HashMap::new();
        registers.insert("40001".to_string(), 50);
        
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::GT,
            value: 30,
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }
    
    #[test]
    fn test_evaluate_triggered() {
        let mut engine = ScriptEngine::new();
        let script = Script::new("Test".to_string(), Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::GT,
            value: 30,
        });
        let id = script.id;
        engine.add_script(script);
        
        let mut registers = HashMap::new();
        registers.insert("40001".to_string(), 50);
        
        let triggered = engine.evaluate(&registers);
        assert_eq!(triggered.len(), 1);
        assert_eq!(triggered[0], id);
    }
    
    #[test]
    fn test_evaluate_not_triggered() {
        let mut engine = ScriptEngine::new();
        let script = Script::new("Test".to_string(), Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::GT,
            value: 30,
        });
        engine.add_script(script);
        
        let mut registers = HashMap::new();
        registers.insert("40001".to_string(), 20);
        
        let triggered = engine.evaluate(&registers);
        assert!(triggered.is_empty());
    }
    
    #[test]
    fn test_execute_script() {
        let mut engine = ScriptEngine::new();
        let mut script = Script::new("Test".to_string(), Trigger::Changed {
            register: "40001".to_string(),
        });
        script.add_action(Action::WriteOn {
            register: "00001".to_string(),
        });
        script.add_action(Action::ShowNotification {
            title: "Alert".to_string(),
            message: "Triggered!".to_string(),
        });
        let id = script.id;
        engine.add_script(script);
        
        let result = engine.execute_script(&id);
        assert!(result.is_ok());
        let actions = result.unwrap();
        assert_eq!(actions.len(), 2);
        assert_eq!(actions[0], "Turn ON 00001");
        assert_eq!(actions[1], "Notify: Alert - Triggered!");
    }
    
    #[test]
    fn test_execute_nonexistent_script() {
        let mut engine = ScriptEngine::new();
        let id = Uuid::new_v4();
        
        let result = engine.execute_script(&id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Script not found");
    }
}
