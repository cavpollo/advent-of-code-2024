#[macro_export]
macro_rules! execute_measuring_time {
    ($func:expr) => {
        let start = Instant::now();

        $func;

        let duration = start.elapsed();
        println!("Execution time: {:?}", duration);
    };
}
