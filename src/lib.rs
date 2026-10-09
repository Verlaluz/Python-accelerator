use pyo3::prelude::*;

/// This #[pyfunction] macro tells Rust to automatically generate the Python bindings for this function
#[pyfunction]
fn calculate_primes_up_to(maximum_search_limit: u32) -> PyResult<Vec<u32>> {
    let mut discovered_prime_numbers: Vec<u32> = Vec::new();
    
    for current_number_to_test in 2..=maximum_search_limit {
        let mut is_prime_flag = true;
        
        // Calculate the mathematical square root to limit our inner loop
        let upper_bound_limit = (current_number_to_test as f64).sqrt() as u32;
        
        for potential_divisor in 2..=upper_bound_limit {
            let remainder_value = current_number_to_test % potential_divisor;
            if remainder_value == 0 {
                is_prime_flag = false;
                break; // Exit the inner loop early
            }
        }
        
        if is_prime_flag {
            discovered_prime_numbers.push(current_number_to_test);
        }
    }
    
    // PyO3 requires we wrap the successful result in an Ok()
    Ok(discovered_prime_numbers)
}

/// This #[pymodule] macro defines the actual Python module that gets imported in Python scripts
#[pymodule]
fn rust_accelerator(_python_interpreter_token: Python, target_python_module: &PyModule) -> PyResult<()> {
    // Add our Rust function to the Python module
    target_python_module.add_function(wrap_pyfunction!(calculate_primes_up_to, target_python_module)?)?;
    Ok(())
}