use simple_calculator::{Calculator, Operator};

use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct WasmCalculator {
    calculator: Calculator,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl WasmCalculator {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        WasmCalculator {
            calculator: Calculator::new(),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter, js_name=activeOperator))]
    pub fn active_operator(&self) -> Option<char> {
        self.calculator
            .active_operator()
            .and_then(|operator| Some(operator.id.to_owned()))
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter, js_name=showAllClear))]
    pub fn show_all_clear(&self) -> bool {
        self.calculator.cleared()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn output(&mut self) -> String {
        self.calculator.output().to_string()
    }

    #[cfg_attr(target_arch="wasm32", wasm_bindgen(js_name=buttonPressed))]
    pub fn button_pressed(&mut self, id: &str) {
        // If a number was pressed.
        if let Ok(number) = id.parse::<u8>() {
            self.calculator.submit_number(number);
            return;
        }

        // If an operator was pressed.
        if let Ok(operator) = Operator::try_from(id) {
            self.calculator.submit_operator(operator);
        }

        match id {
            "." => {
                self.calculator.submit_decimal();
            }
            "=" => {
                self.calculator.submit_equals();
            }
            "±" => {
                self.calculator.submit_negative();
            }
            "%" => {
                self.calculator.submit_percentage();
            }
            "c" => {
                self.calculator.clear();
            }
            _ => log("Unknown button pressed."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    // Allows a sequence of calcualator presses to be executed,
    // using a new Calculator instance.
    macro_rules! calc {
        ( $( $x:expr ),* ) => {
            {
                #[allow(unused_mut)]
                let mut calc = WasmCalculator::new();
                $(
                    calc.button_pressed($x);
                )*
                calc.output()
            }
        };
    }

    #[wasm_bindgen_test]
    fn initial_result() {
        assert_eq!(calc!(), "0");
    }

    #[wasm_bindgen_test]
    fn shows_ac() {
        let mut calc = WasmCalculator::new();
        assert!(calc.show_all_clear());
        calc.button_pressed("3");
        calc.button_pressed("+");
        calc.button_pressed("c");
        assert!(calc.show_all_clear());
    }

    #[wasm_bindgen_test]
    fn shows_c() {
        let mut calc = WasmCalculator::new();
        calc.button_pressed("3");
        assert!(!calc.show_all_clear());
    }

    #[wasm_bindgen_test]
    fn active_operator() {
        let mut calc = WasmCalculator::new();
        calc.button_pressed("+");
        assert_eq!(calc.active_operator(), Some('+'));
    }

    #[wasm_bindgen_test]
    fn sign_function() {
        assert_eq!(calc!("1", "±"), "-1");
    }

    #[wasm_bindgen_test]
    fn sign_function_initial() {
        assert_eq!(calc!("±", "1"), "-1");
    }

    #[wasm_bindgen_test]
    fn percentage_function() {
        assert_eq!(calc!("1", "%"), "0.01");
    }

    #[wasm_bindgen_test]
    fn outputs_last_number_input_a() {
        assert_eq!(calc!("1", "+", "2"), "2");
    }

    #[wasm_bindgen_test]
    fn outputs_last_number_input_b() {
        assert_eq!(calc!("1", "+", "2", "=", "3"), "3");
    }

    #[wasm_bindgen_test]
    fn add() {
        assert_eq!(calc!("1", "+", "2", "="), "3");
    }

    #[wasm_bindgen_test]
    fn add_negative() {
        assert_eq!(calc!("1", "+", "2", "±", "="), "-1");
    }

    #[wasm_bindgen_test]
    fn substract() {
        assert_eq!(calc!("3", "-", "2", "="), "1");
    }

    #[wasm_bindgen_test]
    fn divide() {
        assert_eq!(calc!("2", "/", "2", "="), "1");
    }

    #[wasm_bindgen_test]
    fn multiply() {
        assert_eq!(calc!("2", "*", "2", "="), "4");
    }

    #[wasm_bindgen_test]
    fn multiply_two_goes() {
        assert_eq!(calc!("2", "*", "2", "=", "1", "*", "3", "="), "3");
    }

    #[wasm_bindgen_test]
    fn clear() {
        assert_eq!(calc!("3", "c"), "0");
    }

    #[wasm_bindgen_test]
    fn memory() {
        assert_eq!(calc!("2", "*", "2", "=", "="), "8");
    }

    #[wasm_bindgen_test]
    fn memory_after_clear() {
        assert_eq!(calc!("2", "*", "2", "=", "c", "3", "="), "6");
    }

    #[wasm_bindgen_test]
    fn decimalise() {
        assert_eq!(calc!("1", "."), "1.");
    }

    #[wasm_bindgen_test]
    fn no_double_decimal() {
        assert_eq!(calc!("1", ".", "."), "1.");
    }

    #[wasm_bindgen_test]
    fn decimal_submits_zero() {
        assert_eq!(calc!("1", "+", "."), "0.");
    }

    #[wasm_bindgen_test]
    fn formats_thousands() {
        assert_eq!(calc!("1", "0", "0", "0"), "1,000");
    }

    #[wasm_bindgen_test]
    fn formats_millions() {
        assert_eq!(calc!("1", "0", "0", "0", "0", "0", "0"), "1,000,000");
    }

    #[wasm_bindgen_test]
    fn negative_zero() {
        assert_eq!(calc!("±"), "-0");
    }

    #[wasm_bindgen_test]
    fn negative_zero_decimal() {
        assert_eq!(calc!("1", "%", "±"), "-0.01");
    }

    #[wasm_bindgen_test]
    fn big_number_exponential() {
        assert_eq!(
            calc!("9", "9", "9", "9", "9", "9", "9", "9", "9", "+", "1", "="),
            "1e9"
        );
    }

    #[wasm_bindgen_test]
    fn big_number_exponential_truncation_a() {
        assert_eq!(
            calc!("1", "2", "3", "4", "5", "6", "7", "8", "9", "*", "10", "="),
            "1.234567e9"
        );
    }

    #[wasm_bindgen_test]
    fn big_number_exponential_truncation_b() {
        assert_eq!(
            calc!("1", "2", "3", "4", "5", "6", "7", "8", "9", "*", "100", "="),
            "1.23456e10"
        );
    }

    #[wasm_bindgen_test]
    fn big_neg_number_exponential_truncation() {
        assert_eq!(
            calc!("-", "1", "2", "3", "4", "5", "6", "7", "8", "9", "*", "100", "="),
            "-1.23456e10"
        );
    }

    #[wasm_bindgen_test]
    fn calculate_on_new_operator() {
        assert_eq!(calc!("1", "+", "2", "+"), "3");
    }

    #[wasm_bindgen_test]
    fn order_of_ops_new_operator() {
        let mut calc = WasmCalculator::new();
        assert!(calc.show_all_clear());
        calc.button_pressed("1");
        calc.button_pressed("+");
        calc.button_pressed("2");
        calc.button_pressed("*");
        assert_eq!(calc.output(), "2");
        assert_eq!(calc.active_operator(), Some('*'));
    }

    #[wasm_bindgen_test]
    fn order_of_ops_calculation() {
        assert_eq!(calc!("1", "+", "2", "*", "3", "="), "7");
    }
}
