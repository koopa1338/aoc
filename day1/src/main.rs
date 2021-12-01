fn scan_report(report: Vec<usize>) -> usize {
    report.windows(2).filter(|window| &window[0] < &window[1]).count()
}

fn measure_of_three(report: Vec<usize>) -> usize {
    let sums = report.windows(3).map(|w| w.iter().sum()).collect::<Vec<usize>>();
    scan_report(sums)
}

fn main() {
    let mut input: Vec<usize> = include_str!("../input.txt").lines().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    println!("depth increased {}", scan_report(input.clone()));

    input.clear();

    input = include_str!("../input2.txt").lines().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<usize>>();

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
