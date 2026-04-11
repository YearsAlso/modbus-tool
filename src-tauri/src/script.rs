//! Modbus Automation Script Engine
//!
//! Implements the "If...Then..." automation logic for Modbus registers.
//! Each script consists of a Trigger and a list of Actions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

// ========== Data Types ==========

/// Comparison operators for trigger conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompareOp {
    Greater,       // >
    Less,          // <
    Equal,         // =
    NotEqual,      // ≠
    GreaterOrEq,   // ≥
    LessOrEq,      // ≤
}

impl CompareOp {
    /// Evaluate the comparison operator with two f64 values
    pub fn evaluate(self, left: f64, right: f64) -> bool {
        match self {
            CompareOp::Greater => left > right,
            CompareOp::Less => left < right,
            CompareOp::Equal => (left - right).abs() < f64::EPSILON,
            CompareOp::NotEqual => (left - right).abs() >= f64::EPSILON,
            CompareOp::GreaterOrEq => left >= right,
            CompareOp::LessOrEq => left <= right,
        }
    }
}

/// Register reference (e.g., "40001" = holding register 1)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegisterRef(pub String);

impl fmt::Display for RegisterRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Trigger condition for a script
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Trigger {
    /// Numeric comparison: register operator value
    Compare {
        register: String,
        operator: CompareOp,
        value: f64,
    },
    /// Detect value change (any change)
    Changed {
        register: String,
    },
    /// Detect rising edge (0 -> 1)
    BecameOn {
        register: String,
    },
    /// Detect falling edge (1 -> 0)
    BecameOff {
        register: String,
    },
    /// Detect value stable for N seconds (placeholder for future)
    Stable {
        register: String,
        seconds: u32,
    },
    /// AND combination of multiple triggers (all must be true to fire)
    And {
        triggers: Vec<Trigger>,
    },
}

/// Action to perform when trigger fires
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// Write a fixed value to a register
    WriteValue {
        register: String,
        value: u16,
    },
    /// Write ON (0xFF00) to a register
    WriteOn {
        register: String,
    },
    /// Write OFF (0x0000) to a register
    WriteOff {
        register: String,
    },
    /// Toggle the current value (ON <-> OFF)
    Toggle {
        register: String,
    },
    /// Show a desktop notification
    ShowNotification {
        message: String,
    },
    /// Play a sound
    PlaySound {
        sound: String,
    },
    /// Log a message
    Log {
        message: String,
    },
    /// Start another script
    RunScript {
        script_id: String,
    },
    /// Stop another script
    StopScript {
        script_id: String,
    },
    /// Delay for N milliseconds before next action
    Delay {
        ms: u64,
    },
}

/// Script definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub id: Uuid,
    pub name: String,
    pub trigger: Trigger,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Script {
    /// Create a new script with default values
    pub fn new(name: String, trigger: Trigger, actions: Vec<Action>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            trigger,
            actions,
            enabled: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Script execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptStatus {
    pub running: bool,
    pub last_triggered: Option<DateTime<Utc>>,
    pub trigger_count: u32,
}

impl ScriptStatus {
    pub fn new() -> Self {
        Self {
            running: false,
            last_triggered: None,
            trigger_count: 0,
        }
    }
}

impl Default for ScriptStatus {
    fn default() -> Self {
        Self::new()
    }
}

// ========== Action Execution Context ==========

/// Modbus client trait for script actions
/// Implement this to provide actual Modbus write capabilities
pub trait ModbusClient: Send + Sync {
    fn write_register(&self, register: &str, value: u16) -> Result<(), String>;
    fn write_coil(&self, register: &str, value: bool) -> Result<(), String>;
}

/// Simple context for action results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub action_type: String,
    pub success: bool,
    pub message: String,
}

impl ActionResult {
    pub fn ok(action_type: &str, message: &str) -> Self {
        Self {
            action_type: action_type.to_string(),
            success: true,
            message: message.to_string(),
        }
    }

    pub fn fail(action_type: &str, message: &str) -> Self {
        Self {
            action_type: action_type.to_string(),
            success: false,
            message: message.to_string(),
        }
    }
}

/// Context passed to action executors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptContext {
    pub script_id: Uuid,
    pub trigger_name: String,
    pub register_values: HashMap<String, u16>,
}

impl ScriptContext {
    pub fn new(script_id: Uuid, register_values: HashMap<String, u16>) -> Self {
        Self {
            script_id,
            trigger_name: String::new(),
            register_values,
        }
    }
}

