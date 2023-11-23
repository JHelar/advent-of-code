use hashbrown::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Robot {
    ore_cost: i32,
    clay_cost: i32,
    obsidian_cost: i32,
}

impl Robot {
    fn new() -> Self {
        Self {
            ore_cost: 0,
            clay_cost: 0,
            obsidian_cost: 0,
        }
    }
    fn from_recipe(recipe_str: &str) -> Self {
        let mut robot = Self::new();

        let (_, resource_str) = recipe_str.split_once(" robot costs ").unwrap();
        resource_str
            .split(" and ")
            .into_iter()
            .for_each(|resource_str| {
                let (cost, resource_type) = resource_str.split_once(' ').unwrap();
                match resource_type {
                    "ore" => robot.ore_cost = cost.parse::<i32>().unwrap(),
                    "clay" => robot.clay_cost = cost.parse::<i32>().unwrap(),
                    "obsidian" => robot.obsidian_cost = cost.parse::<i32>().unwrap(),
                    _ => todo!("Unknown resource type {resource_type}"),
                }
            });

        robot
    }
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot: Robot,
    ore_robot_max: i32,
    clay_robot: Robot,
    clay_robot_max: i32,
    obsidian_robot: Robot,
    obsidian_robot_max: i32,
    geode_robot: Robot,
}

