extern crate rand;
use rand::{Rand, Rng};

extern crate threadpool;
use threadpool::ThreadPool;

use std::sync::mpsc::channel;
use std::cmp::PartialEq;


/// Rappresent a candidate solution for the problem
#[derive(Debug,Clone,PartialEq)]
pub struct Individual<T: 'static> {
    pub genes: Vec<T>,
}

impl<T> Individual<T>
    where T: Clone + Rand + Send + Sync + PartialEq + 'static
{
    /// Create a new individual which contains a vector of genenumber of random initiliazed T
    pub fn new(genenumber: usize) -> Individual<T> {
        let mut vec: Vec<T> = Vec::new();
        for _ in 0..genenumber {
            vec.push(rand::random::<T>());
        }
        Individual { genes: vec.clone() }
    }

    /// Create a new individual from a vector of T
    pub fn new_with_vec(v: Vec<T>) -> Individual<T> {
        Individual { genes: v.clone() }
    }

    /// Return an Individual<T> which is the result of the crossover operation
    /// between self and the second Individual<T>, accordingly the uniform_rate parameter
    pub fn crossover(&self, i2: Individual<T>, uniform_rate: f32) -> Individual<T> {
        let i1 = self.clone();
        let mut v: Vec<T> = Vec::new();
        let len = if i1.genes.len() < i2.genes.len() {
            i1.genes.len()
        } else {
            i2.genes.len()
        };
        let mut rng = rand::thread_rng();
        for i in 0..len {
            if rng.gen_range(0f32, 1f32) < uniform_rate {
                v.push(i1.genes.get(i).unwrap().clone());
            } else {
                v.push(i2.genes.get(i).unwrap().clone());
            }
        }
        Individual::new_with_vec(v)
    }

    /// Return an Individual<T> which is the result of the mutate operation, accordingly the mutation_rate parameter
    pub fn mutate(&self, mutation_rate: f32) -> Individual<T> {
        let i = self.clone();
        let mut v: Vec<T> = Vec::new();
        let mut rng = rand::thread_rng();
        for x in i.genes.iter() {
            if rng.gen_range(0f32, 1f32) < mutation_rate {
                v.push(rand::random::<T>());
            } else {
                v.push(x.clone());
            }
        }
        Individual::new_with_vec(v)
    }
}
/// A set of Individuals
#[derive(Clone)]
pub struct Population<T: 'static> {
    /// Contains set of Individual and the relative score
    pub individuals_and_scores: Vec<(Individual<T>, i32)>,
    /// Contains the configurations used to create the Population
    pub configuration: PopulationConfiguration<T>,
    /// Rappresent the generation of the Population
    pub generation: usize,
}

/// Rappresent the configuration associated to a Population
#[derive(Clone)]
pub struct PopulationConfiguration<T: 'static> {
    /// Fitness function used to calculate the score of an Individual
    pub fitness: fn(Individual<T>) -> i32,
    /// Size of the Population
    pub population_size: usize,
    /// Number of genes of each Individual
    pub genenumber: usize,
    /// Parameter used by the crossover function
    pub uniform_rate: f32,
    /// Parameter used by the mutate function
    pub mutation_rate: f32,
    /// Size of the set used to select the parent of the offsprings
    pub tournmant_size: usize,
    /// Number of Individual to copy in the next generation
    pub elitism_size: usize,
    /// Number of thread used during the evolve function
    pub threadpool_size: usize,
}

impl<T> Population<T>
    where T: Clone + Rand + Send + Sync + PartialEq + 'static
{
    /// Create a new Population from a vector of individuals,
    /// a configuration and the number of the generation
    pub fn new_with_vec(vec: Vec<(Individual<T>, i32)>,
                        configuration: PopulationConfiguration<T>,
                        generation: usize)
                        -> Population<T> {
        Population {
            individuals_and_scores: vec,
            configuration: configuration,
            generation: generation,
        }
    }

    /// Create a new random generation accordingly the configuration
    pub fn new(configuration: PopulationConfiguration<T>) -> Population<T> {
        let mut v = Vec::<(Individual<T>, i32)>::new();
        for _ in 0..configuration.population_size {
            let i = Individual::<T>::new(configuration.genenumber);
            let score = (configuration.fitness)(i.clone());
            v.push((i, score));
        }
        Population::new_with_vec(v, configuration, 0)
    }

    /// Get the Individual and the relative score of the Individual 
    /// with the highest score
    pub fn get_fittest(&self) -> Option<(Individual<T>, i32)> {
        let individuals = self.individuals_and_scores.clone();
        let opt = individuals.iter().max_by_key(|a| a.1);
        if opt.is_some() {
            Some(opt.unwrap().clone())
        } else {
            None
        }
    }

    /// Get the Individuals with the highest score 
    fn get_top(&self, number: usize) -> Vec<(Individual<T>, i32)> {
        let mut v: Vec<(Individual<T>, i32)> = Vec::new();
        let iter = self.individuals_and_scores.iter().clone();
        if self.individuals_and_scores.len() <= number {
            self.individuals_and_scores.clone();
        } else {
            let first_max = iter.max_by_key(|a| a.1);
            v.push(first_max.unwrap().clone());
            for _ in 1..number {
                let mut max: Option<(Individual<T>, i32)> = None;
                for is in self.individuals_and_scores.clone() {
                    if max.clone().is_none() {
                        max = Some(is);
                    } else if max.clone().unwrap().1 < is.1 && v.iter().all(|a| a.0 != is.0) {
                        max = Some(is);
                    }
                }
                if max.is_some() {
                    v.push(max.unwrap());
                }
            }
        }
        v
    }

    /// Get the Individual with the highest score from a random selection
    /// of the individuals of the population
    fn tournment(&self) -> Individual<T> {
        let mut v: Vec<(Individual<T>, i32)> = Vec::new();
        for _ in 0..self.configuration.tournmant_size {
            let n = rand::thread_rng().gen_range(0, self.individuals_and_scores.len());
            v.push(self.individuals_and_scores.get(n).unwrap().clone());
        }
        v.iter().max_by_key(|a| a.1).unwrap().0.clone()
    }

    /// Create a new Population from the current, using the crossover 
    /// and mutation  operator
    pub fn evolve(&self) -> Population<T> {
        let mut v: Vec<(Individual<T>, i32)> = Vec::new();
        let new_elitism_size = if self.configuration.elitism_size >
                                  self.configuration.population_size {
            self.configuration.population_size
        } else {
            self.configuration.elitism_size
        };
        for elite in self.get_top(new_elitism_size) {
            v.push(elite);
        }
        let (tx, rx) = channel();
        let pool = ThreadPool::new(if self.configuration.threadpool_size > 0 {
            self.configuration.threadpool_size
        } else {
            1
        });
        for _ in new_elitism_size..self.configuration.population_size {
            let tx = tx.clone();
            let pop = (*self).clone();
            pool.execute(move || {
                let i1 = pop.tournment();
                let i2 = pop.tournment();
                let ic = i1.crossover(i2, pop.configuration.uniform_rate);
                let im = ic.mutate(pop.configuration.mutation_rate);
                let f = (pop.configuration.fitness)(im.clone());
                tx.send((im, f)).unwrap();
            });
        }
        for _ in new_elitism_size..self.configuration.population_size {
            v.push(rx.recv().unwrap());
        }
        Population::new_with_vec(v, self.configuration.clone(), self.generation + 1)
    }
}
