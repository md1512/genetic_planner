extern crate rand;
use rand::{Rand, Rng};

#[derive(Debug,Clone,PartialEq)]
pub struct Individual<T> {
    pub genes: Vec<T>,
}

impl<T> Individual<T>
    where T: Clone + Rand
{
    pub fn new(genelenght: usize) -> Individual<T> {
        let mut vec: Vec<T> = Vec::new();
        for _ in 0..genelenght {
            vec.push(rand::random::<T>());
        }
        Individual { genes: vec.clone() }
    }
    pub fn new_with_vec(v: Vec<T>) -> Individual<T> {
        Individual { genes: v.clone() }
    }


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
pub struct Population<T> {
    pub individuals_and_scores: Vec<(Individual<T>, i32)>,
    pub configuration: PopulationConfiguration<T>,
}

#[derive(Clone)]
pub struct PopulationConfiguration<T> {
    pub fitness: fn(Individual<T>) -> i32,
    pub population_size: usize,
    pub genelenght: usize,
    pub uniform_rate: f32,
    pub mutation_rate: f32,
    pub tournmant_size: usize,
    pub elitism_size: usize,
}

impl<T> Population<T>
    where T: Clone + Rand
{
    pub fn new_with_vec(vec: Vec<(Individual<T>, i32)>,
                        configuration: PopulationConfiguration<T>)
                        -> Population<T> {
        Population {
            individuals_and_scores: vec,
            configuration: configuration,
        }
    }

    pub fn new(configuration: PopulationConfiguration<T>) -> Population<T> {
        let mut v = Vec::<(Individual<T>, i32)>::new();
        for _ in 0..configuration.population_size {
            let i = Individual::<T>::new(configuration.genelenght);
            let score = (configuration.fitness)(i.clone());
            v.push((i, score));
        }
        Population::new_with_vec(v, configuration)
    }

    pub fn get_fittest(&self) -> Option<(Individual<T>, i32)> {
        let individuals = self.individuals_and_scores.clone();
        let opt = individuals.iter().max_by_key(|a| a.1);
        if opt.is_some() {
            Some(opt.unwrap().clone())
        } else {
            None
        }
    }

    fn get_top(&self, size: usize) -> Vec<(Individual<T>, i32)> {
        let mut v: Vec<(Individual<T>, i32)> = Vec::new();
        let mut individuals = self.individuals_and_scores.clone();
        individuals.sort_by_key(|a| -a.1);
        for i in 0..size {
            v.push(individuals.get(i).unwrap().clone());
        }
        v
    }

    fn tournment(&self) -> Individual<T> {
        let mut v: Vec<(Individual<T>, i32)> = Vec::new();
        for _ in 0..self.configuration.tournmant_size {
            let n = rand::thread_rng().gen_range(0, self.individuals_and_scores.len());
            v.push(self.individuals_and_scores.get(n).unwrap().clone());
        }
        v.iter().max_by_key(|a| a.1).unwrap().0.clone()
    }

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
        for _ in new_elitism_size..self.configuration.population_size {
            let i1 = self.tournment();
            let i2 = self.tournment();
            let ic = i1.crossover(i2, self.configuration.uniform_rate);
            let im = ic.mutate(self.configuration.mutation_rate);
            let f = (self.configuration.fitness)(im.clone());
            v.push((im, f));
        }
        Population::new_with_vec(v, self.configuration.clone())
    }
}
