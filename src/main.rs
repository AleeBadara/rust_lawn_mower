use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let contents = get_file_content("test.txt");
    let mut lawn: Lawn = Lawn::default();
    let mut mower_list: Vec<Mower> = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if index == 0 {
            lawn = get_lawn(line);
        } else if index % 2 != 0 {
            let mower = get_mower(line);
            mower_list.push(mower);
        } else if index % 2 == 0 {
            let mut current_mower = mower_list.pop().expect("No mower found");
            move_mower(line, &mut current_mower, &lawn);
            //println!("{:#?}", current_mower);
            current_mower.print_position();
        }
    }
}

fn get_file_content(file_name: &str) -> String {
    let path = Path::new(file_name);
    let display = path.display();
    let error_message = format!("Failed to open file {}", display);
    let mut file = File::open(&path).expect(&error_message);
    let mut contents = String::new();
    let error_message = format!("Couldn't read content of file {}", display);
    file.read_to_string(&mut contents).expect(&error_message);
    contents
}

fn get_lawn(input: &str) -> Lawn {
    let input_split: Vec<&str> = input.split(" ").collect();
    if input_split.len() != 2 {
        panic!(
            "Error geting lawn. You must specified a width and height for the lawn. Exemple: 5 5"
        );
    }
    Lawn::new(input_split[0], input_split[1])
}

fn get_mower(input: &str) -> Mower {
    let input_split: Vec<&str> = input.split(" ").collect();
    if input_split.len() != 3 {
        panic!("Error getting mower. You must specify a x, y coordinates and a direction. Exemple: 1 2 N");
    }
    Mower::new(input_split[0], input_split[1], input_split[2])
}

fn move_mower(instructions: &str, mower: &mut Mower, lawn: &Lawn) {
    for instruction in instructions.chars() {
        match instruction {
            'R' => {
                turn_right(mower);
            }
            'L' => {
                turn_left(mower);
            }
            'F' => {
                go_forward(mower, &lawn);
            }
            _ => {
                panic!("The instruction {} is incorrect. It must only contains the following caracters: RLF. Example: FFRFFRFRRF", instruction);
            }
        }
    }
}

fn turn_right(mower: &mut Mower) {
    match mower.direction {
        Direction::North => {
            mower.direction = Direction::East;
        }
        Direction::West => {
            mower.direction = Direction::North;
        }
        Direction::South => {
            mower.direction = Direction::West;
        }
        Direction::East => {
            mower.direction = Direction::South;
        }
    }
}

fn turn_left(mower: &mut Mower) {
    match mower.direction {
        Direction::North => {
            mower.direction = Direction::West;
        }
        Direction::West => {
            mower.direction = Direction::South;
        }
        Direction::South => {
            mower.direction = Direction::East;
        }
        Direction::East => {
            mower.direction = Direction::North;
        }
    }
}

fn go_forward(mower: &mut Mower, lawn: &Lawn) {
    match mower.direction {
        Direction::North => {
            if mower.y + 1 <= lawn.height {
                mower.y += 1;
            }
        }
        Direction::West => {
            if mower.x != 0 {
                mower.x -= 1;
            }
        }
        Direction::South => {
            if mower.y != 0 {
                mower.y -= 1;
            }
        }
        Direction::East => {
            if mower.x + 1 <= lawn.width {
                mower.x += 1;
            }
        }
    }
}

#[derive(Debug)]
struct Mower {
    x: u8,
    y: u8,
    direction: Direction,
}

impl Mower {
    fn new(input_x: &str, input_y: &str, input_direction: &str) -> Self {
        let x = input_x.parse::<u8>().expect("");
        let y = input_y.parse::<u8>().expect("");
        let direction = match input_direction {
            "N" => Direction::North,
            "E" => Direction::East,
            "W" => Direction::West,
            "S" => Direction::South,
            _ => {
                panic!("Error direction. The direction must be one of these caracters: N, E, S, W")
            }
        };
        Mower {
            x: x,
            y: y,
            direction: direction,
        }
    }
    fn print_position(&self) {
        let direction = match &self.direction {
            Direction::North => 'N',
            Direction::East => 'E',
            Direction::West => 'W',
            Direction::South => 'S',
        };
        println!("{} {} {}", &self.x, &self.y, direction);
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
struct Lawn {
    width: u8,
    height: u8,
}

impl Lawn {
    fn new(width_input: &str, height_input: &str) -> Self {
        let width = width_input
            .parse::<u8>()
            .expect("Error incorrect input. The width of the lawn must be an integer.");
        let height = height_input
            .parse::<u8>()
            .expect("Error incorrect input. The height of the lawn must be an integer.");
        Lawn {
            width: width,
            height: height,
        }
    }
    fn default() -> Self {
        Lawn {
            width: 0,
            height: 0,
        }
    }
}
