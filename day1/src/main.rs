fn scan_report(report: Vec<usize>) -> usize {
    let mut count = 0;
    let mut iter = report.iter().peekable();
    while let Some(&depth) = iter.next() {
        if iter.peek() > Some(&&depth) {
            count += 1;
        }
    }
    count
}

fn measure_of_three(report: Vec<usize>) -> usize {
    let mut count = 0;
    let mut current = 0;
    for window in report.windows(3) {
        let sum = window.iter().sum();

        if current == 0 {
            current = sum;
            continue;
        }

        if sum > current {
            count += 1;
        }
        current = sum;
    }
    count
}

fn main() {
    let mut input: Vec<usize> = Vec::new();
    for depth in include_str!("../input.txt").lines() {
        input.push(depth.parse::<usize>().unwrap());
    }

    println!("depth increased {}", scan_report(input.clone()));

    input.clear();

    for depth in include_str!("../input2.txt").lines() {
        input.push(depth.parse::<usize>().unwrap());
    }

    println!("measure of three increased {}", measure_of_three(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_report() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(scan_report(input), 7);
    }

    #[test]
    fn test_measure_of_three() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(measure_of_three(input), 5);
    }
}
