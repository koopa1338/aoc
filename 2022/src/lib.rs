use stats_alloc::{StatsAlloc, Region, INSTRUMENTED_SYSTEM};
use std::alloc::System;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub fn stats<R, T>(fun: R, part: u8)
where
    R: FnOnce() -> T,
    T: std::fmt::Display,
{
    println!("\tPart {part}:");
    let now = std::time::Instant::now();
    let mut memory = Region::new(&GLOBAL);
    println!("\tSolution: {}", fun());
    let stats = memory.change_and_reset();
    println!("\t{}", "-".repeat(20));
    println!("\tTiming: {}ms", now.elapsed().as_secs_f64() * 1000f64);
    println!("\tMemory:");
    println!("\t\tallocations: {}", stats.allocations);
    println!("\t\tallocated: {} Bytes", stats.bytes_allocated);
    println!("\t\treallocations: {}", stats.reallocations);
    println!("\t\tdeallocations: {}", stats.deallocations);
    println!("\t\tdeallocated: {} Bytes", stats.bytes_deallocated);
    println!();
}
