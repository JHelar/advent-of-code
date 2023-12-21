use hashbrown::HashMap;
use std::any::Any;
use std::collections::VecDeque;
use std::fmt::Debug;

type Signal = (String, String, u8);

trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    Button,
    FlipFlop,
    Conjunction,
    Output,
}

trait Module: AToAny {
    fn process(&mut self, signal: Signal) -> Option<Vec<Signal>>;
    fn get_name(&self) -> String;
    fn get_type(&self) -> ModuleType;
    fn get_state(&self) -> u8;
}

impl Debug for dyn Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}Module{{{}}}", self.get_type(), self.get_name())
    }
}

struct Output {
    name: String,
    lows: u64,
}

impl Module for Output {
    fn process(&mut self, (_, _, signal_strength): Signal) -> Option<Vec<Signal>> {
        if signal_strength == 0 {
            self.lows += 1;
        }
        None
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Output
    }

    fn get_state(&self) -> u8 {
        if self.lows > 0 {
            1
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct Broadcast {
    output: Vec<String>,
}

impl Module for Broadcast {
    fn process(&mut self, (_, _, signal_strength): Signal) -> Option<Vec<Signal>> {
        Some(
            self.output
                .iter()
                .map(|module_name| (self.get_name(), module_name.clone(), signal_strength))
                .collect(),
        )
    }

    fn get_name(&self) -> String {
        "broadcaster".to_string()
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Broadcast
    }

    fn get_state(&self) -> u8 {
        1
    }
}

#[derive(Debug)]
struct Button {}

impl Module for Button {
    fn process(&mut self, _: Signal) -> Option<Vec<Signal>> {
        Some(vec![(self.get_name(), "broadcaster".to_string(), 0)])
    }

    fn get_name(&self) -> String {
        "button".to_string()
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Button
    }

    fn get_state(&self) -> u8 {
        1
    }
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: u8,
    output: Vec<String>,
}

impl Module for FlipFlop {
    fn process(&mut self, (_, _, signal_strength): Signal) -> Option<Vec<Signal>> {
        if signal_strength == 0 {
            self.state = if self.state == 0 { 1 } else { 0 };
            Some(
                self.output
                    .iter()
                    .map(|module_name| (self.get_name(), module_name.clone(), self.state))
                    .collect(),
            )
        } else {
            None
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::FlipFlop
    }

    fn get_state(&self) -> u8 {
        self.state
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    inputs: HashMap<String, u8>,
    state: u8,
    output: Vec<String>,
}

impl Module for Conjunction {
    fn process(&mut self, (signal_source, _, signal_strength): Signal) -> Option<Vec<Signal>> {
        *self.inputs.get_mut(&signal_source).unwrap() = signal_strength;
        self.state = if self
            .inputs
            .iter()
            .all(|(_, module_strength)| *module_strength == 1)
        {
            0
        } else {
            1
        };

        Some(
            self.output
                .iter()
                .map(|module_name| (self.get_name(), module_name.to_string(), self.state))
                .collect(),
        )
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Conjunction
    }

    fn get_state(&self) -> u8 {
        self.state
    }
}

fn read_input() -> HashMap<String, Box<dyn Module>> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut conjunctions = Vec::new();
    let mut outputs: Vec<(String, String)> = Vec::new();

    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let (type_name_string, output_string) = line.trim().split_once(" -> ").unwrap();
            let output: Vec<String> = output_string
                .split(", ")
                .map(|str| str.to_string())
                .collect();

            match type_name_string[0..1].as_ref() {
                "b" => {
                    let module = Broadcast {
                        output: output.clone(),
                    };
                    outputs.append(
                        &mut output
                            .into_iter()
                            .map(|output_name| (module.get_name(), output_name))
                            .collect::<Vec<(String, String)>>(),
                    );
                    modules.insert(module.get_name(), Box::new(module));
                }
                "%" => {
                    let module = FlipFlop {
                        output: output.clone(),
                        name: type_name_string[1..].to_string(),
                        state: 0,
                    };
                    outputs.append(
                        &mut output
                            .into_iter()
                            .map(|output_name| (module.get_name(), output_name))
                            .collect::<Vec<(String, String)>>(),
                    );
                    modules.insert(module.get_name(), Box::new(module));
                }
                "&" => {
                    let module = Conjunction {
                        name: type_name_string[1..].to_string(),
                        output: output.clone(),
                        inputs: HashMap::new(),
                        state: 0,
                    };
                    outputs.append(
                        &mut output
                            .into_iter()
                            .map(|output_name| (module.get_name(), output_name))
                            .collect::<Vec<(String, String)>>(),
                    );
                    conjunctions.push(module.get_name());
                    modules.insert(module.get_name(), Box::new(module));
                }
                module_type => panic!("Unknown module {module_type}"),
            }
        });

    let button_module = Button {};
    modules.insert(button_module.get_name(), Box::new(button_module));
    for conjunction_name in conjunctions {
        let module = modules.get_mut(&conjunction_name).unwrap();
        let conjunction = module
            .as_mut()
            .as_any_mut()
            .downcast_mut::<Conjunction>()
            .unwrap();

        for (from, _) in outputs.iter().filter(|(_, to)| *to == conjunction_name) {
            conjunction.inputs.insert(from.clone(), 0);
        }
    }
    modules
}

fn part1() -> Option<u64> {
    let mut modules = read_input();

    let mut lows = 0;
    let mut highs = 0;

    let mut signals: VecDeque<Signal> = VecDeque::new();

    for _ in 0..1000 {
        signals.push_front(("button".to_string(), "button".to_string(), 0));
        while let Some(signal) = signals.pop_back() {
            if signal.1 != "button".to_string() {
                if signal.2 == 1 {
                    highs += 1;
                } else {
                    lows += 1;
                }
            }

            if let Some(module) = modules.get_mut(&signal.1) {
                if let Some(new_signals) = module.process(signal) {
                    new_signals.into_iter().for_each(|signal| {
                        signals.push_front(signal);
                    });
                }
            } else {
                let output = Output {
                    name: signal.1,
                    lows: 0,
                };
                modules.insert(output.get_name(), Box::new(output));
            }
        }
    }
    let result = highs * lows;
    Some(result)
}

fn part2() -> Option<u64> {
    let mut modules = read_input();
    let mut signals: VecDeque<Signal> = VecDeque::new();
    let mut adders = HashMap::new();

    adders.insert("ph".to_string(), 0_u64);
    adders.insert("vn".to_string(), 0_u64);
    adders.insert("kt".to_string(), 0_u64);
    adders.insert("hn".to_string(), 0_u64);

    'outer: for button_presses in 1_u64.. {
        signals.push_front(("button".to_string(), "button".to_string(), 0));
        while let Some(signal) = signals.pop_back() {
            if let Some(module) = modules.get_mut(&signal.1) {
                if let Some(new_signals) = module.process(signal) {
                    new_signals.into_iter().for_each(|signal| {
                        signals.push_front(signal);
                    });
                }
            } else {
                let output = Output {
                    name: signal.1,
                    lows: 0,
                };
                modules.insert(output.get_name(), Box::new(output));
            }
            let mut completed = true;
            for (adder_name, count) in adders.iter_mut() {
                if let Some(adder_module) = modules.get_mut(adder_name) {
                    if adder_module.get_state() == 1 {
                        *count = button_presses
                    }
                } else {
                    println!("Missing module {adder_name}");
                }
                if *count == 0 {
                    completed = false
                }
            }
            if completed {
                break 'outer;
            }
        }
    }

    let result = adders.iter().map(|(_,count)| *count).reduce(|sum, count| sum * count);
    result
}

fn main() {
    println!("--- Day 20: Pulse Propagation ---");
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
