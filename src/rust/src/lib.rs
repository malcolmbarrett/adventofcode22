use extendr_api::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
// use polars::prelude::*;

/// Calculate the solution to 1-a
/// @export
#[extendr]
fn day_one_a(input: Vec<i32>) -> i32 {
  let elves: Vec<Elf> = input
    .into_iter()
    .fold(Vec::new(), |mut acc, calories| {
      if calories.is_na() || acc.is_empty() {
        acc.push(Vec::new());
      }

      if !calories.is_na() {
        acc.last_mut().unwrap().push(calories);
      }

      acc
    })
    .into_iter()
    .map(|calories| Elf { calories })
    .collect();

  let total_calories: Vec<i32> = elves
    .into_iter()
    .map(|elf| elf.calories.iter().sum())
    .collect();

  *total_calories
    .iter()
    .max()
    .unwrap()
}

/// Calculate the solution to 1-b
/// @export
#[extendr]
fn day_one_b(input: Vec<i32>) -> i32 {
  let elves: Vec<Elf> = input
    .into_iter()
    .fold(Vec::new(), |mut acc, calories| {
      if calories.is_na() || acc.is_empty() {
        acc.push(Vec::new());
      }

      if !calories.is_na() {
        acc.last_mut().unwrap().push(calories);
      }

      acc
    })
    .into_iter()
    .map(|calories| Elf { calories })
    .collect();

  let mut total_calories: Vec<i32> = elves
    .into_iter()
    .map(|elf| elf.calories.iter().sum())
    .collect();

  total_calories.sort();
  total_calories.pop().unwrap() +
    total_calories.pop().unwrap() +
    total_calories.pop().unwrap()
}

/// Calculate the solution to 2-a
/// @export
#[extendr]
fn day_two_a(theirs: Vec<String>, mine: Vec<String>) -> i32 {
  let their_score = to_points(theirs);
  let my_score = to_points(mine);

  let game_score: Vec<i32> = their_score
    .into_iter()
    .zip(my_score.into_iter())
    .map(|(x, y)| get_score(x, y))
    .collect();

  game_score
    .iter()
    .sum()
}

/// Calculate the solution to 2-b
/// @export
#[extendr]
fn day_two_b(theirs: Vec<String>, mine: Vec<String>) -> i32 {
  let their_score = to_points(theirs);
  let my_score = to_points(mine);

  let game_score: Vec<i32> = their_score
    .into_iter()
    .zip(my_score.into_iter())
    .map(|(x, y)| get_strategy(x, y))
    .map(|(x, y)| get_score(x, y))
    .collect();

  game_score
    .iter()
    .sum()
}

fn to_points(letters: Vec<String>) -> Vec<i32> {
  letters
    .iter()
    .map(|x| as_points(x))
    .collect()
}

fn get_strategy(x: i32, y: i32) -> (i32, i32) {
  if y == 1 {
    // lose
    let call = match x {
      1 => 3,
      2 => 1,
      3 => 2,
      _ => panic!("you're playing wrong!")
    };

    (x, call)
  } else if y == 2 {
    // draw
    (x, x)
  } else {
    //win
    let call = match x {
      1 => 2,
      2 => 3,
      3 => 1,
      _ => panic!("you're playing wrong!")
    };

    (x, call)
  }
}

fn get_score(x: i32, y: i32) -> i32 {
  let score = match x {
    1 => {
      if y == 1 {
        y + 3
      } else if y == 2 {
        y + 6
      } else {
        y
      }
    },
    2 => {
      if y == 1 {
        y
      } else if y == 2 {
        y + 3
      } else {
        y + 6
      }
    },
    3 => {
      if y == 1 {
        y + 6
      } else if y == 2 {
        y
      } else {
        y + 3
      }
    },
    _ => panic!("You're playing wrong!")
  };

  score
}

fn as_points(letter: &str) -> i32 {
  match letter {
    "A" | "X" => 1,
    "B" | "Y" => 2,
    "C" | "Z" => 3,
    _ => panic!("not the right letter!")
  }
}

#[derive(Debug)]
struct Elf {
  calories: Vec<i32>
}

/// Calculate the solution to 2-b
/// @export
#[extendr]
fn day_three_a(contents: Vec<String>) -> i32 {
  contents
    .iter()
    .map(|content| {
      let halfway = content.chars().count() / 2;
      let (first, second) = content.split_at(halfway);
      for letter in first.chars() {
        if second.contains(letter) {
          let mut score = letter.to_owned() as i32;
          score = score - 96;
          if score < 0 {
            score = score + 58;
          }

          return score
        }
      }

      unreachable!()
    })
    .sum()
}

