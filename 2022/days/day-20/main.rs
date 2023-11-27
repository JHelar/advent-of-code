use core::num;
use std::env;
use std::fmt::Display;
use std::fs;
use std::ptr::NonNull;

#[derive(Debug, Clone, Copy)]
struct Node {
    value: i64,
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
}

impl Node {
    fn new(value: i64) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }

    fn move_forward(&mut self) {
        unsafe {
            let prev = self.prev;
            let next = self.next;
            let next_next = (*self.next.unwrap().as_ptr()).next;

            self.prev = next;
            self.next = next_next;

            let self_ref = Some(NonNull::from(self));
            
            (*next_next.unwrap().as_ptr()).prev = self_ref;
            
            (*next.unwrap().as_ptr()).next = self_ref;
            (*next.unwrap().as_ptr()).prev = prev;

            (*prev.unwrap().as_ptr()).next = next;
        }
    }

    fn move_backward(&mut self) {
        unsafe {
            let prev = self.prev;
            let next = self.next;
            let prev_prev = (*self.prev.unwrap().as_ptr()).prev;

            self.next = prev;
            self.prev = prev_prev;

            let self_ref = Some(NonNull::from(self));
            
            (*prev_prev.unwrap().as_ptr()).next = self_ref;
            
            (*prev.unwrap().as_ptr()).next = next;
            (*prev.unwrap().as_ptr()).prev = self_ref;

            (*next.unwrap().as_ptr()).prev = prev;
        }
    }
}

struct MixingList {
    zero: Option<NonNull<Node>>,
    head: Option<NonNull<Node>>,
    cursor: Option<NonNull<Node>>,
    len: i64
}

impl MixingList {
    fn new() -> Self {
        Self {
            head: None,
            cursor: None,
            zero: None,
            len: 0,
        }
    }

    fn push_back(&mut self, value: i64) -> NonNull<Node> {
        let node = Box::new(Node::new(value));
        let new_node_ptr = NonNull::from(Box::leak(node));

        let new_node = Some(new_node_ptr);

        if value == 0 {
            self.zero = new_node;
        }

        if self.head.is_none() {
            self.head = new_node;
        }

        unsafe {
            match self.cursor {
                None => {
                    (*new_node_ptr.as_ptr()).next = self.head;
                    (*new_node_ptr.as_ptr()).prev = self.head;
                }
                Some(prev_node) => {
                    (*prev_node.as_ptr()).next = new_node;

                    (*new_node_ptr.as_ptr()).prev = Some(prev_node);
                    (*new_node_ptr.as_ptr()).next = self.head;
                    (*self.head.unwrap().as_ptr()).prev = new_node;
                }
            }
        }
        self.cursor = new_node;
        self.len += 1;

        new_node_ptr
    }

    fn move_node(&mut self, node: NonNull<Node>) {
        unsafe {
            let value = (*node.as_ptr()).value;
            let move_amount = value.abs() % (self.len - 1);

            (0..move_amount).for_each(|_x| {
                if value > 0 {
                    (*node.as_ptr()).move_forward();
                } else {
                    (*node.as_ptr()).move_backward();
                }
            })
        }
    }

    fn get_node_value_from_position(&mut self, position: i32) -> i64 {
        let zero = self.zero.unwrap();
        let mut cursor = zero;
        unsafe {
            (0..position).for_each(|_x| {
                if position > 0 {
                    cursor = (*cursor.as_ptr()).next.unwrap();
                } else {
                    cursor = (*cursor.as_ptr()).prev.unwrap();
                }
            })
        }

        Self::node_value(cursor)
    }

    fn node_value(node: NonNull<Node>) -> i64 {
        unsafe { (*node.as_ptr()).value }
    }
}

impl Display for MixingList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let head = self.head.unwrap();
            let mut current_node = head;

            loop {
                write!(f, "{}", Self::node_value(current_node)).unwrap();

                current_node = (*current_node.as_ptr()).next.unwrap();

                if current_node == head {
                    break;
                } else {
                    write!(f, ", ").unwrap();
                }
            }

            Ok(())
        }
    }
}

fn parse_input() -> (Vec<NonNull<Node>>, MixingList) {
    let mut list = MixingList::new();
    let mut nodes = Vec::new();
    fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .for_each(|num| {
            let node = list.push_back(num);
            nodes.push(node);
        });

    (nodes, list)
}

fn part1() {
    let (nums, mut list) = parse_input();

    for node in nums.iter() {
        list.move_node(*node);
    }

    let result: i64 = vec![1000, 2000, 3000].iter().map(|position| list.get_node_value_from_position(*position)).sum();

    println!("Result: {}", result);
}

fn part2() {
    let (nums, mut list) = parse_input();

    nums.iter().for_each(|node| {
        unsafe {
            (*node.as_ptr()).value *= 811589153;
        }
    });

    for _i in 0..10 {
        for node in nums.iter() {
            list.move_node(*node);
        }
    }

    let result: i64 = vec![1000, 2000, 3000].iter().map(|position| list.get_node_value_from_position(*position)).sum();

    println!("Result: {}", result);
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
