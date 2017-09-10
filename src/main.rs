extern crate annealing_redux;
extern crate config;

use config::{Config, File, FileFormat, Value};
use annealing_redux as ar;
use ar::db::make_cities;
use ar::annealing::Annealer;
use ar::solution::DistMatrix;
use std::sync::Arc;
use std::thread;

fn main() {
    let dists = Arc::new(make_cities().unwrap());
    let annealers = from_config(dists);
    let mut children = vec![];

    for (mut annealer, seed) in annealers {
        children.push(thread::spawn(move || {
            let solutions = annealer.threshold_accepting();
            println!("{}\t\tseed:{}", solutions[solutions.len() - 1], seed);
        }));
    }
    for child in children {
        child.join().expect("Error while joining child");
    }
}

fn begin_annealer(annealer: &mut Annealer, seed: u32) {
    let solutions = annealer.threshold_accepting();
    println!("{}\t\tseed:{}", solutions[solutions.len() - 1], seed)
}

fn from_config(dists: DistMatrix) -> Vec<(Annealer, u32)> {
    let mut c = Config::new();
    c.merge(File::new("Settings", FileFormat::Toml).required(true))
        .expect("No Configuration File 'Settings.toml'");

    let ids: Vec<u16> = to_u16_vec(c.get_array("city_ids").unwrap());
    let bs: u32 = c.get_int("batch_size").unwrap() as u32;
    let seeds: Vec<u32> = to_u32_vec(c.get_array("seeds").unwrap());
    let ap: f64 = c.get_float("accepted_percent").unwrap();
    let it: f64 = c.get_float("init_temp").unwrap();
    let mt: f64 = c.get_float("min_temp").unwrap();
    let ep: f64 = c.get_float("e_p").unwrap();
    let phi: f64 = c.get_float("phi").unwrap();
    let mut annealers = Vec::with_capacity(seeds.len());

    for seed in seeds {
        let an = Annealer::new(
            ids.clone(),
            bs,
            [seed, seed * 7, seed * 23, seed * 69],
            ap,
            it,
            mt,
            ep,
            phi,
            dists.clone(),
        );
        annealers.push((an, seed));
    }
    annealers
}

fn to_u32_vec(values: Vec<Value>) -> Vec<u32> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().expect("Error converting value to i64") as u32);
    }
    v
}

fn to_u16_vec(values: Vec<Value>) -> Vec<u16> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().expect("Error converting value to i64") as u16);
    }
    v
}
