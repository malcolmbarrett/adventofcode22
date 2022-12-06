use extendr_api::prelude::*;
use regex::Regex;
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

/// Calculate the solution to 2-a
/// @export
// #[extendr]
// fn _day_two_a_polars(input: &str) -> i32 {
//   let mut df = CsvReader::from_path(input)
//     .unwrap()
//     .has_header(false)
//     .with_delimiter(32)
//     .finish()
//     .unwrap();
//
//   df.set_column_names(&["theirs", "mine"]);
//
//   df.with_column(
//     df.column("theirs")
//       .unwrap()
//       .utf8()
//       .unwrap()
//       .into_iter()
//       .map(|letter| as_points(letter.unwrap()))
//       .collect::<Series>()
//     );
//
//     df.set_column_names(&["theirs", "mine", "their_score"]);
//
//     df.with_column(
//     df.column("mine")
//       .unwrap()
//       .utf8()
//       .unwrap()
//       .into_iter()
//       .map(|letter| as_points(letter.unwrap()))
//       .collect::<Series>()
//     );
//
//
//     df.set_column_names(&["theirs", "mine", "their_score", "my_score"]);
//
//
//   rprintln!("{df}");
//
//   15
// }

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
}
