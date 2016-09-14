extern crate rand;
use rand::Rand;
use rand::Rng;
use genetic::*;

pub trait State
    where Self: Sized + Clone + Send + Sync + 'static
{
    fn get_initial_state() -> Self;
    fn get_random_action() -> Action<Self>;
    fn is_goal(&self) -> bool;
    fn get_heuristic(&self) -> i32;
}

pub struct Node<T>
    where T: State + Clone + Send + Sync + 'static
{
    pub state: T,
    pub actions: Vec<Action<T>>,
}

impl<T> Node<T>
    where T: State + Clone
{
    pub fn new(state: T) -> Node<T> {
        Node {
            state: state,
            actions: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Action<T>
    where T: State + Clone + Send + Sync + 'static
{
    pub action: fn(state: T) -> Option<T>,
    pub name: String,
}

impl<T> Rand for Action<T>
    where T: State + Clone
{
    fn rand<R: Rng>(_: &mut R) -> Action<T> {
        T::get_random_action()
    }
}

pub struct PlannerConfiguration {
    pub max_moves: usize,
    pub population_size: usize,
    pub elitism_size: usize,
    pub tournmant_size: usize,
    pub uniform_rate: f32,
    pub mutation_rate: f32,
}

fn apply_actions<T>(i: Individual<Action<T>>) -> Node<T>
    where T: State + Clone + Send + Sync + 'static
{
    let mut state = Some(T::get_initial_state());
    let mut old_state = state.clone();
    let mut used_actions: Vec<Action<T>> = Vec::new();
    let mut actions = i.genes.iter();
    let mut action = actions.next();
    while action.is_some() && state.clone().is_some() && !state.clone().unwrap().is_goal() {
        state = (action.clone().unwrap().action)(state.unwrap());
        if state.clone().is_some() {
            old_state = state.clone();
            let tmp = action.clone().unwrap();
            let last_action = Action {
                action: tmp.action,
                name: tmp.name.to_string(),
            };
            used_actions.push(last_action);
            action = actions.next();
        }
    }
    match state {
        None => {
            Node {
                state: old_state.unwrap(),
                actions: used_actions,
            }
        }
        Some(sstate) => {
            Node {
                state: sstate,
                actions: used_actions,
            }
        }
    }
}

fn fitness_planner<T>(i: Individual<Action<T>>) -> i32
    where T: State + Clone + Send + Sync + 'static
{
    let node = apply_actions(i);
    -node.state.get_heuristic()
}

pub fn find_solution<T>(c: PlannerConfiguration) -> Node<T>
    where T: State + Clone + Send + Sync + 'static
{
    let pc = PopulationConfiguration {
        genelenght: c.max_moves,
        population_size: c.population_size,
        elitism_size: c.elitism_size,
        tournmant_size: c.tournmant_size,
        uniform_rate: c.uniform_rate,
        mutation_rate: c.mutation_rate,
        fitness: fitness_planner,
    };
    let mut pop = Population::new(pc);
    let mut best_actions = pop.get_fittest();
    let mut node: Node<T> = apply_actions(best_actions.unwrap().0);
    while !node.state.is_goal() {
        pop = pop.evolve();
        best_actions = pop.get_fittest();
        node = apply_actions(best_actions.unwrap().0);
    }
    Node {
        state: node.state,
        actions: node.actions,
    }
}

pub fn find_best_fit<T>(c: PlannerConfiguration, iterations: usize) -> Node<T>
    where T: State + Clone + Send + Sync + 'static
{
    let pc = PopulationConfiguration {
        genelenght: c.max_moves,
        population_size: c.population_size,
        elitism_size: c.elitism_size,
        tournmant_size: c.tournmant_size,
        uniform_rate: c.uniform_rate,
        mutation_rate: c.mutation_rate,
        fitness: fitness_planner,
    };
    let mut pop = Population::new(pc);
    for _ in 0..iterations {
        pop = pop.evolve();
    }
    let best_actions = pop.get_fittest();
    let node = apply_actions(best_actions.unwrap().0);
    Node {
        state: node.state,
        actions: node.actions,
    }
}
