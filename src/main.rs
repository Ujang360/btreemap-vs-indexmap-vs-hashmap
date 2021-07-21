#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use indexmap::IndexMap;
use rand::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

const ELEMENTS: usize = 16 * 1024 * 1024;

fn generate_values(target: &mut [u128]) {
    let mut randomizer = thread_rng();
    randomizer.fill(&mut target[0..ELEMENTS]);
}

fn main() {
    println!("[INSERT-SORT-BENCH]");
    println!("=> Elements: {}", ELEMENTS);

    // declarations

    let mut some_random_collections = vec![0; ELEMENTS];
    let mut some_indexmap = IndexMap::with_capacity(ELEMENTS);
    let mut some_hashmap = HashMap::with_capacity(ELEMENTS);
    let mut some_btreemap = BTreeMap::new();

    // generates random values

    let instant = Instant::now();
    generate_values(&mut some_random_collections);
    println!("=> Generate Randoms: {:?}", instant.elapsed());

    // insert into btreemap

    let instant = Instant::now();
    some_random_collections.iter().for_each(|source| {
        some_btreemap.insert(*source, *source);
    });
    println!("=> BTreeMap: {:?}", instant.elapsed());

    // insert into indexmap

    let instant = Instant::now();
    some_random_collections.iter().for_each(|source| {
        some_indexmap.insert(*source, *source);
    });
    some_indexmap.par_sort_keys();
    println!("=> IndexMap: {:?}", instant.elapsed());

    // insert into hashmap

    let instant = Instant::now();
    some_random_collections.sort_unstable();
    println!("=> Sorting: {:?}", instant.elapsed());
    let instant = Instant::now();
    some_random_collections.iter().for_each(|source| {
        some_hashmap.insert(*source, *source);
    });
    println!("=> HashMap: {:?}", instant.elapsed());

    // Samples
    println!("Index 0 -> {}", some_indexmap.get_index(0).unwrap().1);
    println!("Index 1 -> {}", some_indexmap.get_index(1).unwrap().1);
    println!("Index 2 -> {}", some_indexmap.get_index(2).unwrap().1);
}
