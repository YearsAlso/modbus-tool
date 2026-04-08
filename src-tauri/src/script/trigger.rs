//! Trigger definitions for automation scripts

use serde::{Deserialize, Serialize};

/// Comparison operators for trigger conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompareOp {
    GT,   // greater than (>)
    LT,   // less than (<)
    EQ,   // equal (=)
    NEQ,  // not equal (≠)
    GTE,  // greater than or equal (≥)
    LTE,  // less than or equal (≤)
}

impl CompareOp {
    /// Evaluate the comparison
    pub fn evaluate(&self, actual: i64, expected: i64) -> bool {
        match self {
            CompareOp::GT => actual > expected,
            CompareOp::LT => actual < expected,
            CompareOp::EQ => actual == expected,
            CompareOp::NEQ => actual != expected,
            CompareOp::GTE => actual >= expected,
            CompareOp::LTE => actual <= expected,
        }
    }
    
    /// Get the display symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            CompareOp::GT => ">",
            CompareOp::LT => "<",
            CompareOp::EQ => "=",
            CompareOp::NEQ => "≠",
            CompareOp::GTE => "≥",
            CompareOp::LTE => "≤",
        }
    }
}

/// Trigger conditions for automation scripts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Trigger {
    /// Compare a register value against a threshold
    Compare {
        register: String,
        operator: CompareOp,
        value: i64,
    },
    
    /// Trigger when any value change is detected
    Changed {
        register: String,
    },
    
    /// Trigger when a coil/input becomes ON (value != 0)
    BecameOn {
        register: String,
    },
    
    /// Trigger when a coil/input becomes OFF (value == 0)
    BecameOff {
        register: String,
    },
    
    /// Trigger when value remains stable for N seconds
    Stable {
        register: String,
        seconds: u64,
    },
}

impl Trigger {
    /// Get the register address this trigger monitors
    pub fn register(&self) -> &str {
        match self {
            Trigger::Compare { register, .. } => register,
            Trigger::Changed { register } => register,
            Trigger::BecameOn { register } => register,
            Trigger::BecameOff { register } => register,
            Trigger::Stable { register, .. } => register,
        }
    }

    /// Check if a register value satisfies this trigger's condition.
    ///
    /// - `register_value`: the current value read from the register (u16)
    /// - `trigger_value`: the threshold / target value defined in the trigger (u16)
    ///
    /// Returns `true` if the condition is satisfied, `false` otherwise.
    /// For comparison triggers, compares `register_value <operator> trigger_value`.
    /// For non-comparison triggers (Changed, BecameOn, BecameOff, Stable), returns `false`
    /// since they require additional state/context that cannot be determined from
    /// a single pair of values.
    pub fn check_trigger(&self, register_value: u16, trigger_value: u16) -> bool {
        match self {
            Trigger::Compare { operator, .. } => {
                let reg = register_value as i64;
                let tgt = trigger_value as i64;
                operator.evaluate(reg, tgt)
            }
            // Non-comparison triggers need additional context (previous value, timestamps)
            // and cannot be evaluated from a single pair of values alone.
            Trigger::Changed { .. } => false,
            Trigger::BecameOn { .. } => false,
            Trigger::BecameOff { .. } => false,
            Trigger::Stable { .. } => false,
        }
    }
    
    /// Human-readable description of the trigger
    pub fn description(&self) -> String {
        match self {
            Trigger::Compare { register, operator, value } => {
                format!("{} {} {}", register, operator.symbol(), value)
            }
            Trigger::Changed { register } => {
                format!("{} changed", register)
            }
            Trigger::BecameOn { register } => {
                format!("{} became ON", register)
            }
            Trigger::BecameOff { register } => {
                format!("{} became OFF", register)
            }
            Trigger::Stable { register, seconds } => {
                format!("{} stable for {}s", register, seconds)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compare_op_evaluate() {
        assert!(CompareOp::GT.evaluate(10, 5));
        assert!(!CompareOp::GT.evaluate(5, 10));
        assert!(CompareOp::LT.evaluate(5, 10));
        assert!(CompareOp::EQ.evaluate(10, 10));
        assert!(CompareOp::NEQ.evaluate(10, 5));
        assert!(CompareOp::GTE.evaluate(10, 10));
        assert!(CompareOp::LTE.evaluate(10, 10));
    }
    
    #[test]
    fn test_compare_op_symbol() {
        assert_eq!(CompareOp::GT.symbol(), ">");
        assert_eq!(CompareOp::LT.symbol(), "<");
        assert_eq!(CompareOp::EQ.symbol(), "=");
        assert_eq!(CompareOp::NEQ.symbol(), "≠");
        assert_eq!(CompareOp::GTE.symbol(), "≥");
        assert_eq!(CompareOp::LTE.symbol(), "≤");
    }
    
    #[test]
    fn test_trigger_description() {
        let t1 = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::GT,
            value: 30,
        };
        assert_eq!(t1.description(), "40001 > 30");
        
        let t2 = Trigger::BecameOn {
            register: "00001".to_string(),
        };
        assert_eq!(t2.description(), "00001 became ON");
    }
    
    #[test]
    fn test_check_trigger_gt() {
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::GT,
            value: 30,
        };
        // register_value > trigger_value
        assert!(trigger.check_trigger(31, 30));   // 31 > 30 → true
        assert!(!trigger.check_trigger(30, 30));  // 30 > 30 → false
        assert!(!trigger.check_trigger(29, 30));  // 29 > 30 → false
    }
    
    #[test]
    fn test_check_trigger_lt() {
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::LT,
            value: 100,
        };
        // register_value < trigger_value
        assert!(trigger.check_trigger(99, 100));   // 99 < 100 → true
        assert!(!trigger.check_trigger(100, 100)); // 100 < 100 → false
        assert!(!trigger.check_trigger(101, 100)); // 101 < 100 → false
    }
    
