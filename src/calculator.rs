// Calculator Logic Layer
use crate::state::CalculatorState;
use crate::operation::Operation;

#[derive(Clone)]
pub struct Calculator {
    state: CalculatorState,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            state: CalculatorState::new(),
        }
    }

    pub fn input_digit(&mut self, digit: u8) {
        // Block input if there's an error (Requirement 5.2)
        if self.state.error.is_some() {
            return;
        }

        // Validate digit is 0-9
        if digit > 9 {
            return;
        }

        // If waiting for a new operand or fresh start, replace display (Requirement 1.3)
        if self.state.waiting_for_operand || self.state.fresh_start {
            self.state.display = digit.to_string();
            self.state.waiting_for_operand = false;
            self.state.fresh_start = false;
        } else {
            // Append to accumulate digits (Requirements 1.1, 1.4)
            self.state.display.push_str(&digit.to_string());
        }
    }

    pub fn input_decimal_point(&mut self) {
        // Block input if there's an error
        if self.state.error.is_some() {
            return;
        }

        // If waiting for a new operand or fresh start, start with "0."
        if self.state.waiting_for_operand || self.state.fresh_start {
            self.state.display = String::from("0.");
            self.state.waiting_for_operand = false;
            self.state.fresh_start = false;
            return;
        }

        // Only add decimal point if one doesn't already exist (Requirement 1.2)
        if !self.state.display.contains('.') {
            self.state.display.push('.');
        }
    }

    pub fn input_operation(&mut self, op: Operation) {
        // Block input if there's an error (Requirement 5.2)
        if self.state.error.is_some() {
            return;
        }

        // If we have invalid input (empty display or just a decimal point), ignore (Requirement 5.3)
        if self.state.display.is_empty() || self.state.display == "." {
            return;
        }

        // Parse current display value
        let current_value = match self.state.display.parse::<f64>() {
            Ok(val) => val,
            Err(_) => return, // Invalid input, ignore (Requirement 5.3)
        };

        // If we already have a stored operation, calculate it first (chain operations)
        if let (Some(stored), Some(prev_op)) = (self.state.stored_value, self.state.current_operation) {
            // Only calculate if we're not waiting for operand (i.e., user entered a new number)
            if !self.state.waiting_for_operand {
                match prev_op.apply(stored, current_value) {
                    Ok(result) => {
                        self.state.display = result.to_string();
                        self.state.stored_value = Some(result);
                    }
                    Err(err) => {
                        self.state.error = Some(err);
                        return;
                    }
                }
            }
        } else {
            // No previous operation, just store the current value
            self.state.stored_value = Some(current_value);
        }

        // Store the new operation (Requirement 2.1)
        self.state.current_operation = Some(op);
        self.state.waiting_for_operand = true;
    }

    pub fn calculate(&mut self) {
        // Block if there's an error (Requirement 5.2)
        if self.state.error.is_some() {
            return;
        }

        // Need both a stored value and an operation to calculate
        let stored = match self.state.stored_value {
            Some(val) => val,
            None => return, // Nothing to calculate
        };

        let operation = match self.state.current_operation {
            Some(op) => op,
            None => return, // No operation to perform
        };

        // Get current value from display (Requirement 2.2)
        let current_value = match self.state.display.parse::<f64>() {
            Ok(val) => val,
            Err(_) => return, // Invalid display value
        };

        // Apply the operation (Requirements 2.2, 5.1)
        match operation.apply(stored, current_value) {
            Ok(result) => {
                // Check for overflow/infinity
                if result.is_infinite() || result.is_nan() {
                    self.state.error = Some(String::from("Error: Overflow"));
                } else {
                    // Display result on the display
                    self.state.display = result.to_string();
                    // Store result for potential chaining
                    self.state.stored_value = Some(result);
                    // Clear the operation
                    self.state.current_operation = None;
                    // Set waiting flag so next digit starts fresh
                    self.state.waiting_for_operand = true;
                }
            }
            Err(err) => {
                // Handle errors like division by zero (Requirement 5.1)
                self.state.error = Some(err);
            }
        }
    }

    pub fn clear(&mut self) {
        // Reset all state fields to initial values (Requirements 3.1, 3.2)
        self.state = CalculatorState::new();
    }

    pub fn get_display_text(&self) -> String {
        if let Some(ref error) = self.state.error {
            error.clone()
        } else {
            self.state.display.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: gui-calculator, Property 1: Digit input accumulation
    // Validates: Requirements 1.1, 1.4
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_digit_input_accumulation(
            digits in prop::collection::vec(0u8..=9, 1..=10)
        ) {
            let mut calc = Calculator::new();
            
            // Build expected display string
            // The first digit replaces the initial "0" placeholder
            let mut expected = String::new();
            for &digit in &digits {
                expected.push_str(&digit.to_string());
            }
            
            // Input each digit
            for &digit in &digits {
                calc.input_digit(digit);
            }
            
            // The display should show the concatenated digits
            // (first digit replaces initial "0", subsequent digits accumulate)
            prop_assert_eq!(calc.get_display_text(), expected);
        }

        // Feature: gui-calculator, Property 2: Single decimal point constraint
        // Validates: Requirements 1.2
        #[test]
        fn test_single_decimal_point_constraint(
            digits_before in prop::collection::vec(0u8..=9, 0..=5),
            digits_after in prop::collection::vec(0u8..=9, 0..=5),
            extra_decimal_attempts in 1usize..=5
        ) {
            let mut calc = Calculator::new();
            
            // Input some digits
            for &digit in &digits_before {
                calc.input_digit(digit);
            }
            
            // Add first decimal point
            calc.input_decimal_point();
            
            // Input more digits
            for &digit in &digits_after {
                calc.input_digit(digit);
            }
            
            // Try to add decimal point multiple more times
            for _ in 0..extra_decimal_attempts {
                calc.input_decimal_point();
            }
            
            // Count decimal points in display
            let decimal_count = calc.get_display_text().matches('.').count();
            
            // Should have at most one decimal point
            prop_assert!(decimal_count <= 1, 
                "Display '{}' contains {} decimal points, expected at most 1", 
                calc.get_display_text(), decimal_count);
        }

        // Feature: gui-calculator, Property 3: Operation state preservation
        // Validates: Requirements 2.1
        #[test]
        fn test_operation_state_preservation(
            operand_digits in prop::collection::vec(0u8..=9, 1..=5),
            operation_idx in 0usize..4
        ) {
            let operation = match operation_idx {
                0 => Operation::Add,
                1 => Operation::Subtract,
                2 => Operation::Multiply,
                _ => Operation::Divide,
            };
            let mut calc = Calculator::new();
            
            // Input the operand
            for &digit in &operand_digits {
                calc.input_digit(digit);
            }
            
            // Get the operand value before operation
            let operand_str = calc.get_display_text();
            let operand_value: f64 = operand_str.parse().unwrap();
            
            // Input the operation
            calc.input_operation(operation);
            
            // Verify the calculator stored both the operand and operation
            prop_assert_eq!(calc.state.stored_value, Some(operand_value),
                "Stored value should be {}", operand_value);
            prop_assert_eq!(calc.state.current_operation, Some(operation),
                "Current operation should be {:?}", operation);
            prop_assert!(calc.state.waiting_for_operand,
                "Calculator should be waiting for next operand");
        }

        // Feature: gui-calculator, Property 6: Post-calculation continuation
        // Validates: Requirements 3.3
        #[test]
        fn test_post_calculation_continuation(
            left_digits in prop::collection::vec(1u8..=9, 1..=3),
            right_digits in prop::collection::vec(1u8..=9, 1..=3),
            operation_idx in 0usize..3, // Avoid division to prevent errors
            continuation_digit in 0u8..=9,
            continuation_op_idx in 0usize..4
        ) {
            let operation = match operation_idx {
                0 => Operation::Add,
                1 => Operation::Subtract,
                _ => Operation::Multiply,
            };
            
            let continuation_op = match continuation_op_idx {
                0 => Operation::Add,
                1 => Operation::Subtract,
                2 => Operation::Multiply,
                _ => Operation::Divide,
            };

            let mut calc = Calculator::new();
            
            // Input first operand
            for &digit in &left_digits {
                calc.input_digit(digit);
            }
            
            // Input operation
            calc.input_operation(operation);
            
            // Input second operand
            for &digit in &right_digits {
                calc.input_digit(digit);
            }
            
            // Calculate
            calc.calculate();
            
            // Should not be in error state
            prop_assert!(calc.state.error.is_none(), "Should not have error after calculation");
            
            // Get the result
            let result_str = calc.get_display_text();
            let result: f64 = result_str.parse().unwrap();
            
            // Test 1: Start new calculation from scratch by entering a digit
            let mut calc_fresh = calc.clone();
            calc_fresh.input_digit(continuation_digit);
            
            // The display should show just the new digit (fresh start)
            prop_assert_eq!(calc_fresh.get_display_text(), continuation_digit.to_string(),
                "After calculation, entering a digit should start fresh");
            
            // Test 2: Continue with the result by entering an operation
            let mut calc_continue = calc.clone();
            calc_continue.input_operation(continuation_op);
            
            // The stored value should be the result from previous calculation
            prop_assert_eq!(calc_continue.state.stored_value, Some(result),
                "After calculation, entering operation should use previous result");
            prop_assert_eq!(calc_continue.state.current_operation, Some(continuation_op),
                "After calculation, operation should be stored");
        }

        // Feature: gui-calculator, Property 5: Clear resets state
        // Validates: Requirements 3.1, 3.2
        #[test]
        fn test_clear_resets_state(
            // Generate random sequences of operations to put calculator in various states
            digit_inputs in prop::collection::vec(0u8..=9, 0..=10),
            decimal_points in 0usize..=3,
            operation_idx in prop::option::of(0usize..4),
            should_calculate in prop::bool::ANY,
            should_cause_error in prop::bool::ANY
        ) {
            let mut calc = Calculator::new();
            
            // Put calculator in a random state by performing various operations
            
            // Input some digits
            for &digit in &digit_inputs {
                calc.input_digit(digit);
            }
            
            // Try adding decimal points
            for _ in 0..decimal_points {
                calc.input_decimal_point();
            }
            
            // Maybe input an operation
            if let Some(op_idx) = operation_idx {
                let operation = match op_idx {
                    0 => Operation::Add,
                    1 => Operation::Subtract,
                    2 => Operation::Multiply,
                    _ => Operation::Divide,
                };
                calc.input_operation(operation);
                
                // Input more digits for second operand
                if !digit_inputs.is_empty() {
                    calc.input_digit(digit_inputs[0]);
                }
                
                // Maybe calculate
                if should_calculate {
                    calc.calculate();
                }
            }
            
            // Maybe cause an error by dividing by zero
            if should_cause_error && !digit_inputs.is_empty() {
                calc.input_digit(digit_inputs[0]);
                calc.input_operation(Operation::Divide);
                calc.input_digit(0);
                calc.calculate();
            }
            
            // Now clear the calculator
            calc.clear();
            
            // Create a fresh calculator for comparison
            let fresh_calc = Calculator::new();
            
            // Verify all state fields are reset to initial values
            prop_assert_eq!(calc.get_display_text(), fresh_calc.get_display_text(),
                "Display should be reset to initial value");
            prop_assert_eq!(calc.state.stored_value, None,
                "Stored value should be None after clear");
            prop_assert_eq!(calc.state.current_operation, None,
                "Current operation should be None after clear");
            prop_assert_eq!(calc.state.waiting_for_operand, false,
                "waiting_for_operand should be false after clear");
            prop_assert_eq!(calc.state.error, None,
                "Error should be None after clear");
            prop_assert_eq!(calc.state.fresh_start, true,
                "fresh_start should be true after clear");
        }

        // Feature: gui-calculator, Property 8: Error state blocks operations
        // Validates: Requirements 5.2
        #[test]
        fn test_error_state_blocks_operations(
            initial_digits in prop::collection::vec(1u8..=9, 1..=3),
            blocked_digit in 0u8..=9,
            blocked_op_idx in 0usize..4
        ) {
            let blocked_operation = match blocked_op_idx {
                0 => Operation::Add,
                1 => Operation::Subtract,
                2 => Operation::Multiply,
                _ => Operation::Divide,
            };

            let mut calc = Calculator::new();
            
            // Set up a calculation that will cause an error (division by zero)
            for &digit in &initial_digits {
                calc.input_digit(digit);
            }
            calc.input_operation(Operation::Divide);
            calc.input_digit(0);
            calc.calculate();
            
            // Verify we're in an error state
            prop_assert!(calc.state.error.is_some(), "Calculator should be in error state");
            let error_message = calc.get_display_text();
            prop_assert!(error_message.contains("Error"), "Display should show error message");
            
            // Try to input a digit - should be blocked
            calc.input_digit(blocked_digit);
            prop_assert_eq!(calc.get_display_text(), error_message.clone(),
                "Display should still show error after digit input attempt");
            
            // Try to input a decimal point - should be blocked
            calc.input_decimal_point();
            prop_assert_eq!(calc.get_display_text(), error_message.clone(),
                "Display should still show error after decimal point attempt");
            
            // Try to input an operation - should be blocked
            calc.input_operation(blocked_operation);
            prop_assert_eq!(calc.get_display_text(), error_message.clone(),
                "Display should still show error after operation attempt");
            
            // Try to calculate - should be blocked
            calc.calculate();
            prop_assert_eq!(calc.get_display_text(), error_message,
                "Display should still show error after calculate attempt");
            
            // Verify error state is maintained
            prop_assert!(calc.state.error.is_some(),
                "Error state should be maintained until clear");
        }

        // Feature: gui-calculator, Property 9: Invalid input preservation
        // Validates: Requirements 5.3
        #[test]
        fn test_invalid_input_preservation(
            valid_digits in prop::collection::vec(1u8..=9, 1..=3),
            operation_idx in 0usize..4,
            num_invalid_ops in 1usize..=5
        ) {
            let operation = match operation_idx {
                0 => Operation::Add,
                1 => Operation::Subtract,
                2 => Operation::Multiply,
                _ => Operation::Divide,
            };

            let mut calc = Calculator::new();
            
            // Input valid digits to establish a valid state
            for &digit in &valid_digits {
                calc.input_digit(digit);
            }
            
            // Input a valid operation
            calc.input_operation(operation);
            
            // Capture the valid state
            let valid_display = calc.get_display_text();
            let valid_stored = calc.state.stored_value;
            let valid_operation = calc.state.current_operation;
            let valid_waiting = calc.state.waiting_for_operand;
            
            // Try to input multiple operations in a row without operands (invalid sequence)
            for _ in 0..num_invalid_ops {
                calc.input_operation(operation);
            }
            
            // Verify the state is preserved (invalid inputs were ignored)
            prop_assert_eq!(calc.get_display_text(), valid_display,
                "Display should be preserved after invalid operation sequence");
            prop_assert_eq!(calc.state.stored_value, valid_stored,
                "Stored value should be preserved after invalid operation sequence");
            prop_assert_eq!(calc.state.current_operation, valid_operation,
                "Current operation should be preserved after invalid operation sequence");
            prop_assert_eq!(calc.state.waiting_for_operand, valid_waiting,
                "waiting_for_operand flag should be preserved after invalid operation sequence");
            prop_assert!(calc.state.error.is_none(),
                "No error should be set for invalid input sequences");
        }

        // Feature: gui-calculator, Property 7: Number formatting consistency
        // Validates: Requirements 4.3
        #[test]
        fn test_number_formatting_consistency(
            integer_part in prop::option::of(0u32..=999999),
            has_decimal in prop::bool::ANY,
            decimal_digits in prop::collection::vec(0u8..=9, 0..=6)
        ) {
            let mut calc = Calculator::new();
            
            // Build a number with optional integer and decimal parts
            if let Some(int_val) = integer_part {
                // Input integer part
                let int_str = int_val.to_string();
                for ch in int_str.chars() {
                    if let Some(digit) = ch.to_digit(10) {
                        calc.input_digit(digit as u8);
                    }
                }
            }
            
            // Add decimal point if requested
            if has_decimal {
                calc.input_decimal_point();
                
                // Add decimal digits
                for &digit in &decimal_digits {
                    calc.input_digit(digit);
                }
            }
            
            let display = calc.get_display_text();
            
            // Property: The display should be parseable as a valid number (or be "0")
            if display != "0" && display != "0." {
                let parse_result = display.parse::<f64>();
                prop_assert!(parse_result.is_ok(), 
                    "Display '{}' should be parseable as a number", display);
                
                // Property: Should not have unnecessary leading zeros (except "0" or "0.x")
                if !display.starts_with("0.") && display != "0" {
                    prop_assert!(!display.starts_with('0'),
                        "Display '{}' should not have unnecessary leading zeros", display);
                }
            }
            
            // Property: Should have at most one decimal point
            let decimal_count = display.matches('.').count();
            prop_assert!(decimal_count <= 1,
                "Display '{}' should have at most one decimal point", display);
            
            // Property: Display should be consistent - same input sequence produces same output
            let mut calc2 = Calculator::new();
            if let Some(int_val) = integer_part {
                let int_str = int_val.to_string();
                for ch in int_str.chars() {
                    if let Some(digit) = ch.to_digit(10) {
                        calc2.input_digit(digit as u8);
                    }
                }
            }
            if has_decimal {
                calc2.input_decimal_point();
                for &digit in &decimal_digits {
                    calc2.input_digit(digit);
                }
            }
            
            prop_assert_eq!(calc.get_display_text(), calc2.get_display_text(),
                "Same input sequence should produce consistent display");
        }
    }
}
