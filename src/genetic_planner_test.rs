extern crate rand;

use rand::Rng;
use genetic_planner::*;

#[derive(Clone)]
struct Coin {
    pub head: bool,
}

impl State for Coin {
    fn get_initial_state() -> Coin {
        Coin { head: false }
    }

    fn get_random_action() -> Action<Coin> {
        let a = rand::thread_rng().gen::<u8>();
        let half = u8::max_value() / 2;
        if a > half {
            Action::<Coin> {
                action: flip,
                name: "Flip".to_string(),
            }
        } else {
            Action::<Coin> {
                action: flop,
                name: "Flop".to_string(),
            }
        }
    }

    fn is_goal(&self) -> bool {
        self.head
    }

    fn get_heuristic(&self) -> i32 {
        if self.head {
            0
        } else {
            1
        }
    }
}

fn flip(c: Coin) -> Option<Coin> {
    let not_head = !c.head;
    Some(Coin { head: not_head })
}

fn flop(_: Coin) -> Option<Coin> {
    None
}

#[test]
fn test() {
    let pc = PlannerConfiguration {
        max_moves: 4,
        population_size: 16,
        tournmant_size: 4,
        elitism_size: 1,
        uniform_rate: 0.5,
        mutation_rate: 0.5,
    };
    let n: Node<Coin> = find_solution(pc);
    assert!(n.state.is_goal());
    assert!(n.actions.len() == 1);
    assert!(n.actions.get(0).unwrap().name == "Flip");
}
