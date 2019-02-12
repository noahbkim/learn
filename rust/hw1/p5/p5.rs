use std::fs;
use std::env;
use std::ops;

#[derive(Debug)]
struct Position {
  x: i32,
  y: i32,
}

const CARDINALS: [Position; 4] = [
  Position {x: 1, y: 0},
  Position {x: -1, y: 0},
  Position {x: 0, y: 1},
  Position {x: 0,  y: -1}
];

const CARDINAL_NAMES: [&str; 4] = ["down", "up", "right", "left"];

impl Position {
  fn new(x: i32, y: i32) -> Position {
    Position {x, y}
  }
}

impl ops::Add<Position> for Position {
  type Output = Position;
  fn add(self, other: Position) -> Position {
    Position::new(self.x + other.x, self.y + other.y)
  }
}

impl ops::Add<&Position> for Position {
  type Output = Position;
  fn add(self, other: &Position) -> Position {
    Position::new(self.x + other.x, self.y + other.y)
  }
}

impl ops::Add<&Position> for &Position {
  type Output = Position;
  fn add(self, other: &Position) -> Position {
    Position::new(self.x + other.x, self.y + other.y)
  }
}

fn parse(data: &String) -> Vec<Vec<char>> {
  let mut grid: Vec<Vec<char>> = Vec::new();
  for line in data.split("\n") {
    let mut row: Vec<char> = Vec::new();
    for element in line.split(" ") {
      row.push(element.chars().next().unwrap());
    }
    grid.push(row);
  }
  return grid;
}

fn verify(grid: &Vec<Vec<char>>, cursor: &Position, direction: &Position, word: &String, index: usize) -> bool {
  return index == word.len() ||
    cursor.x >= 0 && (cursor.x as usize) < grid.len() &&
    cursor.y >= 0 && (cursor.y as usize) < grid[0].len() &&
    grid[cursor.x as usize][cursor.y as usize] == word.chars().nth(index).unwrap() &&
    verify(grid, &(cursor + direction), direction, word, index + 1);
}

fn search(grid: &Vec<Vec<char>>, word: &String) {
  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      if grid[i][j] == word.chars().next().unwrap() {
        for k in 0..4 {
          let direction: &Position = &CARDINALS[k];
          let cursor: Position = Position::new(i as i32, j as i32) + direction;
          if verify(grid, &cursor, direction, word, 1) {
            println!("found at {}, {} going {}", i, j, CARDINAL_NAMES[k]);
          }
        }
      }
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
      println!("missing file path and word!");
      return;
  }

  let data: String = fs::read_to_string(&args[1]).expect("unable to read file!");
  let grid: Vec<Vec<char>> = parse(&data);
  search(&grid, &args[2]);
}

// Example input
// a p s d z
// c d n i b
// s n o w a
// j b w o w
//
// $ ./p5 grid.txt won
// found at 2, 3 going left
// found at 3, 2 going up
