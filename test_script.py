import time
import rust_accelerator  # This is your compiled Rust code!

print("Starting heavy calculation using Native Rust via Python...")

python_start_timer = time.time()

# Call the Rust function exactly as if it were a normal Python function
maximum_limit = 5_000_000
resulting_primes_list = rust_accelerator.calculate_primes_up_to(maximum_limit)

python_end_timer = time.time()
total_execution_duration = python_end_timer - python_start_timer

print(f"✅ Rust Module found {len(resulting_primes_list)} prime numbers!")
print(f"⏱️ Time taken: {total_execution_duration:.4f} seconds")