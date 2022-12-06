pub fn timing<R, T>(fun: R, part: u8)
where
    R: FnOnce() -> T,
    T: std::fmt::Display,
{
    println!("\tPart {part}:");
    let now = std::time::Instant::now();
    println!("\tSolution: {}", fun());
    println!("\t{}", "-".repeat(20));
    println!("\tTiming: {}s", now.elapsed().as_secs_f64());
    println!();
}