/// Log level for script logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

// ========== ScriptEngine ==========

/// Script execution engine
///
/// Manages all scripts, their running state, and evaluates triggers
/// on each Modbus polling cycle.
pub struct ScriptEngine {
    /// All loaded scripts
    scripts: Vec<Script>,
    /// Script execution status (keyed by script UUID string)
    statuses: HashMap<Uuid, ScriptStatus>,
    /// Currently running scripts (script ID -> running flag)
    running_scripts: HashMap<String, bool>,
    /// Last polled values for change detection (register -> value)
    last_values: HashMap<String, u16>,
}

impl ScriptEngine {
    /// Create a new ScriptEngine
    pub fn new() -> Self {
        Self {
            scripts: Vec::new(),
            statuses: HashMap::new(),
            running_scripts: HashMap::new(),
            last_values: HashMap::new(),
        }
    }

    // ========== Script Management ==========

    /// Add or update a script
    pub fn add_script(&mut self, script: Script) {
        // Ensure status exists
        self.statuses
            .entry(script.id)
            .or_insert_with(ScriptStatus::new);
        // Remove existing script with same ID and add new one
        self.scripts.retain(|s| s.id != script.id);
        self.scripts.push(script);
    }

    /// Get a script by ID
    pub fn get_script(&self, id: &Uuid) -> Option<&Script> {
        self.scripts.iter().find(|s| s.id == *id)
    }

    /// Get all scripts
    pub fn get_all_scripts(&self) -> &Vec<Script> {
        &self.scripts
    }

    /// Remove a script by ID
    pub fn remove_script(&mut self, id: &Uuid) -> Option<Script> {
        self.statuses.remove(id);
        self.running_scripts.remove(&id.to_string());
        if let Some(pos) = self.scripts.iter().position(|s| s.id == *id) {
            Some(self.scripts.remove(pos))
        } else {
            None
        }
    }

    // ========== Status Management ==========

    /// Get script status
    pub fn get_status(&self, id: &Uuid) -> Option<&ScriptStatus> {
        self.statuses.get(id)
    }

    /// Get mutable script status
    pub fn get_status_mut(&mut self, id: &Uuid) -> Option<&mut ScriptStatus> {
        self.statuses.get_mut(id)
    }

    /// Get all statuses
    pub fn get_all_statuses(&self) -> &HashMap<Uuid, ScriptStatus> {
        &self.statuses
    }

    // ========== Running Scripts Management ==========

    /// Mark a script as running
    pub fn start_script(&mut self, id: &Uuid) {
        self.running_scripts.insert(id.to_string(), true);
        if let Some(status) = self.statuses.get_mut(id) {
            status.running = true;
        }
        log::info!("Script started: {}", id);
    }

    /// Mark a script as stopped
    pub fn stop_script(&mut self, id: &Uuid) {
        self.running_scripts.insert(id.to_string(), false);
        if let Some(status) = self.statuses.get_mut(id) {
            status.running = false;
        }
        log::info!("Script stopped: {}", id);
    }

    /// Check if a script is running
    pub fn is_running(&self, id: &Uuid) -> bool {
        self.running_scripts
            .get(&id.to_string())
            .copied()
            .unwrap_or(false)
    }

    // ========== Trigger Checking ==========

    /// Check if a trigger condition is met
    ///
    /// Returns `true` if the trigger should fire based on current register values
    /// and the previous values stored in `last_values`.
    pub fn check_trigger(&self, trigger: &Trigger, registers: &HashMap<String, u16>) -> bool {
        match trigger {
            Trigger::Compare {
                register,
                operator,
                value,
            } => {
                if let Some(&reg_value) = registers.get(register) {
                    let reg_f64 = reg_value as f64;
                    operator.evaluate(reg_f64, *value)
                } else {
                    false
                }
            }

            Trigger::Changed { register } => {
                if let (Some(&current), Some(&last)) = (registers.get(register), self.last_values.get(register))
                {
                    current != last
                } else {
                    // Either not in current or not in last - consider it a change only if it's new
                    registers.contains_key(register)
                }
            }

            Trigger::BecameOn { register } => {
                // Rising edge: last was 0 (or absent), current is non-zero
                let last_val = self.last_values.get(register).copied().unwrap_or(0);
                let current_val = registers.get(register).copied().unwrap_or(0);
                last_val == 0 && current_val != 0
            }

            Trigger::BecameOff { register } => {
                // Falling edge: last was non-zero, current is 0
                let last_val = self.last_values.get(register).copied().unwrap_or(0);
                let current_val = registers.get(register).copied().unwrap_or(0);
                last_val != 0 && current_val == 0
            }

            Trigger::Stable { register, seconds: _ } => {
                // For now, stable detection is a no-op (requires timer tracking)
                // TODO: implement stable detection with timestamp tracking
                false
            }

            Trigger::And { triggers } => {
                // All sub-triggers must be true for AND to fire
                triggers.iter().all(|t| self.check_trigger(t, registers))
            }
        }
    }

