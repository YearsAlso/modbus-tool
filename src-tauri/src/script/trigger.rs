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
}
