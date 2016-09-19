extern crate genetic_planner;
use genetic_planner::genetic_planner as gp;
use genetic_planner::genetic_planner::{State, Action, Plan, PlannerConfiguration};


extern crate rand;
use rand::Rng;

use std::fmt;

const MAZE_SIZE: usize = 10;

#[derive(Copy,Clone,PartialEq)]
enum Tile {
    Wall,
    Empty,
    Finish,
}
#[derive(Clone)]
struct Maze {
    maze: [[Tile; MAZE_SIZE]; MAZE_SIZE],
    bot_position: (usize, usize),
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut txt = "".to_string();
        let mut position = (0usize, 0);
        let maze = self.maze;
        for r in maze.iter() {
            for c in r.iter() {
                txt = txt +
                      if self.bot_position == position {
                    "<B>"
                } else {
                    match c {
                        &Tile::Wall => "[W]",
                        &Tile::Empty => "[ ]",
                        &Tile::Finish => "[F]",
                    }
                };

                position = (position.0, position.1 + 1);
            }
            txt = txt + "\n";
            position = (position.0 + 1, 0);
        }
        write!(f, "{}", txt)
    }
}

fn go_right(m: Maze) -> Option<Maze> {
    go(m, (0, 1))
}

fn go_left(m: Maze) -> Option<Maze> {
    go(m, (0, -1))
}

fn go_up(m: Maze) -> Option<Maze> {
    go(m, (-1, 0))
}

fn go_down(m: Maze) -> Option<Maze> {
    go(m, (1, 0))
}

fn go(m: Maze, d: (isize, isize)) -> Option<Maze> {
    if m.bot_position.1 == 0 && d.1 == -1 {
        None
    } else if m.bot_position.1 == MAZE_SIZE - 1 && d.1 == 1 {
        None
    } else if m.bot_position.0 == 0 && d.0 == -1 {
        None
    } else if m.bot_position.0 == MAZE_SIZE - 1 && d.0 == 1 {
        None
    } else {
        let new_pos = sum(&m.bot_position, &d);
        if m.maze[new_pos.0][new_pos.1] == Tile::Wall {
            None
        } else {
            let new_maze = Maze {
                maze: m.maze,
                bot_position: new_pos,
            };
            Some(new_maze)
        }
    }
}

fn sum(a: &(usize, usize), b: &(isize, isize)) -> (usize, usize) {
    let mut c = a.clone();
    match b.0 {
        1 => c = (c.0 + 1, c.1),
        -1 => c = (c.0 - 1, c.1),
        _ => {}
    }
    match b.1 {
        1 => c = (c.0, c.1 + 1),
        -1 => c = (c.0, c.1 - 1),
        _ => {}
    }
    c
}

impl gp::State for Maze {
    fn is_goal(&self) -> bool {
        let (y, x) = self.bot_position;
        self.maze[y][x] == Tile::Finish
    }

    fn get_heuristic(&self) -> i32 {
        let mut position = (0usize, 0);
        let mut finish = (0usize, 0);
        let maze = self.maze;
        for r in maze.iter() {
            for _ in r.iter() {
                if maze[position.0][position.1] == Tile::Finish {
                    finish = position;
                }
                position = (position.0, position.1 + 1);
            }
            position = (position.0 + 1, 0);
        }
        position = self.bot_position;
        let (bottom, top) = if position.0 > finish.0 {
            (position.0, finish.0)
        } else {
            (finish.0, position.0)
        };
        let (right, left) = if position.1 > finish.1 {
            (position.1, finish.1)
        } else {
            (finish.1, position.1)
        };
        (bottom - top + right - left) as i32
    }

    fn get_initial_state() -> Maze {
        let e = Tile::Empty;
        let f = Tile::Finish;
        let w = Tile::Wall;
        Maze {
            maze: [[e, e, e, w, e, e, w, w, e, e],
                   [e, e, e, w, e, e, e, e, e, e],
                   [e, e, w, w, e, e, e, e, e, e],
                   [e, e, e, e, e, e, w, e, e, f],
                   [e, e, w, w, e, e, w, e, e, e],
                   [e, e, w, w, e, e, w, e, e, e],
                   [e, e, w, e, e, e, w, w, e, e],
                   [e, e, e, e, e, e, w, e, e, e],
                   [e, e, e, e, e, e, w, w, e, e],
                   [e, e, e, e, e, e, w, w, e, e]],
            bot_position: (0, 0),
        }
    }

    fn get_random_action() -> gp::Action<Maze> {
        let r: u8 = rand::thread_rng().gen();
        if r < 64 {
            Action {
                action: go_up,
                name: "Up".to_string(),
            }
        } else if r < 128 {
            Action {
                action: go_down,
                name: "Down".to_string(),
            }
        } else if r < 128 + 64 {
            Action {
                action: go_right,
                name: "Right".to_string(),
            }
        } else {
            Action {
                action: go_left,
                name: "Left".to_string(),
            }
        }
    }
}

fn print(m: &Option<Maze>) {
    if m.is_some() {
        println!("{}", m.clone().unwrap());
    } else {
        println!("None");
    }
}
/// Find the path for a bot in a maze 
fn main() {
    println!("Running...");
    let pc = PlannerConfiguration {
        max_actions: 40,
        population_size: 200,
        tournmant_size: 20,
        elitism_size: 3,
        uniform_rate: 0.5,
        mutation_rate: 0.7,
        threadpool_size: 16,
    };
    let mut state: Maze = Maze::get_initial_state();
    let n: Plan<Maze> = gp::find_solution(pc);
    println!("[W] Wall [F] Finish <B> Bot\n");
    let mut j = 0;
    for i in n.actions {
        println!("({}):{}", j, i.name);
        j += 1;
        let op_state = (i.action)(state);
        print(&op_state);
        state = op_state.unwrap();
    }
}