    #[test]
    fn test_check_trigger_eq() {
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::EQ,
            value: 42,
        };
        // register_value == trigger_value
        assert!(trigger.check_trigger(42, 42));    // 42 == 42 → true
        assert!(!trigger.check_trigger(41, 42));  // 41 == 42 → false
        assert!(!trigger.check_trigger(43, 42));  // 43 == 42 → false
    }
    
    #[test]
    fn test_check_trigger_neq() {
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::NEQ,
            value: 0,
        };
        // register_value != trigger_value
        assert!(trigger.check_trigger(1, 0));     // 1 != 0 → true
        assert!(!trigger.check_trigger(0, 0));    // 0 != 0 → false
    }
    
    #[test]
    fn test_check_trigger_gte() {
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::GTE,
            value: 80,
        };
        // register_value >= trigger_value
        assert!(trigger.check_trigger(80, 80));    // 80 >= 80 → true
        assert!(trigger.check_trigger(81, 80));    // 81 >= 80 → true
        assert!(!trigger.check_trigger(79, 80));  // 79 >= 80 → false
    }
    
    #[test]
    fn test_check_trigger_lte() {
        let trigger = Trigger::Compare {
            register: "40001".to_string(),
            operator: CompareOp::LTE,
            value: 20,
        };
        // register_value <= trigger_value
        assert!(trigger.check_trigger(20, 20));    // 20 <= 20 → true
        assert!(trigger.check_trigger(19, 20));    // 19 <= 20 → true
        assert!(!trigger.check_trigger(21, 20));   // 21 <= 20 → false
    }
    
    #[test]
    fn test_check_trigger_non_comparison() {
        // Non-comparison triggers (Changed, BecameOn, BecameOff, Stable)
        // require additional state/context, so check_trigger returns false
        let changed = Trigger::Changed { register: "40001".to_string() };
        assert!(!changed.check_trigger(1, 0));
        
        let became_on = Trigger::BecameOn { register: "00001".to_string() };
        assert!(!became_on.check_trigger(1, 0));
        
        let became_off = Trigger::BecameOff { register: "00001".to_string() };
        assert!(!became_off.check_trigger(0, 1));
        
        let stable = Trigger::Stable { register: "40001".to_string(), seconds: 10 };
        assert!(!stable.check_trigger(100, 100));
    }
    
    #[test]
    fn test_check_trigger_edge_cases() {
        // Boundary values for u16
        let gt_trigger = Trigger::Compare { register: "0".into(), operator: CompareOp::GT, value: 0 };
        assert!(gt_trigger.check_trigger(u16::MAX, 0));    // max > 0
        assert!(!gt_trigger.check_trigger(0, 0));          // 0 > 0
        
        let lt_trigger = Trigger::Compare { register: "0".into(), operator: CompareOp::LT, value: u16::MAX as i64 };
        assert!(lt_trigger.check_trigger(0, u16::MAX));    // 0 < max
        assert!(!lt_trigger.check_trigger(u16::MAX, u16::MAX)); // max < max
        
        let eq_trigger = Trigger::Compare { register: "0".into(), operator: CompareOp::EQ, value: 0 };
        assert!(eq_trigger.check_trigger(0, 0));           // 0 == 0
        
        let neq_trigger = Trigger::Compare { register: "0".into(), operator: CompareOp::NEQ, value: 0 };
        assert!(neq_trigger.check_trigger(1, 0));           // 1 != 0
    }