/// Calculate the solution to 2-b
/// @export
#[extendr]
fn day_three_b(contents: Vec<String>) -> i32 {
  contents
    .chunks(3)
    .map(|group| {
      for letter in group[0].chars() {
        if group[1].contains(letter) && group[2].contains(letter) {
          let mut score = letter.to_owned() as i32;
          score = score - 96;
          if score < 0 {
            score = score + 58;
          }

          return score
        }
      }

      unreachable!()
    })
    .sum()
}


/// Calculate the solution to 4-a
/// @export
#[extendr]
fn day_four_a(first: Vec<String>, second: Vec<String>) -> i32 {
  let first = split_string(first);
  let second = split_string(second);

  first
    .iter()
    .zip(second.iter())
    .filter(|(a, b)| {
      let range_a = a.0..a.1+1;
      let range_b = b.0..b.1+1;

      range_a.contains(&b.0) && range_a.contains(&b.1) ||
        range_b.contains(&a.0) && range_b.contains(&a.1)
    })
    .count() as i32
}

/// Calculate the solution to 4-a
/// @export
#[extendr]
fn day_four_b(first: Vec<String>, second: Vec<String>) -> i32 {
  let first = split_string(first);
  let second = split_string(second);

  first
    .iter()
    .zip(second.iter())
    .filter(|(a, b)| {
      let range_a = a.0..a.1+1;
      let range_b = b.0..b.1+1;

      range_a.contains(&b.0) || range_a.contains(&b.1) ||
        range_b.contains(&a.0) || range_b.contains(&a.1)
    })
    .count() as i32
}

fn split_string(x: Vec<String>) -> Vec<(i32, i32)> {
  x
    .iter()
    .map(|x| {
      let pair: Vec<&str> = x.split("-").collect();
      (pair[0].parse::<i32>().unwrap(),
       pair[1].parse::<i32>().unwrap())
    })
    .collect()
}

/// Calculate the solution to 5-a
/// @export
#[extendr]
fn day_five_a(stacks: List, directions: Vec<String>) -> String {
  let numbers: Vec<Vec<i32>> = directions
    .into_iter()
    .map(|x| capture_numbers(&x))
    .collect();

  let mut stacks: Vec<Vec<String>> = stacks
    .into_iter()
    .map(|x| x.1.as_string_vector().unwrap())
    .map(|x| x.into_iter().filter(|y| y != "NA").rev().collect())
    .collect();

  for direction in numbers.iter() {
    for _ in 0..direction[0] {
      let moved_box = stacks[direction[1] as usize].pop().unwrap();
      stacks[direction[2] as usize].push(moved_box);
    }
  }

  let mut top_boxes = String::new();

  for mut stack in stacks {
    let top_box = stack.pop().unwrap();
    top_boxes.push_str(&top_box);
  }

  top_boxes
}

/// Calculate the solution to 5-b
/// @export
#[extendr]
fn day_five_b(stacks: List, directions: Vec<String>) -> String {
  let numbers: Vec<Vec<i32>> = directions
    .into_iter()
    .map(|x| capture_numbers(&x))
    .collect();

  let mut stacks: Vec<Vec<String>> = stacks
    .into_iter()
    .map(|x| x.1.as_string_vector().unwrap())
    .map(|x| x.into_iter().filter(|y| y != "NA").rev().collect())
    .collect();

  for direction in numbers.iter() {
    let index = stacks[direction[1] as usize].len() - (direction[0] as usize);
    let mut moved_boxes = stacks[direction[1] as usize]
    .split_off(index);

    stacks[direction[2] as usize].append(&mut moved_boxes);
  }

  let mut top_boxes = String::new();

  for mut stack in stacks {
    let top_box = stack.pop().unwrap();
    top_boxes.push_str(&top_box);
  }

  top_boxes
}

fn capture_numbers(text: &String) -> Vec<i32> {
  let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  let mut numbers = Vec::new();
  let caps = pattern.captures(text).unwrap();

  numbers.push(caps.get(1).map_or("", |m| m.as_str()).parse::<i32>().unwrap());
  numbers.push(caps.get(2).map_or("", |m| m.as_str()).parse::<i32>().unwrap() - 1);
  numbers.push(caps.get(3).map_or("", |m| m.as_str()).parse::<i32>().unwrap() - 1);

  numbers
}

/// Calculate the solution to 6-1
/// @export
#[extendr]
fn day_six_a(input: String) -> usize {
  let mut letters: Vec<&str> = input.split("").collect();
  letters.remove(0);
  letters.pop();
  let mut pattern;
  for marker in letters.windows(4) {
    pattern = marker.concat();
    let mut marker = marker.to_owned();
    marker.sort();
    marker.dedup();
    if marker.len() == 4 {
      return input.find(&pattern).unwrap() + 4
    }
  }

  unreachable!()
}

