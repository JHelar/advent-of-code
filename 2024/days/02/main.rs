use std::collections::HashSet;

fn read_input() -> Vec<Vec<isize>> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_failed_indices(report: &Vec<isize>) -> Vec<usize> {
    let ordering = report.first().unwrap().cmp(report.last().unwrap());
    let mut failed_index = HashSet::new();

    for i in 0..report.len() {
        match (report[i], report.get(i + 1)) {
            (level, Some(next_level)) => {
                let diff = (level - next_level).abs();
                if level.cmp(next_level) != ordering || diff < 1 || diff > 3 {
                    failed_index.insert(i);
                    failed_index.insert(i + 1);
                }
            }
            (_, None) => {}
        }
    }
    failed_index.into_iter().collect()
}

fn part1() -> Option<usize> {
    let reports = read_input();
    let safe_reports = reports.iter().filter(|report| get_failed_indices(&report).len() == 0).count();
    Some(safe_reports)
}

fn part2() -> Option<usize> {
    let reports = read_input();
    let mut safe_reports = 0;
    for report in reports {
        let failed_indices = get_failed_indices(&report);
        if failed_indices.len() == 0 {
            safe_reports += 1;
        } else {
            for skip_index in failed_indices.iter() {
                let modified_report = report
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| if *skip_index == i { None } else { Some(*v) })
                    .collect();

                if get_failed_indices(&modified_report).len() == 0 {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    Some(safe_reports)
}

fn main() {
    println!("--- Day 2: Red-Nosed Reports ---");
    if let Some(part) = std::env::args().skip(1).next() {
        if let Some(result) = match part.as_str() {
            "1" => part1(),
            "2" => part2(),
            _ => panic!("💥 Invalid part number: {part}"),
        } {
            println!("🎁 Result part {part}: {result}");
        }
    } else {
        if let Some(result_1) = part1() {
            println!("🎁 Result part 1: {result_1}");
        }
        if let Some(result_2) = part2() {
            println!("🎁 Result part 2: {result_2}");
        }
    }
}
