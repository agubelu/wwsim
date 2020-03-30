use std::collections::{HashMap, HashSet};
use std::env;
use rand::Rng;

mod utils;
use utils::{read_country_data, read_closest_data, read_log};

mod game_utils;
use game_utils::{compute_neighbors, find_conquered_id, find_conqueror_id};

///////////////////////////////////////////////////////////////////////////////

pub enum LogType { Conquer, Indep }

pub struct LogEntry {
    kind: LogType,
    country1: u16,
    country2: u16,
}

pub struct Country {
    id: u16,
    name: String
}

///////////////////////////////////////////////////////////////////////////////

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_runs: usize = args.get(1).expect("Provide the number of runs").parse().expect("Not a valid number");

    let country_data = read_country_data();
    let closest_data = read_closest_data();
    let log_data = read_log();

    let mut rng = rand::thread_rng();

    // Load the log data
    let mut remaining_after_log: HashSet<u16> = country_data.keys().cloned().collect();
    let mut owners_data_after_log: HashMap<u16, u16> = country_data.keys().map(|k| (*k, *k)).collect();
    let mut owns_data_after_log: HashMap<u16, u16> = country_data.keys().map(|k| (*k, 1)).collect();

    let rem_ref_log = &mut remaining_after_log;
    let owners_ref_log = &mut owners_data_after_log;
    let owns_ref_log = &mut owns_data_after_log;

    for log_elem in &log_data {
        let (id1, id2) = (log_elem.country1, log_elem.country2);
        match log_elem.kind {
            LogType::Conquer => conquer(id1, id2, owners_ref_log, owns_ref_log, rem_ref_log),
            LogType::Indep => independence(id1, owners_ref_log, owns_ref_log, rem_ref_log)
        }
    }

    // Simulate the runs starting from the last log point
    for _ in 0..n_runs {
        let mut epoch = log_data.len();

        let mut remaining = remaining_after_log.clone();
        let mut owners_data = owners_data_after_log.clone();
        let mut owns_data = owns_data_after_log.clone();
        
        let owners_ref = &mut owners_data;
        let owns_ref = &mut owns_data;
        let remaining_ref = &mut remaining;

        while remaining_ref.len() > 1 {
            epoch += 1;

            let independence_chance = 1.0 / (12.0 + (epoch as f64 / 10.0));
            let neighbors = compute_neighbors(owners_ref, &closest_data);
            let conqueror_id = find_conqueror_id(owners_ref, &neighbors);

            if rng.gen::<f64>() < independence_chance {
                independence(conqueror_id, owners_ref, owns_ref, remaining_ref);
            } else {
                let conquered_id = find_conquered_id(conqueror_id, owners_ref, &neighbors);
                conquer(conqueror_id, conquered_id, owners_ref, owns_ref, remaining_ref);
            }
        }

        println!("{}", country_data[remaining.iter().next().unwrap()].name);
    }
}

///////////////////////////////////////////////////////////////////////////////

fn independence<'a, 'b> (
    indep_terr_id: u16,
    owners_data: &'b mut HashMap<u16, u16>,
    owns_data: &'b mut HashMap<u16, u16>,
    remaining: &'b mut HashSet<u16>
) {
    let old_owner_id = owners_data[&indep_terr_id];

    owners_data.insert(indep_terr_id, indep_terr_id);
    *owns_data.entry(indep_terr_id).or_insert(0) += 1;
    *owns_data.entry(old_owner_id).or_insert(0) -= 1;

    if owns_data[&indep_terr_id] == 1 {
        remaining.insert(indep_terr_id);
    }

    if owns_data[&old_owner_id] == 0 {
        remaining.remove(&old_owner_id);
    }
}

fn conquer<'a, 'b> (
    conqueror_terr_id: u16,
    conquered_terr_id: u16,
    owners_data: &'b mut HashMap<u16, u16>,
    owns_data: &'b mut HashMap<u16, u16>,
    remaining: &'b mut HashSet<u16>
) {
    let original_conqueror_id = owners_data[&conqueror_terr_id];
    let original_conquered_id = owners_data[&conquered_terr_id];

    *owns_data.entry(original_conqueror_id).or_insert(0) += 1;
    *owns_data.entry(original_conquered_id).or_insert(0) -= 1;

    owners_data.insert(conquered_terr_id, owners_data[&conqueror_terr_id]);

    if owns_data[&original_conquered_id] == 0 {
        remaining.remove(&original_conquered_id);
    }
}