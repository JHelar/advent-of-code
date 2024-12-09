use std::rc::Rc;

type Mem = Vec<Option<Rc<File>>>;
type Files = Vec<Rc<File>>;

#[derive(Debug, Clone)]
struct File {
    id: usize,
    mem_ptr: Vec<usize>,
    size: usize,
}

fn read_input() -> (Mem, Files) {
    let mut mem_line = String::new();
    std::io::stdin().read_line(&mut mem_line).unwrap();

    let mut mem = Vec::new();
    let mut files = Vec::new();

    for (id, file) in mem_line
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .enumerate()
    {
        let mut file_info = file.iter();
        let mem_ptr = mem.len();

        let size = file_info.next().unwrap().to_digit(10).unwrap() as usize;

        let file = Rc::new(File {
            id,
            mem_ptr: (mem_ptr..mem_ptr + size).collect(),
            size,
        });

        for _ in 0..file.size {
            mem.push(Some(Rc::clone(&file)));
        }
        files.push(file);

        if let Some(free_size_char) = file_info.next() {
            let free_size = free_size_char.to_digit(10).unwrap();
            for _ in 0..free_size {
                mem.push(None);
            }
        }
    }

    (mem, files)
}

fn print_mem(mem: &Mem) {
    for m in mem.iter() {
        match m {
            Some(_) => print!("X"),
            None => print!(" "),
        }
    }
    println!("\n")
}

fn file_fragment(mem: &mut Mem, files: &mut Files) {
    files.reverse();
    let mut empty_mem_ptr = 0;

    for file in files.iter_mut() {
        for file_mem_ptr in 0..file.size {
            if empty_mem_ptr >= file.mem_ptr[file_mem_ptr] {
                return;
            }

            for mem_ptr in empty_mem_ptr..mem.len() {
                match mem.get(mem_ptr).unwrap() {
                    Some(_) => {
                        empty_mem_ptr += 1;
                    }
                    None => {
                        *mem.get_mut(mem_ptr).unwrap() = Some(Rc::clone(file));
                        *mem.get_mut(file.mem_ptr[file_mem_ptr]).unwrap() = None;

                        Rc::make_mut(file).mem_ptr[file_mem_ptr] = empty_mem_ptr;

                        break;
                    }
                }
            }
        }
    }
}

fn file_move(mem: &mut Mem, files: &mut Files) {
    files.reverse();

    for file in files.iter_mut() {
        for mem_ptr in 0..mem.len() {
            if mem_ptr >= file.mem_ptr[0] {
                break;
            }
            if (mem_ptr..mem_ptr + file.size)
                .all(|block_mem_ptr| mem.get(block_mem_ptr).is_some_and(|block| block.is_none()))
            {
                for i in 0..file.size {
                    mem[mem_ptr + i] = Some(Rc::clone(&file));
                    mem[file.mem_ptr[i]] = None;
                    Rc::make_mut(file).mem_ptr[i] = mem_ptr + i;
                }
                break;
            }
        }
    }
}

fn checksum(mem: &Mem) -> usize {
    mem.iter()
        .enumerate()
        .filter_map(|(pos, space)| {
            if let Some(file) = space {
                Some(file.id * pos)
            } else {
                None
            }
        })
        .sum()
}

fn part1() -> Option<usize> {
    let (mut mem, mut files) = read_input();
    file_fragment(&mut mem, &mut files);

    print_mem(&mem);

    let result = checksum(&mem);
    Some(result)
}

fn part2() -> Option<usize> {
    let (mut mem, mut files) = read_input();
    file_move(&mut mem, &mut files);

    print_mem(&mem);

    let result = checksum(&mem);
    Some(result)
}

fn main() {
    println!("--- Day 9: Disk Fragmenter ---");
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
