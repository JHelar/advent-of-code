use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

#[derive(Debug)]
struct Vector3(f64, f64, f64);

impl Vector3 {
    fn from_line(line: &str) -> Self {
        let mut positions = line
            .split(",")
            .into_iter()
            .map(|number| number.trim().parse::<f64>().unwrap());
        let x = positions.next().unwrap();
        let y = positions.next().unwrap();
        let z = positions.next().unwrap();

        Self(x, y, z)
    }

    fn fill(val: f64) -> Self {
        Self(val, val, val)
    }

    fn is_within(&self, from: &Self, to: &Self) -> bool {
        self.0 >= from.0 && self.0 <= to.0 && self.1 >= from.1 && self.1 <= to.1
    }
}

#[derive(Debug)]
struct Hail {
    position: Vector3,
    vector: Vector3,
    a: f64,
    b: f64,
    c: f64,
}

impl Hail {
    fn from_line_string(line: &str) -> Self {
        let (position_str, vector_str) = line.split_once(" @ ").unwrap();

        let vector = Vector3::from_line(vector_str);
        let position = Vector3::from_line(position_str);
        let a = vector.1;
        let b = -vector.0;
        let c = vector.1 * position.0 - vector.0 * position.1;

        Self {
            position,
            vector,
            a,
            b,
            c,
        }
    }

    fn is_parallel(&self, other: &Hail) -> bool {
        self.a * other.b == other.a * self.b
    }

    fn get_intersection(&self, other: &Hail) -> Vector3 {
        let x = (self.c * other.b - other.c * self.b) / (self.a * other.b - other.a * self.b);
        let y = (other.c * self.a - self.c * other.a) / (self.a * other.b - other.a * self.b);

        Vector3(x, y, 0_f64)
    }

    fn is_in_future(&self, pos: &Vector3) -> bool {
        ((pos.0 - self.position.0) * self.vector.0) >= 0_f64
            && ((pos.1 - self.position.1) * self.vector.1) >= 0_f64
    }
}

fn get_intersection_point(hailstones: &Vec<Hail>) -> Vector3 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let xr = Int::new_const(&ctx, "xr");
    let yr = Int::new_const(&ctx, "yr");
    let zr = Int::new_const(&ctx, "zr");
    let vxr = Int::new_const(&ctx, "vxr");
    let vyr = Int::new_const(&ctx, "vyr");
    let vzr = Int::new_const(&ctx, "vzr");

    for hailstone in hailstones {
        let sx = Int::from_i64(&ctx, hailstone.position.0 as i64);
        let sy = Int::from_i64(&ctx, hailstone.position.1 as i64);
        let sz = Int::from_i64(&ctx, hailstone.position.2 as i64);
        let vx = Int::from_i64(&ctx, hailstone.vector.0 as i64);
        let vy = Int::from_i64(&ctx, hailstone.vector.1 as i64);
        let vz = Int::from_i64(&ctx, hailstone.vector.2 as i64);

        solver.assert(&((&xr - &sx) * (&vy - &vyr))._eq(&((&yr - &sy) * (&vx - &vxr))));
        solver.assert(&((&yr - &sy) * (&vz - &vzr))._eq(&((&zr - &sz) * (&vy - &vyr))));
    }

    solver.check();
    let model = solver.get_model().unwrap();

    let x = model.get_const_interp(&xr).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&yr).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&zr).unwrap().as_i64().unwrap();

    Vector3(
        x as f64,
        y as f64,
        z as f64
    )
}

fn read_input() -> Vec<Hail> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Hail::from_line_string(line.trim()))
        .collect()
}

fn part1() -> Option<u64> {
    let hailstones = read_input();

    let from = Vector3::fill(200000000000000_f64);
    let to = Vector3::fill(400000000000000_f64);
    let mut result = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let a = &hailstones[i];
            let b = &hailstones[j];

            if a.is_parallel(b) {
                continue;
            }

            let intersection = a.get_intersection(b);
            if !intersection.is_within(&from, &to) {
                continue;
            }

            if a.is_in_future(&intersection) && b.is_in_future(&intersection) {
                result += 1;
            }
        }
    }

    Some(result)
}

fn part2() -> Option<u64> {
    let hailstones = read_input();
    let intersection = get_intersection_point(&hailstones);
    let result = (intersection.0 + intersection.1 + intersection.2) as u64;
    Some(result)
}

fn main() {
    println!("--- Day 24: Never Tell Me The Odds ---");
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
