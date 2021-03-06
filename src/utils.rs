use std::collections::HashMap;
use std::fs;
use std::iter::Iterator;

use crate::Country;

fn read_lines(path: &str) -> Vec<String> {
    let file_str = fs::read_to_string(path).unwrap();
    let lines = file_str
        .split('\n')
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());
    lines.collect()
}

pub fn read_closest_data() -> HashMap<u16, Vec<u16>> {
    let lines = read_lines("data/closest.csv");
    lines
        .iter()
        .map(|line| {
            let spl: Vec<&str> = line.split(";").collect();
            let id: u16 = spl.get(0).unwrap().parse().unwrap();
            let ls: Vec<u16> = spl
                .get(1)
                .unwrap()
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();
            (id, ls)
        })
        .collect()
}

pub fn read_country_data() -> HashMap<u16, Country> {
    let lines = read_lines("data/country_data.csv");
    lines
        .iter()
        .map(|line| {
            let spl: Vec<&str> = line.split(";").collect();
            let id: u16 = spl.get(0).unwrap().parse().unwrap();
            let name: String = spl.get(1).unwrap().to_string();
            (id, Country { name })
        })
        .collect()
}
