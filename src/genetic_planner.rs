extern crate rand;
use rand::Rand;
use rand::Rng;

use std::cmp::PartialEq;

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

impl<T> PartialEq for Action<T>
    where T: State + Clone + Send + Sync + 'static
{
    fn eq(&self, other: &Action<T>) -> bool {
        self.name != other.name
    }
}

pub struct PlannerConfiguration {
    pub max_moves: usize,
    pub population_size: usize,
    pub elitism_size: usize,
    pub tournmant_size: usize,
    pub uniform_rate: f32,
    pub mutation_rate: f32,
    pub threadpool_size: usize,
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

fn get_population_configuration<T>(c: PlannerConfiguration) -> PopulationConfiguration<Action<T>>
    where T: State + Clone + Send + Sync + 'static
{
    PopulationConfiguration {
        genelenght: c.max_moves,
        population_size: c.population_size,
        elitism_size: c.elitism_size,
        tournmant_size: c.tournmant_size,
        uniform_rate: c.uniform_rate,
        mutation_rate: c.mutation_rate,
        fitness: fitness_planner,
        threadpool_size: c.threadpool_size,
    }
}
pub fn find_solution_and_population_from_population<T>(pop: Population<Action<T>>)
                                                       -> (Node<T>, Population<Action<T>>)
    where T: State + Clone + Send + Sync + 'static
{
    let mut pop = pop.clone();
    let mut best_actions = pop.get_fittest();
    let mut node: Node<T> = apply_actions(best_actions.unwrap().0);
    while !node.state.is_goal() {
        pop = pop.evolve();
        best_actions = pop.get_fittest();
        node = apply_actions(best_actions.unwrap().0);
    }
    (Node {
        state: node.state,
        actions: node.actions,
    },
     pop)
}

pub fn find_solution_and_population<T>(c: PlannerConfiguration) -> (Node<T>, Population<Action<T>>)
    where T: State + Clone + Send + Sync + 'static
{
    let pc = get_population_configuration(c);
    let pop = Population::new(pc);
    find_solution_and_population_from_population(pop)
}

pub fn find_solution<T>(c: PlannerConfiguration) -> Node<T>
    where T: State + Clone + Send + Sync + 'static
{
    find_solution_and_population(c).0
}

pub fn find_solution_from_population<T>(pop: Population<Action<T>>) -> Node<T>
    where T: State + Clone + Send + Sync + 'static
{
    find_solution_and_population_from_population(pop).0
}

pub fn find_best_and_population_after_iterations_from_population<T>
    (pop: Population<Action<T>>,
     iterations: usize)
     -> (Node<T>, Population<Action<T>>)
    where T: State + Clone + Send + Sync + 'static
{
    let mut pop = pop.clone();
    for _ in 0..iterations {
        pop = pop.evolve();
    }
    let best_actions = pop.get_fittest();
    let node = apply_actions(best_actions.unwrap().0);
    (Node {
        state: node.state,
        actions: node.actions,
    },
     pop)
}

pub fn find_best_and_population_after_iterations<T>(c: PlannerConfiguration,
                                                    iterations: usize)
                                                    -> (Node<T>, Population<Action<T>>)
    where T: State + Clone + Send + Sync + 'static
{
    let pc = get_population_configuration(c);
    let pop = Population::new(pc);
    find_best_and_population_after_iterations_from_population(pop, iterations)
}

pub fn find_best_after_iterations<T>(c: PlannerConfiguration, iterations: usize) -> Node<T>
    where T: State + Clone + Send + Sync + 'static
{
    find_best_and_population_after_iterations(c, iterations).0
}

pub fn find_best_after_iterations_from_population<T>(pop: Population<Action<T>>,
                                                     iterations: usize)
                                                     -> Node<T>
    where T: State + Clone + Send + Sync + 'static
{
    find_best_and_population_after_iterations_from_population(pop, iterations).0
}
