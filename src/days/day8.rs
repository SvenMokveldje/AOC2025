use core::num;
use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Coord {
    place: usize,
    x: usize,
    y: usize,
    z: usize,
}

pub fn day() {
    let input = read_to_string("src/inputs/day8.txt").unwrap();
     
    let parsed_input = parse_input(input);
   
    let total_result = second(parsed_input);

    print!("{:?}\n", total_result);
}

fn parse_input(input: String) -> Vec<Coord> {
    let mut vectors: Vec<Coord> = Vec::new();
    for (i, line) in input.lines().into_iter().enumerate() {
        let number_strings: Vec<&str> = line.split(',').collect();
        let number: Vec<usize> = number_strings.iter().map(|s| s.parse::<usize>().unwrap()).collect();

        vectors.push(Coord { place: i, x: number[0], y: number[1], z: number[2] });
    }

    return vectors;
}

fn get_distances(coords: &Vec<Coord>) -> Vec<(usize, usize, f64)> {
    let mut distances = Vec::new();
    
    for (i, coord1) in coords.iter().enumerate() {
        for (j, coord2) in coords.iter().enumerate().skip(i + 1) {
            let dx = (coord1.x as f64 - coord2.x as f64).powi(2);
            let dy = (coord1.y as f64 - coord2.y as f64).powi(2);
            let dz = (coord1.z as f64 - coord2.z as f64).powi(2);
            let distance = (dx + dy + dz).sqrt();
            
            distances.push((coord1.place, coord2.place, distance));
        }
    }
    
    distances
}


fn first(input: Vec<Coord> ) -> usize {
    let mut iter = 1000;

    let coords = input.clone();

    let mut junction_groups: Vec<HashSet<usize>> = coords.iter()
        .map(|coord| {
            let mut set = HashSet::new();
            set.insert(coord.place);
            set
        })
        .collect();
    
    let mut distances = get_distances(&coords);
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());


    let shortest_1000 = &distances[..distances.len().min(1000)];

    for &(p1, p2, _) in shortest_1000 {
        let same_group = junction_groups.iter().any(|group| {
            group.contains(&p1) && group.contains(&p2)
        });
        
        if same_group {
            iter += 1;
            continue;
        }

        let mut group_indices = Vec::new();
        for (idx, group) in junction_groups.iter().enumerate() {
            if group.contains(&p1) || group.contains(&p2) {
                group_indices.push(idx);
            }
        }
        
        let mut merged_group = HashSet::new();
        
        for &idx in group_indices.iter().rev() {
            let group = junction_groups.remove(idx);
            merged_group.extend(group);
        }
        
        junction_groups.push(merged_group);
    } 

    let mut sizes: Vec<usize> = junction_groups.iter().map(|g| g.len()).collect();

    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let three = &sizes[..sizes.len().min(3)];

    let product: usize = three.iter().product();

    return product;


}

fn second(input: Vec<Coord> ) -> usize {
    let mut iter = 1000;

    let coords = input.clone();

    let mut junction_groups: Vec<HashSet<usize>> = coords.iter()
        .map(|coord| {
            let mut set = HashSet::new();
            set.insert(coord.place);
            set
        })
        .collect();
    
    let mut distances = get_distances(&coords);
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut last_p1: Option<&Coord> = None;
    let mut last_p2: Option<&Coord> = None;

    for &(p1, p2, _d) in distances.iter() {
        let same_group = junction_groups.iter().any(|group| {
            group.contains(&p1) && group.contains(&p2)
        });
        
        if same_group {
            iter += 1;
            continue;
        }

        last_p1 = coords.iter().find(|f| f.place == p1);
        last_p2 = coords.iter().find(|f| f.place == p2);

        let mut group_indices = Vec::new();
        for (idx, group) in junction_groups.iter().enumerate() {
            if group.contains(&p1) || group.contains(&p2) {
                group_indices.push(idx);
            }
        }
        
        let mut merged_group = HashSet::new();
        
        for &idx in group_indices.iter().rev() {
            let group = junction_groups.remove(idx);
            merged_group.extend(group);
        }
        
        junction_groups.push(merged_group);
    } 

    if let (Some(c1), Some(c2)) = (last_p1, last_p2) {
        c1.x * c2.x
    } else {
        0
    }
}