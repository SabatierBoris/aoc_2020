use std::io;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Toboggan {
    pos: Position,
    nb_trees: u32,
}

impl Toboggan {
    fn slope_one(mut self, map: Map) {
        let next_x = self.pos.x + 3;
        let next_y = self.pos.y + 1;

        let item = map.get(next_x, next_y);

        if item == '#' {
            self.nb_trees += 1;
        }

        self.pos = Position {
            x: next_x,
            y: next_y,
        };
    }

    fn slope(self, map: Map) {
        let mut i = 0;

        loop {
            //self.slope_one(map);

            i += 1;
            if i >= 11 {
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn get(self, x: usize, y: usize) -> char {
        self.0[y][x]
    }
}

fn main() {
    let mut pos = Position { x: 0, y: 0 };
    let mut cpt = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }

        if input.chars().nth(pos.x).unwrap() == '#' {
            cpt += 1;
        }

        let next_x = (pos.x + 3) % input.trim().len();
        let next_y = pos.y + 1;

        pos.x = next_x;
        pos.y = next_y;
        println!("{}", cpt);
    }
}
