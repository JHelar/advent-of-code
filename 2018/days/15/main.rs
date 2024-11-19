mod graph;
mod vector2;

use std::{cmp::Ordering, fmt::Display, rc::Rc, usize};

use graph::{Edge, Graph, Node};
use vector2::Vector2;

const UP: Vector2 = Vector2(0, -1);
const DOWN: Vector2 = Vector2(0, 1);
const LEFT: Vector2 = Vector2(-1, 0);
const RIGHT: Vector2 = Vector2(1, 0);
const DIRECTIONS: [Vector2; 4] = [UP, DOWN, LEFT, RIGHT];

type UnitId = usize;

#[derive(Debug)]
enum ActionResult {
    NoEnemies(Vec<UnitVariant>),
    NoPath,
    Death(Vec<UnitVariant>),
    Nothing,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum UnitVariant {
    Elf,
    Goblin,
}

impl Display for UnitVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Elf => write!(f, "E"),
            Self::Goblin => write!(f, "G"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Unit {
    unit_id: UnitId,
    variant: UnitVariant,
    hp: isize,
    ap: isize,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.variant {
            UnitVariant::Goblin => write!(f, "G"),
            UnitVariant::Elf => write!(f, "E"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TileType {
    Wall,
    Air,
    Unit(Unit),
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    position: Vector2,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.tile_type {
            TileType::Air => write!(f, " "),
            TileType::Wall => write!(f, "🪨"),
            TileType::Unit(unit) => write!(f, "{}", unit),
        }
    }
}

impl Ord for Vector2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let y_pos = self.1.cmp(&other.1);
        if matches!(y_pos, Ordering::Equal) {
            self.0.cmp(&other.0)
        } else {
            y_pos
        }
    }
}

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.tile_type == other.tile_type && self.position == other.position
    }
}

impl Eq for Tile {}

struct Cave {
    origin: (Vec<Vec<Tile>>, Vec<usize>),
    map: Vec<Vec<Tile>>,
    units: Vec<UnitId>,
    graph: Graph<Vector2>,
}

impl Cave {
    fn new(map: Vec<Vec<Tile>>, units: Vec<UnitId>, graph: Graph<Vector2>) -> Self {
        let origin: (Vec<Vec<Tile>>, Vec<UnitId>) = (map.clone(), units.clone());
        Self {
            map,
            units,
            graph,
            origin,
        }
    }

    fn get_unit_tile(&self, unit_id: &UnitId) -> Option<&Tile> {
        for row in self.map.iter() {
            let tile = row.iter().find(|&tile| match tile.tile_type {
                TileType::Unit(unit) => &unit.unit_id == unit_id,
                _ => false,
            });
            if tile.is_some() {
                return tile;
            }
        }
        None
    }

    fn get_unit_tile_mut(&mut self, unit_id: &UnitId) -> Option<&mut Tile> {
        for row in self.map.iter_mut() {
            let tile = row.iter_mut().find(|tile| match tile.tile_type {
                TileType::Unit(unit) => &unit.unit_id == unit_id,
                _ => false,
            });
            if tile.is_some() {
                return tile;
            }
        }
        None
    }

    fn get_tile(&self, position: &Vector2) -> Option<&Tile> {
        if let Some(row) = self.map.get(position.1 as usize) {
            row.get(position.0 as usize)
        } else {
            None
        }
    }

    fn get_tile_mut(&mut self, position: &Vector2) -> Option<&mut Tile> {
        if let Some(row) = self.map.get_mut(position.1 as usize) {
            row.get_mut(position.0 as usize)
        } else {
            None
        }
    }

    fn get_neighbor_tiles(&self, position: &Vector2) -> Vec<&Tile> {
        self.graph
            .get_neighbor_nodes(position)
            .iter()
            .map(|node| self.get_tile(&node.value).unwrap())
            .collect()
    }

    fn reset(&mut self) {
        self.map = self.origin.0.clone();
        self.units = self.origin.1.clone();
    }