impl Blueprint {
    fn from_str(blueprint_line: &str) -> Self {
        let (name, recipe) = blueprint_line.split_once(':').unwrap();
        let mut recipe_iter = recipe.trim().split('.').into_iter().map(|str| str.trim());

        let ore_robot_recipe = recipe_iter.next().unwrap();
        let clay_robot_recipe = recipe_iter.next().unwrap();
        let obsidian_robot_recipe = recipe_iter.next().unwrap();
        let geode_robot_recipe = recipe_iter.next().unwrap();

        let ore_robot = Robot::from_recipe(ore_robot_recipe);
        let clay_robot = Robot::from_recipe(clay_robot_recipe);
        let obsidian_robot = Robot::from_recipe(obsidian_robot_recipe);
        let geode_robot = Robot::from_recipe(geode_robot_recipe);

        let ore_robot_max = vec![
            ore_robot.ore_cost,
            clay_robot.ore_cost,
            obsidian_robot.ore_cost,
            geode_robot.ore_cost,
        ]
        .iter()
        .max()
        .unwrap()
        .clone();

        let clay_robot_max = vec![
            ore_robot.clay_cost,
            clay_robot.clay_cost,
            obsidian_robot.clay_cost,
            geode_robot.clay_cost,
        ]
        .iter()
        .max()
        .unwrap()
        .clone();

        let obsidian_robot_max = vec![
            ore_robot.obsidian_cost,
            clay_robot.obsidian_cost,
            obsidian_robot.obsidian_cost,
            geode_robot.obsidian_cost,
        ]
        .iter()
        .max()
        .unwrap()
        .clone();

        Self {
            id: name
                .trim()
                .split("Blueprint ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            ore_robot,
            ore_robot_max,
            clay_robot,
            clay_robot_max,
            obsidian_robot,
            obsidian_robot_max,
            geode_robot,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct State {
    ore_count: i32,
    clay_count: i32,
    obsidian_count: i32,
    geode_count: i32,

    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

impl State {
    fn new() -> Self {
        Self {
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn can_build(&self, robot: &Robot) -> bool {
        if self.ore_count < robot.ore_cost {
            return false;
        }
        if self.clay_count < robot.clay_cost {
            return false;
        }
        if self.obsidian_count < robot.obsidian_cost {
            return false;
        }
        true
    }

    fn should_have_built(&self, _robot: &Robot) -> bool {
        /* Todo: look into this */
        // if robot.ore_cost != 0 && (self.ore_count - self.ore_robots) >= robot.ore_cost {
        //     return true;
        // }

        // if robot.clay_cost != 0 && (self.clay_count - self.clay_robots) >= robot.clay_cost {
        //     return true;
        // }

        false
    }

    fn collect_and_build(&mut self, robot: &Robot) {
        self.collect();

        self.ore_count -= robot.ore_cost;
        self.clay_count -= robot.clay_cost;
        self.obsidian_count -= robot.obsidian_cost;
    }

    fn collect(&mut self) {
        self.ore_count += self.ore_robots;
        self.clay_count += self.clay_robots;
        self.obsidian_count += self.obsidian_robots;
        self.geode_count += self.geode_robots;
    }
}

fn run_simulation(
    state: &State,
    blueprint: &Blueprint,
    max_minutes: i32,
    minute: i32,
    mem: &mut HashMap<(i32, State), i32>,
) -> i32 {
    let key = (minute, *state);
    if mem.contains_key(&key) {
        return *mem.get(&key).unwrap();
    }

    if minute > max_minutes {
        return state.geode_count;
    }

    let mut best_value = 0;

    // Try build robots
    if state.can_build(&blueprint.geode_robot) {
        let mut new_state = state.clone();

        new_state.collect_and_build(&blueprint.geode_robot);
        new_state.geode_robots += 1;

        best_value = best_value.max(run_simulation(
            &new_state,
            blueprint,
            max_minutes,
            minute + 1,
            mem,
        ));
    } else {
        if blueprint.obsidian_robot_max != state.obsidian_robots
            && state.can_build(&blueprint.obsidian_robot)
            && !state.should_have_built(&blueprint.obsidian_robot)
        {
            let mut new_state = state.clone();

            new_state.collect_and_build(&blueprint.obsidian_robot);
            new_state.obsidian_robots += 1;

            best_value = best_value.max(run_simulation(
                &new_state,
                blueprint,
                max_minutes,
                minute + 1,
                mem,
            ));
        }
        if blueprint.clay_robot_max != state.clay_robots
            && state.can_build(&blueprint.clay_robot)
            && !state.should_have_built(&blueprint.clay_robot)
        {
            let mut new_state = state.clone();

            new_state.collect_and_build(&blueprint.clay_robot);
            new_state.clay_robots += 1;

            best_value = best_value.max(run_simulation(
                &new_state,
                blueprint,
                max_minutes,
                minute + 1,
                mem,
            ));
        }
        if blueprint.ore_robot_max != state.ore_robots
            && state.can_build(&blueprint.ore_robot)
            && !state.should_have_built(&blueprint.ore_robot)
        {
            let mut new_state = state.clone();

            new_state.collect_and_build(&blueprint.ore_robot);
            new_state.ore_robots += 1;

            best_value = best_value.max(run_simulation(
                &new_state,
                blueprint,
                max_minutes,
                minute + 1,
                mem,
            ));
        }
        // Do not build any robot
        let mut new_state = state.clone();
        new_state.collect();

        best_value = best_value.max(run_simulation(
            &new_state,
            blueprint,
            max_minutes,
            minute + 1,
            mem,
        ));
    }

    mem.insert(key, best_value);
    best_value
}

fn parse_input() -> Vec<Blueprint> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .trim()
        .lines()
        .map(Blueprint::from_str)
        .collect()
}

fn part1() {
    let blueprints = parse_input();

    let mut sum: i32 = 0;
    let mut mem = HashMap::new();

    for blueprint in blueprints.iter() {
        let state = State::new();
        let result = run_simulation(&state, blueprint, 24, 1, &mut mem);

        mem.clear();

        sum += result * blueprint.id;
    }

    println!("Result: {sum}");
}

fn part2() {
    let blueprints = parse_input();

    let mut sum: i32 = 1;
    let mut mem = HashMap::new();

    for blueprint in blueprints.iter().take(3) {
        println!("{:?}", blueprint);
        let state = State::new();
        let result = run_simulation(&state, blueprint, 32, 1, &mut mem);

        mem.clear();

        sum *= result;
    }

    println!("Result: {sum}");
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
