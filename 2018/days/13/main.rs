use std::{
    cell::RefCell,
    char,
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fmt::Display,
    rc::Rc,
};

type Vector2 = (isize, isize);

const DIR_UP: usize = 0;
const DIR_RIGHT: usize = 1;
const DIR_DOWN: usize = 2;
const DIR_LEFT: usize = 3;

fn turn_clockwise(from_dir: usize) -> usize {
    (from_dir + 1) % 4
}

fn turn_counter_clockwise(from_dir: usize) -> usize {
    if from_dir == 0 {
        3
    } else {
        from_dir - 1
    }
}

struct TrackMap {
    map: Vec<Option<Rc<RefCell<Track>>>>,
    size: Vector2,
}

impl TrackMap {
    fn get_track_mut(&mut self, position: Vector2) -> Option<Rc<RefCell<Track>>> {
        let index = position.0 + (self.size.0 * position.1);
        if index < 0 || index > (self.map.len() as isize - 1) {
            None
        } else if let Some(track) = &self.map[index as usize] {
            Some(Rc::clone(track))
        } else {
            None
        }
    }

    fn get_track(&self, position: Vector2) -> Option<Rc<RefCell<Track>>> {
        let index = position.0 + (self.size.0 * position.1);
        if index < 0 || index > (self.map.len() as isize - 1) {
            None
        } else if let Some(track) = &self.map[index as usize] {
            Some(Rc::clone(track))
        } else {
            None
        }
    }

    fn get_position_from_index(&self, index: isize) -> Vector2 {
        (index % self.size.0, index / self.size.0)
    }
}

impl Display for TrackMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if let Some(track) = self.get_track((x, y)) {
                    write!(f, "{}", track.borrow())?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Track {
    c: char,
    directions: [Option<Rc<RefCell<Track>>>; 4],
}

impl Track {
    fn from_char(char: &char, x: isize, y: isize) -> Option<(Self, Option<Train>)> {
        match char {
            '|' | '-' | '/' | '\\' | '+' => Some((
                Self {
                    c: *char,
                    directions: [None, None, None, None],
                },
                None,
            )),
            _ => match Train::from_char(char, x, y) {
                Some(train) => match train.direction {
                    DIR_LEFT => Some((
                        Self {
                            c: '-',
                            directions: [None, None, None, None],
                        },
                        Some(train),
                    )),
                    DIR_RIGHT => Some((
                        Self {
                            c: '-',
                            directions: [None, None, None, None],
                        },
                        Some(train),
                    )),
                    DIR_UP => Some((
                        Self {
                            c: '|',
                            directions: [None, None, None, None],
                        },
                        Some(train),
                    )),
                    DIR_DOWN => Some((
                        Self {
                            c: '|',
                            directions: [None, None, None, None],
                        },
                        Some(train),
                    )),
                    _ => panic!("Unreachable code"),
                },
                None => {
                    println!("Invalid char {char}");
                    None
                }
            },
        }
    }

    fn get_next(
        &self,
        from_dir: usize,
        crossing_state: usize,
    ) -> (Rc<RefCell<Track>>, usize, usize) {
        match self.c {
            '/' => match from_dir {
                DIR_UP | DIR_DOWN => (
                    self.directions[turn_clockwise(from_dir)].clone().unwrap(),
                    turn_clockwise(from_dir),
                    crossing_state,
                ),
                DIR_LEFT | DIR_RIGHT => (
                    self.directions[turn_counter_clockwise(from_dir)]
                        .clone()
                        .unwrap(),
                    turn_counter_clockwise(from_dir),
                    crossing_state,
                ),
                _ => panic!("Nope"),
            },
            '\\' => match from_dir {
                DIR_UP | DIR_DOWN => (
                    self.directions[turn_counter_clockwise(from_dir)]
                        .clone()
                        .unwrap(),
                    turn_counter_clockwise(from_dir),
                    crossing_state,
                ),
                DIR_LEFT | DIR_RIGHT => (
                    self.directions[turn_clockwise(from_dir)].clone().unwrap(),
                    turn_clockwise(from_dir),
                    crossing_state,
                ),
                _ => panic!("Nope"),
            },
            '-' | '|' => (
                self.directions[from_dir].clone().unwrap(),
                from_dir,
                crossing_state,
            ),
            '+' => match crossing_state {
                0 => (
                    self.directions[turn_counter_clockwise(from_dir)]
                        .clone()
                        .unwrap(),
                    turn_counter_clockwise(from_dir),
                    1,
                ),
                1 => (self.directions[from_dir].clone().unwrap(), from_dir, 2),
                2 => (
                    self.directions[turn_clockwise(from_dir)].clone().unwrap(),
                    turn_clockwise(from_dir),
                    0,
                ),
                _ => panic!("Nope"),
            },
            _ => panic!("Nope"),
        }
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}

#[derive(Debug)]
struct Train {
    position: Vector2,
    direction: usize,
    crossing_state: usize,
    track: Option<Rc<RefCell<Track>>>,
}

impl Ord for Train {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let y_pos = self.position.1.cmp(&other.position.1);
        if matches!(y_pos, Ordering::Equal) {
            self.position.0.cmp(&other.position.0)
        } else {
            y_pos
        }
    }
}

impl PartialOrd for Train {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Train {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Train {}

impl Train {
    fn from_char(char: &char, x: isize, y: isize) -> Option<Self> {
        let position = (x, y);
        let direction = match char {
            '<' => DIR_LEFT,
            '>' => DIR_RIGHT,
            '^' => DIR_UP,
            'v' => DIR_DOWN,
            _ => return None,
        };

        Some(Self {
            direction,
            position,
            track: None,
            crossing_state: 0,
        })
    }

