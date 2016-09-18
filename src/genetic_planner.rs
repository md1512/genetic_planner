extern crate rand;
use rand::Rand;
use rand::Rng;

use std::cmp::PartialEq;

use genetic::*;

pub trait State
    where Self: Sized + Clone + Send + Sync + 'static
{
    /// Get the initial state
    fn get_initial_state() -> Self;
    /// Get a random action 
    fn get_random_action() -> Action<Self>;
    /// Verify if the current state is the goal
    fn is_goal(&self) -> bool;
    /// Get an aproximated distance to the goal state
    fn get_heuristic(&self) -> i32;
}

/// Contains the actions of a Plan 
pub struct Plan<T>
    where T: State + Clone + Send + Sync + 'static
{
    /// State reached using the actions, the first state is
    /// State::get_initial_state()
    pub state: T,
    /// Actions of the Plan
    pub actions: Vec<Action<T>>,
}

impl<T> Plan<T>
    where T: State + Clone
{
    /// Create a new Action
    pub fn new(state: T) -> Plan<T> {
        Plan {
            state: state,
            actions: Vec::new(),
        }
    }
}

/// Contains a function applicable to T and a name
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

/// Contains the configuration of the Planner 
pub struct PlannerConfiguration {
    /// Max number of actions
    pub max_actions: usize,
    /// Number of the Individual in the Population
    pub population_size: usize,
    /// Number of Individual to copy in the next generation
    pub elitism_size: usize,
    /// Size of the set used to select the parentof the offsprings
    pub tournmant_size: usize,
    /// Parameter used by the crossover function
    pub uniform_rate: f32,
    /// Parameter used by the mutate function
    pub mutation_rate: f32,
    /// Number of thread used in the evolve function
    pub threadpool_size: usize,
}

/// Apply the Action of the Individual to the initial state
fn apply_actions<T>(i: Individual<Action<T>>) -> Plan<T>
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
            Plan {
                state: old_state.unwrap(),
                actions: used_actions,
            }
        }
        Some(sstate) => {
            Plan {
                state: sstate,
                actions: used_actions,
            }
        }
    }
}

/// Calculate the fitness of an Individual<Action<T>>
fn fitness_planner<T>(i: Individual<Action<T>>) -> i32
    where T: State + Clone + Send + Sync + 'static
{
    let node = apply_actions(i);
    -node.state.get_heuristic()
}

/// Convert PlannerConfiguration to PopulationConfiguration
fn get_population_configuration<T>(c: PlannerConfiguration) -> PopulationConfiguration<Action<T>>
    where T: State + Clone + Send + Sync + 'static
{
    PopulationConfiguration {
        genenumber: c.max_actions,
        population_size: c.population_size,
        elitism_size: c.elitism_size,
        tournmant_size: c.tournmant_size,
        uniform_rate: c.uniform_rate,
        mutation_rate: c.mutation_rate,
        fitness: fitness_planner,
        threadpool_size: c.threadpool_size,
    }
}

/// Find a Plan and its Population starting a Population
pub fn find_solution_and_population_from_population<T>(pop: Population<Action<T>>)
                                                       -> (Plan<T>, Population<Action<T>>)
    where T: State + Clone + Send + Sync + 'static
{
    let mut pop = pop.clone();
    let mut best_actions = pop.get_fittest();
    let mut node: Plan<T> = apply_actions(best_actions.unwrap().0);
    while !node.state.is_goal() {
        pop = pop.evolve();
        best_actions = pop.get_fittest();
        node = apply_actions(best_actions.unwrap().0);
    }
    (Plan {
        state: node.state,
        actions: node.actions,
    },
     pop)
}

/// Find a Plan and its Population
pub fn find_solution_and_population<T>(c: PlannerConfiguration) -> (Plan<T>, Population<Action<T>>)
    where T: State + Clone + Send + Sync + 'static
{
    let pc = get_population_configuration(c);
    let pop = Population::new(pc);
    find_solution_and_population_from_population(pop)
}

/// Find a plan 
pub fn find_solution<T>(c: PlannerConfiguration) -> Plan<T>
    where T: State + Clone + Send + Sync + 'static
{
    find_solution_and_population(c).0
}

/// Find a plan starting from a Population
pub fn find_solution_from_population<T>(pop: Population<Action<T>>) -> Plan<T>
    where T: State + Clone + Send + Sync + 'static
{
    find_solution_and_population_from_population(pop).0
}

/// Found the best plan and its Population, after <iterations> iterations
pub fn find_best_and_population_after_iterations_from_population<T>
    (pop: Population<Action<T>>,
     iterations: usize)
     -> (Plan<T>, Population<Action<T>>)
    where T: State + Clone + Send + Sync + 'static
{
    let mut pop = pop.clone();
    for _ in 0..iterations {
        pop = pop.evolve();
    }
    let best_actions = pop.get_fittest();
    let node = apply_actions(best_actions.unwrap().0);
    (Plan {
        state: node.state,
        actions: node.actions,
    },
     pop)
}

/// Found the best plan and its Population after <iterations> iterations
pub fn find_best_and_population_after_iterations<T>(c: PlannerConfiguration,
                                                    iterations: usize)
                                                    -> (Plan<T>, Population<Action<T>>)
    where T: State + Clone + Send + Sync + 'static
{
    let pc = get_population_configuration(c);
    let pop = Population::new(pc);
    find_best_and_population_after_iterations_from_population(pop, iterations)
}

/// Found the best plan after <iterations> iterations
pub fn find_best_after_iterations<T>(c: PlannerConfiguration, iterations: usize) -> Plan<T>
    where T: State + Clone + Send + Sync + 'static
{
    find_best_and_population_after_iterations(c, iterations).0
}

/// Found the best plan after <iterations> iterations starting from a Population
pub fn find_best_after_iterations_from_population<T>(pop: Population<Action<T>>,
                                                     iterations: usize)
                                                     -> Plan<T>
    where T: State + Clone + Send + Sync + 'static
{
    find_best_and_population_after_iterations_from_population(pop, iterations).0
}
