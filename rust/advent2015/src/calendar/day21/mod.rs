
#[derive(Debug)]
struct Item {
  cost: u8,
  damage: u8,
  armor: u8
}

const WEAPONS:[Item;5] =
[
  Item { cost: 8, damage: 4, armor: 0}, /* dagger */
  Item { cost: 10, damage: 5, armor: 0}, /* shortsword */
  Item { cost: 25, damage: 6, armor: 0}, /* warhammer */
  Item { cost: 40, damage: 7, armor: 0}, /* longsword */
  Item { cost: 74, damage: 8, armor: 0}, /* greataxe */
];

const ARMOR:[Item;6] =
[
  Item { cost: 0, damage: 0, armor: 0}, /* noop */
  Item { cost: 13, damage: 0, armor: 1}, /* leather */
  Item { cost: 31, damage: 0, armor: 2}, /* chainmail */
  Item { cost: 53, damage: 0, armor: 3}, /* splintmail */
  Item { cost: 75, damage: 0, armor: 4}, /* bandedmail */
  Item { cost: 102, damage: 0, armor: 5}, /* platemail */
];

const RINGS:[Item;8] =
[
  Item { cost: 0, damage: 0, armor: 0}, /* noop */
  Item { cost: 0, damage: 0, armor: 0}, /* noop */
  Item { cost: 25, damage: 1, armor: 0}, /* damage +1 */
  Item { cost: 50, damage: 2, armor: 0}, /* damage +2 */
  Item { cost: 100, damage: 3, armor: 0}, /* damage +3 */
  Item { cost: 20, damage: 0, armor: 1}, /* defense +1 */
  Item { cost: 40, damage: 0, armor: 2}, /* defense +1 */
  Item { cost: 80, damage: 0, armor: 3}, /* defense +1 */
];

#[derive(Debug)]
struct Combatant {
  hp: u8,
  current_hp: i8,
  damage: u8,
  armor: u8
}

impl Combatant {
  fn new(hp: u8, dmg: u8, arm: u8) -> Self {
    Combatant {
      hp: hp,
      current_hp: hp as i8,
      damage: dmg,
      armor: arm,
    }
  }

  fn apply_item(&mut self, item: &Item) {
    self.damage += item.damage;
    self.armor += item.armor;
  }

  fn reset_hp(&mut self) {
    self.current_hp = self.hp as i8;
  }

  fn strip(&mut self) {
    self.damage = 0;
    self.armor = 0;
  }
}

fn fight (player: &mut Combatant, boss: &mut Combatant) -> bool {

  loop {

    // This is a definite loss.  Can't hug this bro to death.
    if player.damage < boss.armor {
      return false;
    }

    boss.current_hp -= (player.damage - boss.armor) as i8;
    if boss.current_hp <= 0 {
      return true;
    }

    // This is a definite win.  Sweet hug, bro.
    if boss.damage < player.armor {
      return true;
    }

    player.current_hp -= (boss.damage - player.armor) as i8;
    if player.current_hp <= 0 {
      return false;
    }
  }
}

fn part1 (input: String) -> String {
  let mut boss:Combatant = Combatant::new(103, 9, 2);

  let mut player:Combatant = Combatant::new(100, 0, 0);

  /*
  let mut test_boss:Combatant = Combatant::new(12, 7, 2);
  let mut test_player:Combatant = Combatant::new(8, 5, 5);
  println!("Win? {}", fight(&mut test_player, &mut test_boss));
  */

  let mut lowest_cost:u16 = 65535;

  for weapon in WEAPONS.into_iter() {
    for armor in ARMOR.into_iter() {

      let mut ring1_count = 0;
      let mut ring2_count = 0;

      for ring1 in RINGS.into_iter() {

        ring1_count += 1;

        for ring2 in RINGS.into_iter() {

          ring2_count += 1;

          if ring1_count == ring2_count {
            continue;
          }

          player.apply_item(weapon);
          player.apply_item(armor);
          player.apply_item(ring1);
          player.apply_item(ring2);

          // We won't be able to reduce boss HP at all.
          if player.damage <= 2 {
            continue;
          }

          if fight(&mut player, &mut boss) {            
            let cost:u16 = weapon.cost as u16 + armor.cost as u16 + ring1.cost as u16 + ring2.cost as u16;
            if cost < lowest_cost {
              lowest_cost = cost;
            }
          }

          player.strip();
          player.reset_hp();
          boss.reset_hp();
        }
      }
    }
  }

  return lowest_cost.to_string();
}

fn part2 (input: String) -> String {
  return input;
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "121".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "884520".to_string());
}
