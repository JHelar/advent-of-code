use std::collections::HashMap;
use std::env;
use std::fs;

type Cube = (i32, i32, i32);
type Droplet = HashMap<Cube, (DropletState, bool)>;

const MIN_X: i32 = -1;
const MIN_Y: i32 = -1;
const MIN_Z: i32 = -1;

const MAX_X: i32 = 21;
const MAX_Y: i32 = 21;
const MAX_Z: i32 = 21;

#[derive(Debug)]
enum DropletState {
    Air,
    Lava,
}

fn neighbors((x, y, z): &Cube) -> Vec<Cube> {
    vec![
        (*x, y - 1, *z), // top
        (*x, y + 1, *z), // bottom
        (x - 1, *y, *z), // left
        (x + 1, *y, *z), // right
        (*x, *y, z + 1), // front
        (*x, *y, z - 1), // back
    ]
}

fn find_edge_cubes(droplet: &mut Droplet, visit_cube: &Cube) {
    if visit_cube.0 < MIN_X
        || visit_cube.0 > MAX_X
        || visit_cube.1 < MIN_Y
        || visit_cube.1 > MAX_Y
        || visit_cube.2 < MIN_Z
        || visit_cube.2 > MAX_Z
    {
        return;
    }
    let state = droplet.get_mut(visit_cube).unwrap();

    // Visited
    if state.1 {
        return;
    }

    match state.0 {
        DropletState::Air => {
            state.1 = true;

            for direction in neighbors(visit_cube) {
                find_edge_cubes(droplet, &direction);
            }
        }
        DropletState::Lava => {
            state.1 = true;
        }
    }
}

fn parse_input() -> Vec<Cube> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line_split = line.split(",").map(|x| x.parse::<i32>().unwrap());
            (
                line_split.next().unwrap(),
                line_split.next().unwrap(),
                line_split.next().unwrap(),
            )
        })
        .collect()
}

fn part1() {
    let cubes = parse_input();
    let mut droplet: Droplet = HashMap::default();
    for cube in cubes.iter() {
        droplet.insert(*cube, (DropletState::Lava, false));
    }

    let sum = cubes.iter().fold(0, |acc, cube| {
        acc + neighbors(cube)
            .iter()
            .filter(|neighbour| !droplet.contains_key(&neighbour))
            .count() as i32
    });

    println!("Surface area: {sum}");
}

fn part2() {
    let cubes = parse_input();
    let mut droplet: Droplet = HashMap::default();

    for cube in cubes.iter() {
        droplet.insert(*cube, (DropletState::Lava, false));
    }

    for z in MIN_Z..=MAX_Z {
        for y in MIN_Y..=MAX_Y {
            for x in MIN_X..=MAX_X {
                let air_cube: Cube = (x, y, z);
                if !droplet.contains_key(&air_cube) {
                    droplet.insert(air_cube, (DropletState::Air, false));
                }
            }
        }
    }

    for x in MIN_X..=MAX_X {
        find_edge_cubes(&mut droplet, &(x, MIN_Y, MIN_Z));
        find_edge_cubes(&mut droplet, &(x, MIN_Y, MAX_Z));
        find_edge_cubes(&mut droplet, &(x, MAX_Y, MIN_Z));
        find_edge_cubes(&mut droplet, &(x, MAX_Y, MAX_Z));
    }

    for y in MIN_Y..=MAX_Y {
        find_edge_cubes(&mut droplet, &(MIN_X, y, MIN_Z));
        find_edge_cubes(&mut droplet, &(MIN_X, y, MAX_Z));
        find_edge_cubes(&mut droplet, &(MAX_X, y, MIN_Z));
        find_edge_cubes(&mut droplet, &(MAX_X, y, MAX_Z));
    }

    for z in MIN_Z..=MAX_Z {
        find_edge_cubes(&mut droplet, &(MIN_X, MIN_Y, z));
        find_edge_cubes(&mut droplet, &(MIN_X, MAX_Y, z));
        find_edge_cubes(&mut droplet, &(MAX_X, MIN_Y, z));
        find_edge_cubes(&mut droplet, &(MAX_X, MAX_Y, z));
    }

    let mut sum = 0;
    for cube in cubes.iter() {
        let (_, visited) = droplet.get(cube).unwrap();
        if !visited {
            continue;
        }

        let cube_neighbours = neighbors(cube);
        let count = cube_neighbours
            .iter()
            .filter(|neighbour| {
                let (state, visited) = droplet.get(&neighbour).unwrap();
                matches!(state, DropletState::Air) && *visited
            })
            .count();

        sum += count;
    }

    println!("Surfaces: {}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
