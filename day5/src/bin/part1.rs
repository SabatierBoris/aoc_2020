use std::cmp::max;
use std::io;

#[derive(Debug)]
struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn new(data: &str) -> Seat {
        let mut min_row = 0;
        let mut max_row = 127;

        let mut min_column = 0;
        let mut max_column = 7;

        for letter in data.chars() {
            match letter {
                'B' => {
                    min_row = max_row - ((max_row - min_row) / 2);
                }
                'F' => {
                    max_row = min_row + ((max_row - min_row) / 2);
                }
                'R' => {
                    min_column = max_column - ((max_column - min_column) / 2);
                }
                'L' => {
                    max_column = min_column + ((max_column - min_column) / 2);
                }
                _ => {
                    println!("??? {:?} ???", letter);
                }
            };
        }

        if min_row != max_row || min_column != max_column {
            println!("WTF !!!");
        }

        Seat {
            row: min_row,
            column: min_column,
        }
    }

    fn get_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

fn main() {
    let mut max_id = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }
        let s = Seat::new(input.trim());
        max_id = max(max_id, s.get_id());
    }
    println!("{}", max_id);
}
