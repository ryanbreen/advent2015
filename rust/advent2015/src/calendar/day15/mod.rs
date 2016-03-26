use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::boxed::Box;

#[derive(Clone)]
#[derive(Debug)]
struct Ingredient {
  name: String,
  capacity: i32,
  durability: i32,
  flavor: i32,
  texture: i32,
  calories: i32,
}

impl Ingredient {
  fn new(name: String, cap: i32, dur: i32, fla: i32, tex: i32, cal: i32) -> Ingredient {
    Ingredient { name: name, capacity: cap, durability: dur, flavor: fla, texture: tex, calories: cal }
  }
}

#[derive(Debug)]
#[derive(Clone)]
struct Mix {
  ingredient: Ingredient,
  count: u8,
}

#[derive(Debug)]
#[derive(Clone)]
struct Recipe {
  ingredient_mix: Box<Vec<Mix>>
}

impl Recipe {
  fn new(ingredients: &Vec<Ingredient>) -> Recipe {
    let mut ingredient_mix:Vec<Mix> = vec!();
    for ingredient in ingredients {
      ingredient_mix.push(Mix { ingredient: ingredient.clone(), count: 0 });
    }
    Recipe { ingredient_mix: Box::new(ingredient_mix) }
  }

  fn score(&self) -> i32 {
    let mut capacity:i32 = 0;
    let mut durability:i32 = 0;
    let mut flavor:i32 = 0;
    let mut texture:i32 = 0;
    for mix in self.ingredient_mix.iter() {
      capacity += mix.count as i32 * mix.ingredient.capacity;
      durability += mix.count as i32 * mix.ingredient.durability;
      flavor += mix.count as i32 * mix.ingredient.flavor;
      texture += mix.count as i32 * mix.ingredient.texture;
    }

    return if capacity > 0 { capacity } else { 0 } *
    if durability > 0 { durability } else { 0 } *
    if flavor > 0 { flavor } else { 0 } *
    if texture > 0 { texture } else { 0 };
  }
}

static mut high_score:i32 = 0;

fn set_ingredient_mix(recipe:&mut Recipe, remainder:u8, index: u8) {
  for i in 0..remainder {

    recipe.ingredient_mix[index as usize].count = i;

    if index+1 == recipe.ingredient_mix.len() as u8 {
      unsafe {
        let score = recipe.score();
        if score > high_score {
          high_score = score;
        }
      }
    } else {
      set_ingredient_mix(recipe, remainder - i, index + 1);
    }
  }
}

fn part1 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let mut ingredients: Vec<Ingredient> = vec!();

  let lines: Vec<&str> = buffer.lines().collect();
  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();

    let name = parts[0].split(':').next().unwrap().to_string();
    let capacity = parts[2].split(',').next().unwrap().parse::<i32>().unwrap();
    let durability = parts[4].split(',').next().unwrap().parse::<i32>().unwrap();
    let flavor = parts[6].split(',').next().unwrap().parse::<i32>().unwrap();
    let texture = parts[8].split(',').next().unwrap().parse::<i32>().unwrap();
    ingredients.push(Ingredient::new(name, capacity, durability, flavor, texture, 0));
  }

  let mut recipe:Recipe = Recipe::new(&ingredients);

  set_ingredient_mix(&mut recipe, 101, 0);

  unsafe {
    return high_score.to_string();
  }
}

fn part2 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  return buffer;
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day15/input".to_string(),
    part1: super::Puzzle {
      run: part1,
    },
    part2: super::Puzzle {
      run: part2,
    }
  };
}

#[test]
fn test_part1() {
  let day = fill();
  assert_eq!((day.part1.run)(day.input.to_string()), "2696".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "1084".to_string());
}
