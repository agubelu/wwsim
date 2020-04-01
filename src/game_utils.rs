use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;

pub fn compute_neighbors<'a, 'b> (
    owners_data: &'b mut HashMap<u16, u16>,
    closest_data: &'a HashMap<u16, Vec<u16>>
) -> HashMap<u16, HashSet<u16>>
{
    let mut neighbors = HashMap::new();

    for (country_id, owner_id) in owners_data.iter() {
        let closest_id = closest_data[country_id].iter()
            .filter(|neigh_id| owners_data[neigh_id] != *owner_id).next().unwrap();
        neighbors.entry(*country_id).or_insert(HashSet::new()).insert(*closest_id);
        neighbors.entry(*closest_id).or_insert(HashSet::new()).insert(*country_id);
    }
    
    neighbors
}

pub fn find_conqueror_id<'a, 'b>(
    owners_data: &'b mut HashMap<u16, u16>,
    neighbors_data: &HashMap<u16, HashSet<u16>>
) -> u16 {

    let candidates: Vec<&u16> = owners_data.keys()
        .filter(|country_id| neighbors_data[*country_id].iter()
            .any(|neigh_id| owners_data[*country_id] != owners_data[neigh_id])
        )
        .collect();
    **candidates.choose(&mut rand::thread_rng()).unwrap()
}

pub fn find_conquered_id<'a, 'b>(
    conqueror_id: u16,
    owners_data: &'b mut HashMap<u16, u16>,
    neighbors_data: &'b HashMap<u16, HashSet<u16>>
) -> u16 {

    let candidates: Vec<&u16> = neighbors_data[&conqueror_id].iter()
        .filter(|c_id| owners_data[&conqueror_id] != owners_data[*c_id])
        .collect();
    **candidates.choose(&mut rand::thread_rng()).unwrap()
}