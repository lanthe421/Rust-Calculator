// Operation Enum

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn apply(&self, left: f64, right: f64) -> Result<f64, String> {
        match self {
            Operation::Add => Ok(left + right),
            Operation::Subtract => Ok(left - right),
            Operation::Multiply => Ok(left * right),
            Operation::Divide => {
                if right == 0.0 {
                    Err(String::from("Error: Division by zero"))
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: gui-calculator, Property 4: Arithmetic correctness
    // Validates: Requirements 2.2, 2.3, 2.4, 2.5, 2.6
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_arithmetic_correctness(
            left in -1000000.0..1000000.0,
            right in -1000000.0..1000000.0,
        ) {
            // Test addition
            let add_result = Operation::Add.apply(left, right);
            prop_assert!(add_result.is_ok());
            prop_assert_eq!(add_result.unwrap(), left + right);

            // Test subtraction
            let sub_result = Operation::Subtract.apply(left, right);
            prop_assert!(sub_result.is_ok());
            prop_assert_eq!(sub_result.unwrap(), left - right);

            // Test multiplication
            let mul_result = Operation::Multiply.apply(left, right);
            prop_assert!(mul_result.is_ok());
            prop_assert_eq!(mul_result.unwrap(), left * right);

            // Test division with non-zero divisor
            if right != 0.0 {
                let div_result = Operation::Divide.apply(left, right);
                prop_assert!(div_result.is_ok());
                prop_assert_eq!(div_result.unwrap(), left / right);
            }
        }

        #[test]
        fn test_division_by_zero(
            left in -1000000.0..1000000.0,
        ) {
            let result = Operation::Divide.apply(left, 0.0);
            prop_assert!(result.is_err());
            prop_assert_eq!(result.unwrap_err(), "Error: Division by zero");
        }
    }
}
