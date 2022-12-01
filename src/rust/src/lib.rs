use extendr_api::prelude::*;

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
}
