//! Modbus Automation Script Module
//! 
//! Simple trigger-action automation scripts for Modbus devices

pub mod trigger;
pub mod action;
pub mod script;
pub mod engine;

pub use trigger::{Trigger, CompareOp};
pub use action::{Action, SoundType};
pub use script::Script;
pub use engine::ScriptEngine;
