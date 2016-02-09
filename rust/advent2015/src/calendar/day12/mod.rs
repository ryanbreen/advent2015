use rustc_serialize::json::Json;
use std::collections::BTreeMap;

fn traverse_array (arr: &Vec<Json>, total: &mut i64) {
  for value in arr.iter() {
    match *value {
      Json::I64(v) => *total += v,
      Json::U64(v) => *total += v as i64,
      Json::Object(ref v) => traverse_obj(v, total),
      Json::Array(ref a) => traverse_array(a, total),
      _ => {}
    };
  }
}

fn traverse_obj (obj: &BTreeMap<String, Json>, total: &mut i64) {
  for (_, value) in obj.iter() {
    match *value {
      Json::I64(v) => *total += v,
      Json::U64(v) => *total += v as i64,
      Json::Object(ref v) => traverse_obj(v, total),
      Json::Array(ref a) => traverse_array(a, total),
      _ => {}
    };
  }
}

fn part1 (input: String) -> String {
  let data = Json::from_str(&input).unwrap();
  let obj = data.as_object().unwrap();

  let mut total:i64 = 0;

  traverse_obj(&obj, &mut total);

  return total.to_string();
}

fn traverse_no_red_array (arr: &Vec<Json>, total: &mut i64) {
  for value in arr.iter() {
    match *value {
      Json::I64(v) => *total += v,
      Json::U64(v) => *total += v as i64,
      Json::Object(ref v) => traverse_no_red_obj(v, total),
      Json::Array(ref a) => traverse_no_red_array(a, total),
      _ => {}
    };
  }
}

fn traverse_no_red_obj (obj: &BTreeMap<String, Json>, total: &mut i64) {

  for (_, value) in obj.iter() {
    match *value {
      Json::String(ref v) => {
        if v == "red" {
          return;
        }
      },
      _ => {}
    };
  }

  for (_, value) in obj.iter() {
    match *value {
      Json::I64(v) => *total += v,
      Json::U64(v) => *total += v as i64,
      Json::Object(ref v) => traverse_no_red_obj(v, total),
      Json::Array(ref a) => traverse_no_red_array(a, total),
      _ => {}
    };
  }
}

fn part2 (input: String) -> String  {
  let data = Json::from_str(&input).unwrap();
  let obj = data.as_object().unwrap();

  let mut total:i64 = 0;

  traverse_no_red_obj(&obj, &mut total);

  return total.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: include_str!("input").to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "119433".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "909".to_string());
}
