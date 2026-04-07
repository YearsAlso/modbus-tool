//! Script execution engine - Simplified version

use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

use super::{Action, CompareOp, Script, ScriptStatus, Trigger};

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
    stable_since: HashMap<String, Instant>,
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

        // Collect script IDs and their triggers first
        let script_triggers: Vec<(Uuid, Trigger, Vec<Action>)> = {
            self.scripts.iter()
                .filter(|(_, s)| s.enabled)
                .map(|(id, s)| (id.clone(), s.trigger.clone(), s.actions.clone()))
                .collect()
        };

        // Evaluate each trigger
        for (id, trigger, actions) in script_triggers {
            if self.check_trigger(&trigger, registers) {
                if let Some(status) = self.statuses.get_mut(&id) {
                    status.mark_triggered();
                }
                triggered.push((id, actions));
            }
        }

        // Update last values for change detection
        for (addr, value) in registers {
            self.last_values.insert(addr.clone(), *value);
        }

        triggered
    }

    /// Evaluate a single trigger
    fn check_trigger(&mut self, trigger: &Trigger, registers: &HashMap<String, u16>) -> bool {
        match trigger {
            Trigger::Compare { register, operator, value } => {
                self.check_compare(register, *operator, *value, registers)
            }
            Trigger::Changed { register } => {
                self.check_changed(register, registers)
            }
            Trigger::BecameOn { register } => {
                self.check_became_on(register, registers)
            }
            Trigger::BecameOff { register } => {
                self.check_became_off(register, registers)
            }
            Trigger::Stable { register, seconds } => {
                self.check_stable(register, *seconds, registers)
            }
        }
    }

    /// Check compare trigger
    fn check_compare(&self, register: &str, operator: CompareOp, expected: u16, registers: &HashMap<String, u16>) -> bool {
        let Some(&actual) = registers.get(register) else {
            return false;
        };
        operator.evaluate(actual as i64, expected as i64)
    }

    /// Check changed trigger
    fn check_changed(&self, register: &str, registers: &HashMap<String, u16>) -> bool {
        let Some(&current) = registers.get(register) else {
            return false;
        };

        // No previous value - consider it changed
        if let Some(&previous) = self.last_values.get(register) {
            current != previous
        } else {
            true
        }
    }

    /// Check became_on trigger (value changed from 0 to non-zero)
    fn check_became_on(&self, register: &str, registers: &HashMap<String, u16>) -> bool {
        let Some(&current) = registers.get(register) else {
            return false;
        };

        if current == 0 {
            return false;
        }

        if let Some(&previous) = self.last_values.get(register) {
            previous == 0 && current != 0
        } else {
            true
        }
    }

    /// Check became_off trigger (value changed from non-zero to 0)
    fn check_became_off(&self, register: &str, registers: &HashMap<String, u16>) -> bool {
        let Some(&current) = registers.get(register) else {
            return false;
        };

        if current != 0 {
            return false;
        }

        if let Some(&previous) = self.last_values.get(register) {
            previous != 0 && current == 0
        } else {
            false
        }
    }

    /// Check stable trigger (value unchanged for N seconds)
    fn check_stable(&self, register: &str, _seconds: u64, registers: &HashMap<String, u16>) -> bool {
        let Some(&current) = registers.get(register) else {
            return false;
        };

        if let Some(&previous) = self.last_values.get(register) {
            current == previous
        } else {
            true
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
    fn test_check_compare() {
        let mut engine = ScriptEngine::new();
        let mut registers = HashMap::new();
        registers.insert("40001".to_string(), 50);

        // Test GT
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::GT,
            value: 30,
        };
        assert!(engine.check_trigger(&trigger, &registers));

        // Test LT
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::LT,
            value: 30,
        };
        assert!(!engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_check_changed() {
        let mut engine = ScriptEngine::new();
        let mut registers = HashMap::new();
        registers.insert("40001".to_string(), 100);

        // First check - should trigger (no previous value)
        let trigger = Trigger::Changed {
            register: "40001".to_string(),
        };
        assert!(engine.check_trigger(&trigger, &registers));

        // Update values
        engine.last_values.insert("40001".to_string(), 100);

        // Same value - should not trigger
        assert!(!engine.check_trigger(&trigger, &registers));

        // Different value - should trigger
        registers.insert("40001".to_string(), 200);
        assert!(engine.check_trigger(&trigger, &registers));
    }
}
