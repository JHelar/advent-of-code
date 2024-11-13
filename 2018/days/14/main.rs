fn read_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    line.trim().to_string()
}

fn print_scores(scores: &Vec<usize>, elf_1: usize, elf_2: usize) {
    let score_line = scores
        .iter()
        .enumerate()
        .map(|(index, score)| {
            if index == elf_1 {
                format!("({score})")
            } else if index == elf_2 {
                format!("[{score}]")
            } else {
                format!(" {score} ")
            }
        })
        .collect::<String>();

    println!("{score_line}")
}

fn set_score(scores: &mut Vec<usize>, elf_1: &mut usize, elf_2: &mut usize) -> Vec<usize> {
    let score = scores[*elf_1] + scores[*elf_2];
    let mut added_scores = Vec::new();

    if score >= 10 {
        let score_1 = score / 10;
        let score_2 = score % 10;
        scores.push(score_1);
        scores.push(score_2);

        added_scores.push(score_1);
        added_scores.push(score_2);
    } else {
        scores.push(score);
        added_scores.push(score);
    }

    *elf_1 = (*elf_1 + (scores[*elf_1] + 1)) % scores.len();
    *elf_2 = (*elf_2 + (scores[*elf_2] + 1)) % scores.len();

    added_scores
}

fn part1() -> Option<String> {
    const RECIPE_COUNT: usize = 10;
    let desired_recipes = read_input().parse::<usize>().unwrap();

    let mut scores = vec![3, 7];
    let mut elf_1: usize = 0;
    let mut elf_2: usize = 1;

    while scores.len() < (desired_recipes + RECIPE_COUNT) {
        set_score(&mut scores, &mut elf_1, &mut elf_2);
    }

    Some(
        scores
            .into_iter()
            .skip(desired_recipes)
            .take(RECIPE_COUNT)
            .map(|score| score.to_string())
            .collect::<String>(),
    )
}

fn part2() -> Option<String> {
    let search_for_recipes = read_input();

    let mut scores = vec![3, 7];
    let mut elf_1: usize = 0;
    let mut elf_2: usize = 1;

    for _ in 0.. {
        set_score(&mut scores, &mut elf_1, &mut elf_2);

        if scores.len() < search_for_recipes.len() + 1 {
            continue;
        }

        let tail = scores[scores.len() - search_for_recipes.len() - 1..]
            .iter()
            .map(|score| score.to_string())
            .collect::<String>();

        if tail.starts_with(&search_for_recipes) {
            return Some((scores.len() - search_for_recipes.len() - 1).to_string())
        }
        if tail.ends_with(&search_for_recipes) {
            return Some((scores.len() - search_for_recipes.len()).to_string())
        }
    }

    None
}

fn main() {
    println!("--- Day 14: Chocolate Charts ---");
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