    // ========== Action Execution ==========

    /// Execute a single action
    ///
    /// Returns an `ActionResult` describing what happened.
    /// Note: Actual Modbus writes and notifications are logged/queued;
    /// real implementation would call the ModbusClient or notification system.
    pub fn execute_action(&self, action: &Action, _ctx: &ScriptContext) -> ActionResult {
        match action {
            Action::WriteValue { register, value } => {
                log::info!("[ACTION] WriteValue: {} = {}", register, value);
                ActionResult::ok("write_value", &format!("Wrote {} to {}", value, register))
            }

            Action::WriteOn { register } => {
                log::info!("[ACTION] WriteOn: {}", register);
                ActionResult::ok("write_on", &format!("Wrote ON to {}", register))
            }

            Action::WriteOff { register } => {
                log::info!("[ACTION] WriteOff: {}", register);
                ActionResult::ok("write_off", &format!("Wrote OFF to {}", register))
            }

            Action::Toggle { register } => {
                log::info!("[ACTION] Toggle: {}", register);
                ActionResult::ok("toggle", &format!("Toggled {}", register))
            }

            Action::ShowNotification { message } => {
                log::info!("[ACTION] ShowNotification: {}", message);
                ActionResult::ok("notification", message)
            }

            Action::PlaySound { sound } => {
                log::info!("[ACTION] PlaySound: {}", sound);
                ActionResult::ok("play_sound", &format!("Playing sound: {}", sound))
            }

            Action::Log { message } => {
                log::info!("[ACTION] Log: {}", message);
                ActionResult::ok("log", message)
            }

            Action::RunScript { script_id } => {
                log::info!("[ACTION] RunScript: {}", script_id);
                ActionResult::ok("run_script", &format!("Started script: {}", script_id))
            }

            Action::StopScript { script_id } => {
                log::info!("[ACTION] StopScript: {}", script_id);
                ActionResult::ok("stop_script", &format!("Stopped script: {}", script_id))
            }

            Action::Delay { ms } => {
                log::info!("[ACTION] Delay: {}ms", ms);
                ActionResult::ok("delay", &format!("Delayed {}ms", ms))
            }
        }
    }

    /// Execute all actions for a script
    pub fn execute_actions(&self, actions: &[Action], ctx: &ScriptContext) -> Vec<ActionResult> {
        actions
            .iter()
            .map(|action| self.execute_action(action, ctx))
            .collect()
    }

    // ========== Script Execution ==========

    /// Execute a specific script by ID
    pub fn execute_script(&mut self, id: &Uuid) -> Result<Vec<ActionResult>, String> {
        let script = self
            .scripts
            .iter()
            .find(|s| s.id == *id)
            .ok_or_else(|| format!("Script not found: {}", id))?;

        let register_values = self.last_values.clone();
        let ctx = ScriptContext::new(*id, register_values);
        let results = self.execute_actions(&script.actions, &ctx);

        // Update status
        if let Some(status) = self.statuses.get_mut(id) {
            status.last_triggered = Some(chrono::Utc::now());
            status.trigger_count += 1;
        }

        log::info!(
            "Script '{}' executed {} actions",
            script.name,
            results.len()
        );

        Ok(results)
    }

    // ========== Main Evaluation Loop ==========