/// Calculate the solution to 6-2
/// @export
#[extendr]
fn day_six_b(input: String) -> usize {
  let mut letters: Vec<&str> = input.split("").collect();
  letters.remove(0);
  letters.pop();
  let mut pattern;
  for marker in letters.windows(14) {
    pattern = marker.concat();
    let mut marker = marker.to_owned();
    marker.sort();
    marker.dedup();
    if marker.len() == 14 {
      return input.find(&pattern).unwrap() + 14
    }
  }

  unreachable!()
}

/// Calculate the solution to 7-1
/// @export
#[extendr]
fn day_seven_a(input: Vec<String>) -> usize {
  parse_sizes(&input)
    .into_values()
    .filter(|&size| size <= 100000)
    .sum()
}


/// Calculate the solution to 7-2
/// @export
#[extendr]
fn day_seven_b(input: Vec<String>) -> usize {
  let sizes = parse_sizes(&input);
  let total = sizes[""];

  sizes
    .into_values()
    .filter(|size| 70000000 - (total - size) >= 30000000)
    .min()
    .unwrap_or(total)
}

fn parse_sizes(lines: &Vec<String>) -> HashMap<String, usize> {
  let mut sizes: HashMap<String, usize> = HashMap::new();

  let mut command = "".to_string();
  for line in lines {
      if let Some(dir) = line.strip_prefix("$ cd ") {
          command = match dir {
              "/" => "".to_string(),
              ".." => command.rsplit_once('/')
                .map_or("", |(command, _)| command)
                .to_string(),
              _ => {
                  if command.is_empty() {
                      dir.to_string()
                  } else {
                      format!("{}/{}", command, dir)
                  }
              }
          };
      } else if let Some(size) = line
          .find(|c: char| !c.is_ascii_digit())
          .and_then(|i| line[..i].parse::<usize>().ok())
      {
          let mut path = command.to_string();
          loop {
              sizes
                  .entry(path.to_string())
                  .and_modify(|s| *s += size)
                  .or_insert(size);
              if path.is_empty() {
                  break;
              }
              path.drain(path.rfind('/').unwrap_or(0)..);
          }
      }
  }

  sizes
}

/// Calculate the solution to 10-1
/// @export
#[extendr]
fn day_ten_a(input: Vec<String>, indices: Vec<i32>) -> i32 {
  let state = collect_state(input);

  indices
    .iter()
    .map(|i| i * state[(i - 1) as usize])
    .sum()
}

/// Calculate the solution to 10-2
/// @export
#[extendr]
fn day_ten_b(input: Vec<String>) -> i32 {
  let state = collect_state(input);

  for (cycle, x) in state.iter().enumerate() {
      if cycle % 40 == 0 {
          rprintln!();
      }

      let sprite_positions = [x - 1, *x, x + 1];

      if sprite_positions.contains(&((cycle % 40) as i32)) {
          rprint!("\u{2593}");
      } else {
          rprint!("\u{2591}");
      }
  }

  rprintln!();

  1
}

fn collect_state(input: Vec<String>) -> Vec<i32>{
  let commands: Vec<Command> = input
    .into_iter()
    .map(|s| s.parse().unwrap())
    .collect();

  let mut state: Vec<i32> = vec![1];

  for cmd in commands.iter() {
    let (new_size, add_x) = match cmd {
      Command::Addx(x) => (state.len() + 2, x),
      Command::Noop => (state.len() + 1, &0)
    };

    state.resize(new_size, state[state.len() - 1]);

    let last = state.len() - 1;
    state[last] = state[last] + add_x;
  }

  state
}

#[derive(Debug)]
enum Command {
  Noop,
  Addx(i32)
}

impl FromStr for Command {
  type Err = extendr_api::Error;
  fn from_str(s: &str) -> Result<Self> {
    if s == "noop" {
      return Ok(Command::Noop)
    }

    let x: i32 = s.strip_prefix("addx ").unwrap().parse().unwrap();

    Ok(Command::Addx(x))
  }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
  mod adventofcode22;
  fn day_one_a;
  fn day_one_b;
  fn day_two_a;
  fn day_two_b;
  fn day_three_a;
  fn day_three_b;
  fn day_four_a;
  fn day_four_b;
  fn day_five_a;
  fn day_five_b;
  fn day_six_a;
  fn day_six_b;
  fn day_seven_a;
  fn day_seven_b;
  fn day_ten_a;
  fn day_ten_b;
}

