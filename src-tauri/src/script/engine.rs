//! Script execution engine

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::{Action, Script, ScriptStatus, Trigger};
use crate::modbus::protocol::*;

/// Script execution engine
/// Evaluates triggers and executes actions
pub struct ScriptEngine {
    /// Registered scripts
    scripts: HashMap<Uuid, Script>,
    
    /// Script execution status
    statuses: HashMap<Uuid, ScriptStatus>,
    
    /// Last known register values for change detection
    last_values: HashMap<String, u16>,
    
    /// Values stable since (for Stable trigger)
    stable_since: HashMap<String, std::time::Instant>,
}

impl ScriptEngine {
    /// Create a new script engine
    pub fn new() -> Self {
        Self {
            scripts: HashMap::new(),
            statuses: HashMap::new(),
            last_values: HashMap::new(),
            stable_since: HashMap::new(),
        }
    }
    
    /// Add a script to the engine
    pub fn add_script(&mut self, script: Script) {
        let id = script.id;
        self.scripts.insert(id, script);
        self.statuses.insert(id, ScriptStatus::new(id));
    }
    
    /// Remove a script by ID
    pub fn remove_script(&mut self, id: &Uuid) -> Option<Script> {
        self.statuses.remove(id);
        self.scripts.remove(id)
    }
    
    /// Get a script by ID
    pub fn get_script(&self, id: &Uuid) -> Option<&Script> {
        self.scripts.get(id)
    }
    
    /// Get all scripts
    pub fn get_all_scripts(&self) -> Vec<&Script> {
        self.scripts.values().collect()
    }
    
    /// Get script status
    pub fn get_status(&self, id: &Uuid) -> Option<&ScriptStatus> {
        self.statuses.get(id)
    }
    
    /// Evaluate all enabled triggers with current register values
    pub fn evaluate(&mut self, registers: &HashMap<String, u16>) -> Vec<(Uuid, Vec<Action>)> {
        let mut triggered = Vec::new();
        
        for (id, script) in &mut self.scripts {
            if !script.enabled {
                continue;
            }
            
            if let Some(actions) = self.evaluate_trigger(&script.trigger, registers) {
                if let Some(status) = self.statuses.get_mut(id) {
                    status.mark_triggered();
                }
                triggered.push((*id, actions));
            }
        }
        
        // Update last values for change detection
        for (addr, value) in registers {
            self.last_values.insert(addr.clone(), *value);
        }
        
        triggered
    }
    
    /// Evaluate a single trigger
    fn evaluate_trigger(&mut self, trigger: &Trigger, registers: &HashMap<String, u16>) -> Option<Vec<Action>> {
        match trigger {
            Trigger::Compare { register, operator, value } => {
                self.evaluate_compare(register, *operator, *value, registers)
            }
            Trigger::Changed { register } => {
                self.evaluate_changed(register, registers)
            }
            Trigger::BecameOn { register } => {
                self.evaluate_became_on(register, registers)
            }
            Trigger::BecameOff { register } => {
                self.evaluate_became_off(register, registers)
            }
            Trigger::Stable { register, seconds } => {
                self.evaluate_stable(register, *seconds, registers)
            }
        }
    }
    
    fn evaluate_compare(
        &self,
        register: &str,
        operator: CompareOp,
        expected: i64,
        registers: &HashMap<String, u16>,
    ) -> Option<Vec<Action>> {
        let value = registers.get(register)?;
        let actual = *value as i64;
        
        if operator.evaluate(actual, expected) {
            // Trigger fired, return a copy of actions
            Some(vec![]) // Placeholder - actual actions come from script
        } else {
            None
        }
    }
    
    fn evaluate_changed(&mut self, register: &str, registers: &HashMap<String, u16>) -> Option<Vec<Action>> {
        let current = registers.get(register)?;
        let last = self.last_values.get(register);
        
        if last != Some(current) {
            Some(vec![]) // Placeholder
        } else {
            None
        }
    }
    
    fn evaluate_became_on(&mut self, register: &str, registers: &HashMap<String, u16>) -> Option<Vec<Action>> {
        let current = registers.get(register)?;
        let last = self.last_values.get(register);
        
        if *current != 0 && last != Some(current) {
            Some(vec![]) // Became ON
        } else {
            None
        }
    }
    
    fn evaluate_became_off(&mut self, register: &str, registers: &HashMap<String, u16>) -> Option<Vec<Action>> {
        let current = registers.get(register)?;
        let last = self.last_values.get(register);
        
        if *current == 0 && last != Some(current) {
            Some(vec![]) // Became OFF
        } else {
            None
        }
    }
    
    fn evaluate_stable(
        &mut self,
        register: &str,
        seconds: u64,
        registers: &HashMap<String, u16>,
    ) -> Option<Vec<Action>> {
        let current = registers.get(register)?;
        let last = self.last_values.get(register);
        
        if last == Some(current) {
            // Value unchanged, check if stable long enough
            let stable = self.stable_since
                .entry(register.to_string())
                .or_insert_with(std::time::Instant::now);
            
            let duration = stable.elapsed().as_secs();
            if duration >= seconds {
                Some(vec![]) // Stable for required time
            } else {
                None
            }
        } else {
            // Value changed, reset timer
            self.stable_since.insert(register.to_string(), std::time::Instant::now());
            None
        }
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_evaluate_changed() {
        let mut engine = ScriptEngine::new();
        let script = Script::new("Test".to_string(), Trigger::Changed {
            register: "40001".to_string(),
        });
        let script_id = script.id;
        engine.add_script(script);
        
        let mut registers = HashMap::new();
        registers.insert("40001".to_string(), 100);
        
        let _triggered = engine.evaluate(&registers);
        // First evaluation should trigger (no previous value)
        // Changed trigger should produce actions
        assert!(!_triggered.is_empty());
        assert_eq!(_triggered[0].0, script_id);
    }
}