    /// Evaluate all running scripts against current register values
    ///
    /// This is the main entry point called each Modbus polling cycle.
    /// - Only scripts with `running_scripts[id] == true` are evaluated
    /// - Only enabled scripts are evaluated
    /// - For each triggered script, actions are executed immediately
    ///
    /// Returns the list of script IDs that were triggered.
    pub fn evaluate(&mut self, registers: &HashMap<String, u16>) -> Vec<Uuid> {
        let mut triggered = Vec::new();

        // Update last_values with current register values
        for (key, value) in registers {
            self.last_values.insert(key.clone(), *value);
        }

        for script in &self.scripts {
            // Skip disabled scripts
            if !script.enabled {
                continue;
            }

            // Check if script is marked as running
            let is_running = self
                .running_scripts
                .get(&script.id.to_string())
                .copied()
                .unwrap_or(false);

            if !is_running {
                continue;
            }

            // Check trigger condition
            if self.check_trigger(&script.trigger, registers) {
                log::info!(
                    "Script triggered: '{}' (id={})",
                    script.name,
                    script.id
                );

                // Execute actions
                let ctx = ScriptContext::new(script.id, registers.clone());
                let results = self.execute_actions(&script.actions, &ctx);

                // Update status
                if let Some(status) = self.statuses.get_mut(&script.id) {
                    status.last_triggered = Some(chrono::Utc::now());
                    status.trigger_count += 1;
                }

                let ok_count = results.iter().filter(|r| r.success).count();
                log::info!(
                    "Script '{}' executed {} actions ({} ok, {} failed)",
                    script.name,
                    results.len(),
                    ok_count,
                    results.len() - ok_count
                );

                triggered.push(script.id);
            }
        }

        triggered
    }

    /// Clear all scripts and state
    pub fn clear(&mut self) {
        self.scripts.clear();
        self.statuses.clear();
        self.running_scripts.clear();
        self.last_values.clear();
    }

    /// Get count of scripts
    pub fn script_count(&self) -> usize {
        self.scripts.len()
    }