    fn step(&mut self) {
        let (next_track, next_direction, next_crossing_state) = self
            .track
            .clone()
            .unwrap()
            .borrow()
            .get_next(self.direction, self.crossing_state);

        self.track = Some(next_track);
        self.direction = next_direction;
        self.position = add(self.position, next_direction);
        self.crossing_state = next_crossing_state;
    }
}

impl Display for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.direction {
            DIR_LEFT => '<',
            DIR_RIGHT => '>',
            DIR_UP => '^',
            DIR_DOWN => 'v',
            _ => panic!("Unreachable code"),
        };
        write!(f, "{c}")
    }
}

fn read_input() -> (TrackMap, Vec<Train>) {
    let mut map = Vec::new();
    let mut map_width: isize = 0;
    let mut max_height = 0;
    let mut trains = Vec::new();

    for (y, line) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
    {
        map_width = line.len() as isize;
        max_height += 1;

        for (x, char) in line.chars().enumerate() {
            if char::is_whitespace(char) {
                map.push(None);
            } else if let Some((track, maybe_train)) =
                Track::from_char(&char, x as isize, y as isize)
            {
                let track_rc = Rc::new(RefCell::new(track));
                if let Some(mut train) = maybe_train {
                    train.track = Some(Rc::clone(&track_rc));
                    trains.push(train);
                }
                map.push(Some(track_rc));
            } else {
                map.push(None);
            }
        }
    }

    (
        TrackMap {
            map,
            size: (map_width, max_height),
        },
        trains,
    )
}

fn add(a: Vector2, direction: usize) -> Vector2 {
    match direction {
        DIR_UP => (a.0, a.1 - 1),
        DIR_DOWN => (a.0, a.1 + 1),
        DIR_LEFT => (a.0 - 1, a.1),
        DIR_RIGHT => (a.0 + 1, a.1),
        _ => panic!("Invalid dir"),
    }
}

fn connect_map(tm: &mut TrackMap) {
    for index in 0..tm.map.len() {
        let position = tm.get_position_from_index(index as isize);
        let up = tm.get_track_mut(add(position, DIR_UP));
        let right = tm.get_track_mut(add(position, DIR_RIGHT));
        let down = tm.get_track_mut(add(position, DIR_DOWN));
        let left = tm.get_track_mut(add(position, DIR_LEFT));

        if let Some(track) = &tm.map[index] {
            track.borrow_mut().directions[DIR_UP] = up;
            track.borrow_mut().directions[DIR_DOWN] = down;
            track.borrow_mut().directions[DIR_LEFT] = left;
            track.borrow_mut().directions[DIR_RIGHT] = right;
        }
    }
}

fn print_train_map(tm: &TrackMap, trains: &Vec<Train>, collision_at: Vector2) {
    print!("\n-----------------------\n\n");
    for y in 0..tm.size.1 {
        for x in 0..tm.size.0 {
            let position = (x, y);
            if let Some(track) = tm.get_track(position) {
                if position == collision_at {
                    print!("X");
                } else if let Some(train) = trains.iter().find(|&train| train.position == position)
                {
                    print!("{}", train);
                } else {
                    print!("{}", track.borrow());
                }
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn part1() -> Option<String> {
    let (mut map, mut trains) = read_input();
    connect_map(&mut map);
    trains.sort();

    for _ in 0.. {
        for train_index in 0..trains.len() {
            {
                let train = trains.get_mut(train_index).unwrap();
                train.step();
            }

            let train_count = trains
                .iter()
                .filter(|&other_train| *other_train == trains[train_index])
                .count();

            if train_count == 2 {
                let collision = trains[train_index].position;
                print_train_map(&map, &trains, collision);
                return Some(format!("{},{}", collision.0, collision.1));
            }
        }
        trains.sort();
    }
    None
}

fn part2() -> Option<String> {
    let (mut map, mut trains) = read_input();
    connect_map(&mut map);
    trains.sort();

    print_train_map(&map, &trains, (-1, -1));
    let mut collisions_at = HashSet::new();

    for _ in 0.. {
        for train_index in 0..trains.len() {
            if collisions_at.contains(&trains[train_index].position) {
                continue;
            }

            {
                if let Some(train) = trains.get_mut(train_index) {
                    train.step();
                }
            }

            let train_count = trains
                .iter()
                .filter(|&other_train| *other_train == trains[train_index])
                .count();

            if train_count == 2 {
                collisions_at.insert(trains[train_index].position);
            }
        }

        trains = trains
            .into_iter()
            .filter(|train| !collisions_at.contains(&train.position))
            .collect();

        collisions_at.clear();

        if trains.len() == 1 {
            print_train_map(&map, &trains, (-1, -1));
            return Some(format!("{},{}", trains[0].position.0, trains[0].position.1));
        }
        trains.sort();
    }
    None
}

fn main() {
    println!("--- Day 13: Mine Cart Madness ---");
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
