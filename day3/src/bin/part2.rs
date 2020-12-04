use std::io;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn get(self, x: usize, y: usize) -> char {
        self.0[y][x]
    }

    fn height(self) -> usize {
        self.0.len()
    }
}

fn main() {
    let mut map = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }

        map.push(input.trim().chars().collect::<Vec<char>>());
    }

    //let map = Map{0:tmp};

    let slopes = vec![
        Position { x: 1, y: 1 },
        Position { x: 3, y: 1 },
        Position { x: 5, y: 1 },
        Position { x: 7, y: 1 },
        Position { x: 1, y: 2 },
    ];

    let height = map.len();
    let mut result = 1;
    for slope in slopes {
        let mut local_result = 0;
        println!("Slope {:?}", slope);
        let mut pos = Position { x: 0, y: 0 };
        loop {
            println!(" - {:?}", pos);
            if pos.y >= height {
                break;
            }

            if map[pos.y][pos.x % map[pos.y].len()] == '#' {
                local_result += 1;
            }

            pos.x += slope.x;
            pos.y += slope.y;
        }
        result *= local_result;
    }
    println!(" => {}", result);
}