    fn set_unit_variant_ap(&mut self, variant: UnitVariant, ap: isize) {
        for unit_index in 0..self.units.len() {
            let unit_id = self.units[unit_index].clone();
            let tile = self.get_unit_tile_mut(&unit_id).unwrap();
            match tile.tile_type {
                TileType::Unit(ref mut unit) if unit.variant == variant => {
                    unit.ap = ap;
                }
                _ => {}
            }
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            let unit_row = row
                .iter()
                .filter_map(|tile| match tile.tile_type {
                    TileType::Unit(unit) => Some(format!("{}({})", unit.variant, unit.hp)),
                    _ => None,
                })
                .collect::<Vec<String>>()
                .join(", ");
            write!(f, "   {unit_row}\n")?;
        }
        Ok(())
    }
}

fn game_round(cave: &mut Cave) -> Result<ActionResult, ActionResult> {
    let mut units = cave.units.clone();
    units.sort_by(
        |a_id, b_id| match (cave.get_unit_tile(a_id), cave.get_unit_tile(b_id)) {
            (
                Some(Tile {
                    tile_type: TileType::Unit(_),
                    position: a,
                }),
                Some(Tile {
                    tile_type: TileType::Unit(_),
                    position: b,
                }),
            ) => {
                a.cmp(b)
            },
            _ => Ordering::Equal
        },
    );

    let mut killed_unit_types = Vec::new();

    for unit_index in units.iter() {
        if cave.get_unit_tile(unit_index).is_none() {
            continue;
        }

        let has_enemies = cave.units.iter().any(|other| {
            match (&cave.get_unit_tile(unit_index), &cave.get_unit_tile(other)) {
                (
                    Some(Tile {
                        tile_type: TileType::Unit(this_unit),
                        position: _,
                    }),
                    Some(Tile {
                        tile_type: TileType::Unit(other_unit),
                        position: _,
                    }),
                ) if this_unit.variant != other_unit.variant => true,
                _ => false,
            }
        });

        if !has_enemies || cave.units.len() == 1 {
            return Err(ActionResult::NoEnemies(killed_unit_types));
        }

        if let Some(target_id) = get_enemy_target(unit_index, cave) {
            if unit_attack(unit_index, &target_id, cave).is_err() {
                let tile = cave.get_unit_tile_mut(&target_id).unwrap();
                match tile.tile_type {
                    TileType::Unit(unit) => {
                        killed_unit_types.push(unit.variant);
                        tile.tile_type = TileType::Air;
                    }
                    _ => {}
                }
            }
        } else {
            match unit_step(unit_index, cave) {
                Err(ActionResult::NoPath) => continue,
                Err(ActionResult::NoEnemies(_)) => {
                    return Err(ActionResult::NoEnemies(killed_unit_types))
                }
                _ => {}
            };
            if let Some(target_id) = get_enemy_target(unit_index, cave) {
                if unit_attack(unit_index, &target_id, cave).is_err() {
                    let tile = cave.get_unit_tile_mut(&target_id).unwrap();
                    match tile.tile_type {
                        TileType::Unit(unit) => {
                            killed_unit_types.push(unit.variant);
                            tile.tile_type = TileType::Air;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    cave.units = cave
        .units
        .iter()
        .filter_map(|unit_id| {
            if cave.get_unit_tile(unit_id).is_some() {
                Some(unit_id.clone())
            } else {
                None
            }
        })
        .collect();
    if killed_unit_types.len() > 0 {
        Ok(ActionResult::Death(killed_unit_types))
    } else {
        Ok(ActionResult::Nothing)
    }
}

fn unit_attack(unit_id: &UnitId, target_id: &UnitId, cave: &mut Cave) -> Result<(), ()> {
    let unit_tile = cave.get_unit_tile(unit_id).unwrap();
    let unit_ap = match unit_tile.tile_type {
        TileType::Unit(unit) => unit.ap,
        _ => panic!("Tried to attack with non unit type"),
    };

    let target_tile = cave.get_unit_tile_mut(target_id).unwrap();
    match target_tile.tile_type {
        TileType::Unit(ref mut target_unit) => {
            target_unit.hp -= unit_ap;
            if target_unit.hp <= 0 {
                Err(())
            } else {
                Ok(())
            }
        }
        _ => panic!("Tried to attack non unit type"),
    }
}

fn unit_step(unit_id: &UnitId, cave: &mut Cave) -> Result<(), ActionResult> {
    let to_position = {
        let unit_tile = cave.get_unit_tile(unit_id).unwrap();

        let considered_positions = cave
            .graph
            .get_neighbor_nodes(&unit_tile.position)
            .iter()
            .filter_map(|node| {
                if cave.get_tile(&node.value).unwrap().tile_type == TileType::Air {
                    Some(*node.value.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Vector2>>();

        if considered_positions.len() == 0 {
            return Err(ActionResult::NoPath);
        }

        let destination_positions = cave
            .units
            .iter()
            .filter_map(|other_id| {
                if unit_id == other_id {
                    return None;
                }
                if let Some(other_tile) = cave.get_unit_tile(other_id) {
                    match (&unit_tile.tile_type, &other_tile.tile_type) {
                        (TileType::Unit(this_unit), TileType::Unit(other_unit))
                            if this_unit.variant != other_unit.variant =>
                        {
                            // Check if we can walk to enemy
                            if cave
                                .graph
                                .get_neighbor_nodes(&other_tile.position)
                                .iter()
                                .any(|node| {
                                    cave.get_tile(&node.value).unwrap().tile_type == TileType::Air
                                })
                            {
                                Some(other_tile.position)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .flat_map(|enemy_position| {
                cave.graph
                    .get_neighbor_nodes(&enemy_position)
                    .iter()
                    .filter_map(|node| {
                        if cave.get_tile(&node.value).unwrap().tile_type == TileType::Air {
                            Some(*node.value.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Vector2>>()
            })
            .collect::<Vec<Vector2>>();

        if destination_positions.len() == 0 {
            return Err(ActionResult::NoPath);
        }

        let mut best_path_len = usize::MAX;
        let mut best_paths = Vec::default();

        let is_edge_walkable =
            &|edge: &Edge<Vector2>| match cave.get_tile(&edge.destination.value).unwrap().tile_type
            {
                TileType::Wall => false,
                TileType::Unit(_) => false,
                _ => true,
            };

        for considered_position in considered_positions {
            for destination_position in destination_positions.iter() {
                if let Some(best_path) = graph::dijkstra::get_path(
                    &considered_position,
                    destination_position,
                    is_edge_walkable,
                    &cave.graph,
                ) {
                    if best_path.len() > best_path_len {
                        continue;
                    }
                    if best_path.len() < best_path_len {
                        best_path_len = best_path.len();
                        best_paths.clear();
                    }
                    best_paths.push((considered_position, destination_position.clone()));
                } else {
                    continue;
                }
            }
        }

        if best_paths.len() == 0 {
            return Err(ActionResult::NoPath);
        }

        best_paths.sort_by(|a, b| {
            let destination_cmp = a.1.cmp(&b.1);
            if matches!(destination_cmp, Ordering::Equal) {
                a.0.cmp(&b.0)
            } else {
                destination_cmp
            }
        });

        best_paths[0].0
    };

    let unit_tile_clone = cave.get_unit_tile(unit_id).unwrap().clone();

    cave.get_tile_mut(&to_position).unwrap().tile_type = unit_tile_clone.tile_type;
    cave.get_tile_mut(&unit_tile_clone.position)
        .unwrap()
        .tile_type = TileType::Air;

    Ok(())
}

fn get_enemy_target(unit_id: &usize, cave: &Cave) -> Option<UnitId> {
    let unit_tile = cave.get_unit_tile(unit_id).unwrap();
    let neighbor_tiles = cave.get_neighbor_tiles(&unit_tile.position);
    let mut enemy_tiles = neighbor_tiles
        .iter()
        .filter_map(
            |&neighbor_tile| match (&unit_tile.tile_type, &neighbor_tile.tile_type) {
                (TileType::Unit(unit), TileType::Unit(neighbor_unit))
                    if unit.variant != neighbor_unit.variant =>
                {
                    Some((neighbor_tile.position, neighbor_unit))
                }
                _ => None,
            },
        )
        .collect::<Vec<(Vector2, &Unit)>>();

    enemy_tiles.sort_by(|(a_position, a_unit), (b_position, b_unit)| {
        let hp = a_unit.hp.cmp(&b_unit.hp);
        if matches!(hp, Ordering::Equal) {
            a_position.cmp(b_position)
        } else {
            hp
        }
    });

    if let Some(&(_, enemy)) = enemy_tiles.first() {
        Some(enemy.unit_id)
    } else {
        None
    }
}

fn calc_score(round: usize, cave: &Cave) -> usize {
    let unit_sum = cave
        .units
        .iter()
        .filter_map(|unit_id| match cave.get_unit_tile(unit_id) {
            Some(Tile {
                tile_type: TileType::Unit(unit),
                position: _,
            }) => Some(unit.hp as usize),
            _ => None,
        })
        .sum::<usize>();

    unit_sum * round
}

fn read_input() -> Cave {
    let mut units = Vec::new();
    let mut map = Vec::new();
    let mut edges = Vec::new();
    let mut nodes = Vec::new();

    for (y, row_line) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
    {
        let mut row = Vec::new();
        for (x, char) in row_line.trim().chars().enumerate() {
            let position = Vector2::new(x as isize, y as isize);
            let tile = match char {
                '#' => Tile {
                    tile_type: TileType::Wall,
                    position,
                },
                '.' => Tile {
                    tile_type: TileType::Air,
                    position,
                },
                'G' => {
                    let goblin = Unit {
                        unit_id: units.len(),
                        variant: UnitVariant::Goblin,
                        hp: 200,
                        ap: 3,
                    };

                    let tile = Tile {
                        tile_type: TileType::Unit(goblin),
                        position,
                    };
                    tile
                }
                'E' => {
                    let elf = Unit {
                        unit_id: units.len(),
                        variant: UnitVariant::Elf,
                        hp: 200,
                        ap: 3,
                    };

                    let tile = Tile {
                        tile_type: TileType::Unit(elf),
                        position,
                    };
                    tile
                }
                _ => panic!("Unknown tile {char}"),
            };

            let position_rc = Rc::new(tile.position);
            match tile.tile_type {
                TileType::Unit(unit) => {
                    units.push(unit.unit_id);
                    nodes.push(Rc::new(Node::new(position_rc)));
                }
                TileType::Air => nodes.push(Rc::new(Node::new(position_rc))),
                TileType::Wall => {}
            }
            row.push(tile);
        }
        map.push(row);
    }

    for node in nodes.iter() {
        let neighbors = DIRECTIONS.iter().filter_map(|direction| {
            let position = node.value.add(direction);
            if let Some(row) = map.get(position.1 as usize) {
                if let Some(tile) = row.get(position.0 as usize) {
                    if tile.tile_type != TileType::Wall {
                        Some(Rc::new(position))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });

        for neighbor in neighbors {
            let neighbor_node = nodes.iter().find(|&n| n.value == neighbor).unwrap();
            let edge = Edge::new(Rc::clone(node), Rc::clone(neighbor_node));
            edges.push(edge);
        }
    }

    let graph = Graph::new(nodes, edges);
    Cave::new(map, units, graph)
}

fn part1() -> Option<usize> {
    let mut cave = read_input();
    println!("Initial");
    println!("{cave}");

    for round in 1.. {
        match game_round(&mut cave) {
            Err(reason) => {
                println!("Done: {:?}", reason);
                println!("{cave}");
                return Some(calc_score(round - 1, &cave));
            }
            _ => {
                println!("Completed round {round}");
                println!("{cave}");
            }
        }
    }
    None
}

fn part2() -> Option<usize> {
    let mut cave = read_input();
    println!("Initial");
    println!("{cave}");

    // Test repo: https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/
    for ap in 4..=200 {
        cave.reset();
        cave.set_unit_variant_ap(UnitVariant::Elf, ap);

        for round in 1.. {
            match game_round(&mut cave) {
                Err(ActionResult::NoEnemies(units))
                    if units.iter().any(|variant| *variant == UnitVariant::Elf) =>
                {
                    println!("Dead elf, reset!");
                    println!("{cave}");
                    break;
                }
                Err(reason) => {
                    println!("Done: {:?}, Ap: {ap}", reason);
                    println!("{cave}");
                    return Some(calc_score(round - 1, &cave));
                }
                Ok(ActionResult::Death(units))
                    if units.iter().any(|variant| *variant == UnitVariant::Elf) =>
                {
                    println!("Dead elf, reset!");
                    println!("{cave}");
                    break;
                }
                _ => {
                    println!("Completed round {round}");
                    println!("{cave}");
                }
            }
        }
    }
    None
}

fn main() {
    println!("--- Day 15: Beverage Bandits ---");
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
