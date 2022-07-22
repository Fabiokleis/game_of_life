#![allow(dead_code)]
use std::io::{self, Write};
use std::path::Path;
use std::{char, fs};

use rand::{thread_rng, Rng};

type Cell = Vec<Vec<char>>;

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    matrix: Cell,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            width,
            height,
            matrix: vec![vec!['.'; width]; height],
        }
    }

    fn show(&self) {
        for i in self.matrix.iter() {
            for j in i.iter() {
                print!("{}", j);
            }
            println!("");
        }
    }

    fn apply_board(&mut self, name: String) {
        let file_path = Path::new("/home/urameshi/git_lab/game_of_life/").join(Path::new(&name.trim()));
    
        let file_content = fs::read_to_string(file_path).expect("cannot read file");

        let mut i = 0;
        let mut j = 0;
        for line in file_content.lines() {
            for ele in line.chars() {
                match ele {
                    '1' => self.matrix[i][j] = '*',
                    '2' => self.matrix[i][j] = 'c',
                    '0' => self.matrix[i][j] = '.',
                    _ => {}
                }
                if ele != ',' {
                    j += 1;
                }
            }
            i += 1;
            j = 0;
        }
    }

    fn run(&mut self) {
        let mut aux = vec![vec!['.'; self.width]; self.height];
        for (i, vc) in self.matrix.iter().enumerate() {
            for (j, c) in vc.iter().enumerate() {
                let mut s = 0;
                let mut s2 = 0;
                for u in -1..2 as i32 {
                    for k in -1..2 as i32 {
                        if u == 0 && k == 0 {
                            continue;
                        }
                        if (i as i32 - u).is_negative()
                            || (i as i32 - u) == self.width as i32
                            || (j as i32 - k).is_negative()
                            || (j as i32 - k) == self.height as i32
                        {
                            continue;
                        }

                        if self.matrix[(i as i32 - u) as usize][(j as i32 - k) as usize] == '*' {
                            s += 1;
                        } else if self.matrix[(i as i32 - u) as usize][(j as i32 - k) as usize]
                            == 'c'
                        {
                            s2 += 1;
                        }
                    }
                }

                match c {
                    '*' => {
                        if s < 2 {
                            aux[i][j] = '.';
                        } else if s > 3 {
                            aux[i][j] = '.';
                        } else if s == 3 || s == 2 {
                            aux[i][j] = '*';
                        }
                    },
                   'c' => {
                        if s2 < 2 {
                            aux[i][j] = '.';
                        } else if s2 > 3 {
                            aux[i][j] = '.';
                        } else if s2 == 3 || s2 == 2 {
                            aux[i][j] = 'c';
                        }
                    },
                    '.' => {
                        if s == 3 {
                            aux[i][j] = '*';
                        } else if s2 == 3 {
                            aux[i][j] = 'c';
                        }
                    },
 
                    _ => {
                        aux[i][j] = '.';
                    }
                }
            }
        }

        self.matrix = aux;
    }

    fn apply_board_random(&mut self) {

        for v in self.matrix.iter_mut() {
            for c in v.iter_mut() {
                let rng = thread_rng().gen_bool(2.0/3.0); // 2/3% of chance

                if rng {
                    *c = '*';
                } else {
                    *c = '.';
                }
            }
        }
    }
}

/// game loop
fn run(board: &mut Board, random: String) {
    if !random.is_empty() {
        board.apply_board_random();
    } else {
        print!("file name (assets/any): ");
        let mut name = String::new();
        io::stdout().flush().expect("cannot flush stdout!");

        std::io::stdin().read_line(&mut name).unwrap();
        board.apply_board(name);
    }
    
    'running: loop {
        // clear terminal stdout
        std::thread::sleep(std::time::Duration::from_millis(100));
        std::process::Command::new("clear").status().unwrap();
        board.run();
        board.show();
        io::stdout().flush().expect("cannot flush stdout!");
    }
}

fn main() {
    
    let mut args = std::env::args();
    args.next();
    let width = args.next().unwrap();
    let height = args.next().unwrap();
    let random = match args.next() {
        Some(r) => r,
        None => String::from(""),
    };

    let mut board = Board::new(width.parse::<usize>().unwrap(), height.parse::<usize>().unwrap()); 
    run(&mut board, random);
}