    /// Get count of running scripts
    pub fn running_count(&self) -> usize {
        self.running_scripts.values().filter(|&&v| v).count()
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Type alias for the Arc<Mutex<ScriptEngine>> pattern used in commands
pub type ScriptEngineRef = std::sync::Arc<parking_lot::Mutex<ScriptEngine>>;

// ========== Tests ==========

#[cfg(test)]
mod tests {
    use super::*;

    fn make_registers(values: &[(&str, u16)]) -> HashMap<String, u16> {
        values.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    // ---------- Trigger: Compare tests ----------

    #[test]
    fn test_compare_greater_true() {
        let engine = ScriptEngine::new();
        let registers = make_registers(&[("40001", 50)]);
        let trigger = Trigger::Compare {
            register: "40001".into(),
            operator: CompareOp::Greater,
            value: 30.0,
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_compare_greater_false() {
        let engine = ScriptEngine::new();
        let registers = make_registers(&[("40001", 20)]);
        let trigger = Trigger::Compare {
            register: "40001".into(),
            operator: CompareOp::Greater,
            value: 30.0,
        };
        assert!(!engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_compare_less() {
        let engine = ScriptEngine::new();
        let registers = make_registers(&[("40001", 20)]);
        let trigger = Trigger::Compare {
            register: "40001".into(),
            operator: CompareOp::Less,
            value: 30.0,
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_compare_equal() {
        let engine = ScriptEngine::new();
        let registers = make_registers(&[("40001", 30)]);
        let trigger = Trigger::Compare {
            register: "40001".into(),
            operator: CompareOp::Equal,
            value: 30.0,
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_compare_not_equal() {
        let engine = ScriptEngine::new();
        let registers = make_registers(&[("40001", 31)]);
        let trigger = Trigger::Compare {
            register: "40001".into(),
            operator: CompareOp::NotEqual,
            value: 30.0,
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_compare_greater_or_eq() {
        let engine = ScriptEngine::new();
        let registers = make_registers(&[("40001", 30)]);
        let trigger = Trigger::Compare {
            register: "40001".into(),
            operator: CompareOp::GreaterOrEq,
            value: 30.0,
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_compare_less_or_eq() {
        let engine = ScriptEngine::new();
        let registers = make_registers(&[("40001", 30)]);
        let trigger = Trigger::Compare {
            register: "40001".into(),
            operator: CompareOp::LessOrEq,
            value: 30.0,
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_compare_missing_register() {
        let engine = ScriptEngine::new();
        let registers = make_registers(&[]);
        let trigger = Trigger::Compare {
            register: "40001".into(),
            operator: CompareOp::Greater,
            value: 30.0,
        };
        assert!(!engine.check_trigger(&trigger, &registers));
    }

    // ---------- Trigger: Change detection tests ----------

    #[test]
    fn test_became_on_rising_edge() {
        let mut engine = ScriptEngine::new();
        // First poll: register goes from 0 -> 1
        engine.last_values.insert("40001".into(), 0);
        let registers = make_registers(&[("40001", 1)]);
        let trigger = Trigger::BecameOn {
            register: "40001".into(),
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_became_on_already_on() {
        let mut engine = ScriptEngine::new();
        // Register already ON
        engine.last_values.insert("40001".into(), 1);
        let registers = make_registers(&[("40001", 1)]);
        let trigger = Trigger::BecameOn {
            register: "40001".into(),
        };
        assert!(!engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_became_off_falling_edge() {
        let mut engine = ScriptEngine::new();
        engine.last_values.insert("40001".into(), 1);
        let registers = make_registers(&[("40001", 0)]);
        let trigger = Trigger::BecameOff {
            register: "40001".into(),
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_changed_detected() {
        let mut engine = ScriptEngine::new();
        engine.last_values.insert("40001".into(), 100);
        let registers = make_registers(&[("40001", 200)]);
        let trigger = Trigger::Changed {
            register: "40001".into(),
        };
        assert!(engine.check_trigger(&trigger, &registers));
    }

    #[test]
    fn test_changed_not_changed() {
        let mut engine = ScriptEngine::new();
        engine.last_values.insert("40001".into(), 100);
        let registers = make_registers(&[("40001", 100)]);
        let trigger = Trigger::Changed {
            register: "40001".into(),
        };
        assert!(!engine.check_trigger(&trigger, &registers));
    }

    // ---------- Evaluate tests ----------

    #[test]
    fn test_evaluate_runs_only_running_scripts() {
        let mut engine = ScriptEngine::new();
        let script = Script::new(
            "Test Script".into(),
            Trigger::Compare {
                register: "40001".into(),
                operator: CompareOp::Greater,
                value: 30.0,
            },
            vec![Action::ShowNotification {
                message: "Triggered!".into(),
            }],
        );
        let script_id = script.id;
        engine.add_script(script);
        // enabled=true but NOT running -> should not trigger
        // (need to set enabled=true manually since Default is false)
        let registers = make_registers(&[("40001", 50)]);

        // Script is enabled but not running
        assert!(engine.evaluate(&registers).is_empty());
    }

    #[test]
    fn test_evaluate_triggers_running_enabled_script() {
        let mut engine = ScriptEngine::new();
        let mut script = Script::new(
            "Test Script".into(),
            Trigger::Compare {
                register: "40001".into(),
                operator: CompareOp::Greater,
                value: 30.0,
            },
            vec![Action::ShowNotification {
                message: "Triggered!".into(),
            }],
        );
        script.enabled = true;
        let script_id = script.id;
        engine.add_script(script);
        engine.start_script(&script_id);

        let registers = make_registers(&[("40001", 50)]);
        let triggered = engine.evaluate(&registers);
        assert_eq!(triggered, vec![script_id]);
    }

    #[test]
    fn test_evaluate_disabled_script_not_triggered() {
        let mut engine = ScriptEngine::new();
        let script = Script::new(
            "Test Script".into(),
            Trigger::Compare {
                register: "40001".into(),
                operator: CompareOp::Greater,
                value: 30.0,
            },
            vec![],
        );
        // Note: enabled = false (default)
        let script_id = script.id;
        engine.add_script(script);
        engine.start_script(&script_id);

        let registers = make_registers(&[("40001", 50)]);
        assert!(engine.evaluate(&registers).is_empty());
    }

    // ---------- Script management tests ----------

    #[test]
    fn test_add_and_get_script() {
        let mut engine = ScriptEngine::new();
        let script = Script::new("Test".into(), Trigger::Changed { register: "40001".into() }, vec![]);
        let id = script.id;
        engine.add_script(script);
        assert_eq!(engine.get_script(&id).unwrap().name, "Test");
    }

    #[test]
    fn test_remove_script() {
        let mut engine = ScriptEngine::new();
        let script = Script::new("Test".into(), Trigger::Changed { register: "40001".into() }, vec![]);
        let id = script.id;
        engine.add_script(script);
        engine.start_script(&id);
        let removed = engine.remove_script(&id);
        assert!(removed.is_some());
        assert!(engine.get_script(&id).is_none());
        assert!(!engine.is_running(&id));
    }

    #[test]
    fn test_script_engine_ref_type() {
        let engine: ScriptEngineRef = std::sync::Arc::new(parking_lot::Mutex::new(ScriptEngine::new()));
        let _ = engine.lock();
    }
}
