//! Script execution engine - Simplified version

use std::collections::HashMap;
use uuid::Uuid;

use super::trigger::CompareOp;
use super::{Script, ScriptStatus, Trigger};

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
    
    pub fn get_all_scripts(&self) -> Vec<&Script> {
        self.scripts.values().collect()
    }
    
    pub fn get_status(&self, id: &Uuid) -> Option<&ScriptStatus> {
        self.statuses.get(id)
    }
    
    pub fn evaluate(&mut self, registers: &HashMap<String, u16>) -> Vec<Uuid> {
        let mut triggered = Vec::new();
        
        for (id, script) in &self.scripts {
            if !script.enabled {
                continue;
            }
            
            if self.check_trigger(&script.trigger, registers) {
                if let Some(status) = self.statuses.get_mut(id) {
                    status.mark_triggered();
                }
                triggered.push(*id);
            }
        }
        
        for (addr, value) in registers {
            self.last_values.insert(addr.clone(), *value);
        }
        
        triggered
    }
    
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
}
