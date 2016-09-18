#[cfg(test)]

extern crate rand;
use genetic::*;

#[test]
#[allow(dead_code)]
#[allow(unused_variables)]
fn create_individual() {
    let i1: Individual<u8> = Individual::new(8);
}

#[test]
fn crossover_test() {
    let i1: Individual<u8> = Individual::new(8);
    let i2: Individual<u8> = Individual::new(8);
    let ic1 = i1.clone().crossover(i2.clone(), 1f32);
    let mut equals = true;
    for i in 0..i1.genes.len() {
        let c1 = i1.genes.get(i).unwrap();
        let c2 = ic1.genes.get(i).unwrap();
        if c1 != c2 {
            equals = false;
        }
    }
    assert!(equals);
    let ic2 = i1.clone().crossover(i2.clone(), 0f32);
    equals = true;
    for i in 0..i1.genes.len() {
        let c1 = i2.genes.get(i).unwrap();
        let c2 = ic2.genes.get(i).unwrap();
        if c1 != c2 {
            equals = false;
        }
    }
    assert!(equals);
}

#[allow(dead_code)]
fn simple_fitness(i: Individual<u8>) -> i32 {
    let mut acc = 0i32;
    for g in i.genes {
        if g > 127 {
            acc += 1
        }
    }
    acc
}

#[test]
fn fitness() {
    let vec: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 128, 129];
    let i = Individual::new_with_vec(vec);
    let f = simple_fitness(i);
    assert_eq!(f, 2);
    let vec2: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 0];
    let i2 = Individual::new_with_vec(vec2);
    let f2 = simple_fitness(i2);
    assert_eq!(f2, 0);
}

#[allow(dead_code)]
fn default_population_configuration() -> PopulationConfiguration<u8> {
    PopulationConfiguration {
        population_size: 64,
        fitness: simple_fitness,
        genenumber: 8,
        mutation_rate: 0.5f32,
        uniform_rate: 0.5f32,
        tournmant_size: 16,
        elitism_size: 2,
        threadpool_size: 8,
    }
}

#[test]
#[allow(unused_variables)]
fn create_population() {
    let p = Population::<u8>::new(default_population_configuration());
}

#[test]
fn get_fittest() {
    let p = Population::<u8>::new(default_population_configuration());
    let fittest = p.get_fittest().unwrap().clone();
    let mut fittest2 = p.individuals_and_scores.first().unwrap().clone();
    for ind in p.individuals_and_scores {
        let f = (p.configuration.fitness)(ind.clone().0);
        if f > fittest2.1 {
            fittest2 = (ind.0, f);
        }
    }
    assert_eq!(fittest.1, fittest2.1);
}

#[test]
fn evolve() {
    let p = Population::<u8>::new(default_population_configuration());
    let pe = p.evolve();
    assert_eq!(p.individuals_and_scores.len(),
               pe.individuals_and_scores.len());
    assert!(pe.get_fittest().unwrap().1 >= p.get_fittest().unwrap().1);
}

#[test]
fn complete_evolve() {
    let mut p = Population::<u8>::new(default_population_configuration());
    while p.get_fittest().unwrap().1 < 4 {
        p = p.evolve();
    }
}
