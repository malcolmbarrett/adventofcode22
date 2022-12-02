use extendr_api::prelude::*;
use polars::prelude::*;

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
  let their_score: Vec<i32> = theirs
    .iter()
    .map(|x| as_points(x))
    .collect();

  let my_score: Vec<i32> = mine
    .iter()
    .map(|x| as_points(x))
    .collect();

  let game_score: Vec<i32> = their_score
    .into_iter()
    .zip(my_score.into_iter())
    .map(|(x, y)| {
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
    }).collect();

    game_score
      .iter()
      .sum()
}

/// Calculate the solution to 2-a
/// @export
#[extendr]
fn _day_two_a_polars(input: &str) -> i32 {
  let mut df = CsvReader::from_path(input)
    .unwrap()
    .has_header(false)
    .with_delimiter(32)
    .finish()
    .unwrap();

  df.set_column_names(&["theirs", "mine"]);

  df.with_column(
    df.column("theirs")
      .unwrap()
      .utf8()
      .unwrap()
      .into_iter()
      .map(|letter| as_points(letter.unwrap()))
      .collect::<Series>()
    );

    df.set_column_names(&["theirs", "mine", "their_score"]);

    df.with_column(
    df.column("mine")
      .unwrap()
      .utf8()
      .unwrap()
      .into_iter()
      .map(|letter| as_points(letter.unwrap()))
      .collect::<Series>()
    );


    df.set_column_names(&["theirs", "mine", "their_score", "my_score"]);


  rprintln!("{df}");

  15
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

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod adventofcode22;
    fn day_one_a;
    fn day_one_b;
    fn day_two_a;
}
