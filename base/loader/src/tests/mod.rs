#![cfg(test)]
use crate::ldrprintln;

pub fn test_runner(tests: &[&dyn Fn()]) {

    ldrprintln!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
