#[macro_export]
macro_rules! execute_measuring_time {
    ($func:expr) => {{
        let start = std::time::Instant::now();

        let response = $func();

        let duration = start.elapsed();
        println!("Execution time: {:?}", duration);

        response
    }};
}
