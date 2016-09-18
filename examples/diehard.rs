extern crate genetic_planner;
use genetic_planner::genetic_planner as gp;
use genetic_planner::genetic_planner::{State, Action, Plan, PlannerConfiguration};


extern crate rand;
use rand::Rng;

use std::fmt;

const CAN_A_CAPACITY: usize = 5;
const CAN_B_CAPACITY: usize = 3;

#[derive(Clone)]
struct Cans {
    can_a: usize,
    can_b: usize,
}


impl fmt::Display for Cans {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A:{},B:{}", self.can_a, self.can_b)
    }
}

fn fill_a(c: Cans) -> Option<Cans> {
    if c.can_a == CAN_A_CAPACITY {
        None
    } else {
        Some(Cans {
            can_a: CAN_A_CAPACITY,
            can_b: c.can_b,
        })
    }
}
fn fill_b(c: Cans) -> Option<Cans> {
    if c.can_b == CAN_B_CAPACITY {
        None
    } else {
        Some(Cans {
            can_b: CAN_B_CAPACITY,
            can_a: c.can_a,
        })
    }
}

fn empty_a(c: Cans) -> Option<Cans> {
    if c.can_a == 0 {
        None
    } else {
        Some(Cans {
            can_a: 0,
            can_b: c.can_b,
        })
    }

}
fn empty_b(c: Cans) -> Option<Cans> {
    if c.can_b == 0 {
        None
    } else {
        Some(Cans {
            can_b: 0,
            can_a: c.can_a,
        })
    }
}

fn fill_a_with_b(c: Cans) -> Option<Cans> {
    if c.can_b == 0 {
        None
    } else {
        let (a, b) = if c.can_a + c.can_b > CAN_A_CAPACITY {
            (CAN_A_CAPACITY, c.can_a + c.can_b - CAN_A_CAPACITY)
        } else {
            (c.can_a + c.can_b, 0)
        };
        Some(Cans {
            can_a: a,
            can_b: b,
        })
    }
}

fn fill_b_with_a(c: Cans) -> Option<Cans> {
    if c.can_a == 0 {
        None
    } else {
        let (b, a) = if c.can_a + c.can_b > CAN_B_CAPACITY {
            (CAN_B_CAPACITY, c.can_a + c.can_b - CAN_B_CAPACITY)
        } else {
            (c.can_a + c.can_b, 0)
        };
        Some(Cans {
            can_a: a,
            can_b: b,
        })
    }
}


impl gp::State for Cans {
    fn is_goal(&self) -> bool {
        self.can_a == 4
    }

    fn get_heuristic(&self) -> i32 {
        (if self.can_a > 4 {
            self.can_a - 4
        } else {
            4 - self.can_a
        }) as i32
    }

    fn get_initial_state() -> Cans {
        Cans {
            can_a: 0,
            can_b: 0,
        }
    }

    fn get_random_action() -> gp::Action<Cans> {
        let r: f32 = rand::thread_rng().gen_range(0f32, 1f32);
        if r < (1f32 / 6f32) {
            Action {
                action: fill_a,
                name: "Fill A".to_string(),
            }
        } else if r < (2f32 / 6f32) {
            Action {
                action: fill_b,
                name: "Fill B".to_string(),
            }
        } else if r < (3f32 / 6f32) {
            Action {
                action: empty_a,
                name: "Empty A".to_string(),
            }
        } else if r < (4f32 / 6f32) {
            Action {
                action: empty_b,
                name: "Empty B".to_string(),
            }
        } else if r < (5f32 / 6f32) {
            Action {
                action: fill_a_with_b,
                name: "Fill A with B".to_string(),
            }
        } else {
            Action {
                action: fill_b_with_a,
                name: "Fill B with A".to_string(),
            }
        }
    }
}

/// The Dia Hard Riddle:
/// You are given a 5 gallon jug and a 3 gallon jug in front of a fountain.
/// Measure out exactly 4 gallons of water.

fn main() {
    let pc = PlannerConfiguration {
        max_actions: 20,
        population_size: 100,
        tournmant_size: 40,
        elitism_size: 3,
        uniform_rate: 0.5,
        mutation_rate: 0.7,
        threadpool_size: 32,
    };
    let mut state: Cans = Cans::get_initial_state();
    let n: Plan<Cans> = gp::find_best_after_iterations(pc, 500);
    let mut j = 0;
    for i in n.actions {
        println!("({}):{}", j, i.name);
        j += 1;
        let op_state = (i.action)(state);
        println!("{}", op_state.clone().unwrap());
        state = op_state.unwrap();
    }
}
