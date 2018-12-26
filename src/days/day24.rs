use crate::prelude::*;
use std::collections::BTreeSet;

pub fn run(data: &AocData) -> AocResult {
    let mut game_state = GameState::parse(data)?;
    let infection_left = {
        let mut game_state = game_state.clone();
        game_state.fight_battle();
        game_state.total_num_units()
    };
    let mut min_boost = 0;
    let mut max_boost = 1;
    while !test_boost(&game_state, max_boost) {
        min_boost = max_boost;
        max_boost *= 2;
    }
    while min_boost + 1 < max_boost {
        let mid_boost = (min_boost + max_boost) / 2;
        if test_boost(&game_state, mid_boost) {
            max_boost = mid_boost;
        } else {
            min_boost = mid_boost;
        }
    }
    game_state.boost(max_boost);
    game_state.fight_battle();
    answers(infection_left, game_state.total_num_units())
}

fn test_boost(game_state: &GameState, boost: u32) -> bool {
    let mut game_state = game_state.clone();
    game_state.boost(boost);
    game_state.fight_battle();
    game_state.has_immune_system_won()
}

#[derive(Debug, Clone)]
struct Group {
    num_units: u32,
    hp: u32,
    weakness: BTreeSet<String>,
    immunity: BTreeSet<String>,
    dp: u32,
    damage_type: String,
    initiative: u32,
}

impl Group {
    fn ep(&self) -> u32 {
        self.num_units * self.dp
    }
}

#[derive(Debug, Clone)]
struct GameState {
    immune_system: Vec<Group>,
    infection: Vec<Group>,
}

impl GameState {
    fn parse(data: &AocData) -> Result<GameState> {
        let re = Regex::new(r"(\d+) units each with (\d+) hit points (\([^\)]*\))? ?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        let attr_re = Regex::new(r"(weak|immune) to ([^\);]+)").unwrap();

        let mut is_immune_system = true;
        let mut immune_system = Vec::new();
        let mut infection = Vec::new();

        for line in data.lines()? {
            if line == "Immune System:" {
                is_immune_system = true;
            } else if line == "Infection:" {
                is_immune_system = false;
            } else if let Some(caps) = re.captures(&line) {
                let mut weakness = BTreeSet::new();
                let mut immunity = BTreeSet::new();
                if let Some(attrs) = caps.get(3) {
                    for caps in attr_re.captures_iter(attrs.as_str()) {
                        let iter = caps[2].split(",").map(|t| t.trim().to_string());
                        if &caps[1] == "weak" {
                            weakness.extend(iter);
                        } else {
                            immunity.extend(iter);
                        }
                    }
                }
                let group = Group {
                    num_units: caps[1].parse().unwrap(),
                    hp: caps[2].parse().unwrap(),
                    weakness,
                    immunity,
                    dp: caps[4].parse().unwrap(),
                    damage_type: caps[5].into(),
                    initiative: caps[6].parse().unwrap(),
                };
                if is_immune_system {
                    immune_system.push(group);
                } else {
                    infection.push(group);
                }
            } else if !line.is_empty() {
                bail!("Failed to parse line: {}", line);
            }
        }

        Ok(GameState {
            immune_system,
            infection,
        })
    }

    fn boost(&mut self, boost: u32) {
        for group in self.immune_system.iter_mut() {
            group.dp += boost;
        }
    }

    fn fight_battle(&mut self) {
        while !self.is_battle_over() {
            let num_units = self.total_num_units();
            self.fight_round();
            if num_units == self.total_num_units() {
                break;
            }
        }
    }

    fn fight_round(&mut self) {
        let immune_system_targets = Self::target_selection(&self.immune_system, &self.infection);
        let infection_targets = Self::target_selection(&self.infection, &self.immune_system);

        let mut attack_order: Vec<(usize, bool)> = (0..immune_system_targets.len())
            .map(|i| (i, true))
            .chain((0..infection_targets.len()).map(|i| (i, false)))
            .collect();
        attack_order.sort_unstable_by_key(|&(i, is_immune_system)| {
            if is_immune_system {
                self.immune_system[i].initiative
            } else {
                self.infection[i].initiative
            }
        });

        for (index, is_immune_system) in attack_order.into_iter().rev() {
            if is_immune_system {
                if let Some(target) = immune_system_targets[index] {
                    attack(&self.immune_system[index], &mut self.infection[target]);
                }
            } else {
                if let Some(target) = infection_targets[index] {
                    attack(&self.infection[index], &mut self.immune_system[target]);
                }
            }
        }

        self.immune_system.retain(|g| g.num_units > 0);
        self.infection.retain(|g| g.num_units > 0);
    }

    fn is_battle_over(&self) -> bool {
        self.immune_system.is_empty() || self.infection.is_empty()
    }

    fn has_immune_system_won(&self) -> bool {
        self.infection.is_empty()
    }

    fn total_num_units(&self) -> u32 {
        self.immune_system
            .iter()
            .chain(self.infection.iter())
            .map(|g| g.num_units)
            .sum()
    }

    fn target_selection(attackers: &[Group], defenders: &[Group]) -> Vec<Option<usize>> {
        let mut attack_order: Vec<usize> = (0..attackers.len()).collect();
        attack_order.sort_unstable_by(|&a, &b| {
            let ga = &attackers[a];
            let gb = &attackers[b];
            let epa = ga.ep();
            let epb = gb.ep();
            if epa == epb {
                gb.initiative.cmp(&ga.initiative)
            } else {
                epb.cmp(&epa)
            }
        });

        let mut targets: Vec<usize> = (0..defenders.len()).collect();
        let mut selected_targets = vec![None; attackers.len()];

        for attacker_index in attack_order.into_iter() {
            let attacker = &attackers[attacker_index];

            targets.sort_unstable_by(|&a, &b| {
                let ga = &defenders[a];
                let gb = &defenders[b];
                let da = damage(attacker, ga);
                let db = damage(attacker, gb);
                if da == db {
                    let epa = ga.ep();
                    let epb = gb.ep();
                    if epa == epb {
                        ga.initiative.cmp(&gb.initiative)
                    } else {
                        epa.cmp(&epb)
                    }
                } else {
                    da.cmp(&db)
                }
            });

            let any_damage = targets
                .iter()
                .rev()
                .any(|&d| damage(attacker, &defenders[d]) > 0);
            if any_damage {
                selected_targets[attacker_index] = targets.pop();
            }
        }

        selected_targets
    }
}

fn damage(attacker: &Group, defender: &Group) -> u32 {
    if defender.immunity.contains(&attacker.damage_type) {
        0
    } else if defender.weakness.contains(&attacker.damage_type) {
        attacker.ep() * 2
    } else {
        attacker.ep()
    }
}

fn attack(attacker: &Group, defender: &mut Group) {
    let damage = damage(attacker, defender);
    if damage > 0 {
        let num_kills = damage / defender.hp;
        defender.num_units = defender.num_units.saturating_sub(num_kills);
    }
}

#[cfg(test)]
#[test]
fn test() {
    let data = AocData::from_str("
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
    ");

    let mut game_state = GameState::parse(&data).unwrap();
    while !game_state.is_battle_over() {
        game_state.fight_round();
    }
    assert_eq!(game_state.total_num_units(), 5216);
}
