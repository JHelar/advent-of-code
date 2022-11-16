use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Rectangle(String, i32, i32, i32, i32);

fn parse_input() -> Vec<Rectangle> {
    let contents = fs::read_to_string("./day3.input").expect("Unable to read file!");
    let lines = contents.lines();
    let mut rectangles: Vec<Rectangle> = vec![];

    for line in lines {
        let parts = line.split(char::is_whitespace).collect::<Vec<&str>>();
        let id = parts[0];

        let coords = parts[2].split(',').collect::<Vec<&str>>();
        let size = parts[3].split('x').collect::<Vec<&str>>();

        let x = coords[0];
        let y = coords[1];

        let width = size[0];
        let height = size[1];

        rectangles.push(Rectangle(
            id.to_string(),
            x.parse::<i32>().unwrap(),
            y[0..y.len() - 1].parse::<i32>().unwrap(),
            width.parse::<i32>().unwrap(),
            height.parse::<i32>().unwrap(),
        ));
    }

    return rectangles;
}

pub fn part1() {
    let rectangles = parse_input();
    let mut fabric: HashMap<String, i32> = HashMap::new();

    for Rectangle(_, x, y, w, h) in rectangles {
        for x1 in x..x + w {
            for y1 in y..y + h {
                let coords = format!("{}x{}", x1, y1);
                let value = fabric.get(&coords).unwrap_or(&0);
                fabric.insert(coords, value + 1);
            }
        }
    }

    let mut result = 0;
    for (_, value) in fabric {
        if value > 1 {
            result += 1;
        }
    }
    println!("{}", result);
}

pub fn part2() {
    let rectangles = parse_input();
    let mut rectangle_coords: HashMap<String, Vec<String>> = HashMap::new();
    let mut fabric: HashMap<String, i32> = HashMap::new();

    for Rectangle(id, x, y, w, h) in rectangles {
        let mut coords_vec = Vec::new();
        for x1 in x..x + w {
            for y1 in y..y + h {
                let coords = format!("{}x{}", x1, y1);

                let value = fabric.get(&coords).unwrap_or(&0);
                fabric.insert(coords.clone(), value + 1);
                coords_vec.push(coords.clone());
            }
        }
        rectangle_coords.insert(id.clone(), coords_vec);
    }

    for (id, coords) in rectangle_coords {
        let mut alone = true;
        for coord in coords {
            if fabric[&coord] > 1 {
                alone = false;
                break;
            }
        }

        if alone {
            println!("Result: {}", id);
            return;
        }
    }
}
