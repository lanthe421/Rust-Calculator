// State Model
use crate::operation::Operation;

#[derive(Clone)]
pub struct CalculatorState {
    pub display: String,
    pub stored_value: Option<f64>,
    pub current_operation: Option<Operation>,
    pub waiting_for_operand: bool,
    pub error: Option<String>,
    pub fresh_start: bool,  // True when in initial state or after clear
}

impl CalculatorState {
    pub fn new() -> Self {
        Self {
            display: String::from("0"),
            stored_value: None,
            current_operation: None,
            waiting_for_operand: false,
            error: None,
            fresh_start: true,
        }
    }
}
