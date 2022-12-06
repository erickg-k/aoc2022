use std::io;

fn remap_char_to_flatten_loc(c: char) -> usize {
    return match c {
        'a'..='z' => c as u32 - 'a' as u32,
        'A'..='Z' => c as u32 - 'A' as u32 + 26,
        _ => 0,
    } as usize;
}

fn remap_char_to_value(c: char) -> u32 {
    return match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0,
    } as u32;
}

fn same_item_by_compartment(rucksack: &str) -> Option<char> {
    let compartment_1 = &rucksack[0..rucksack.len() / 2];
    let compartment_2 = &rucksack[rucksack.len() / 2..rucksack.len()];
    let mut occupancy = [false; 52];
    for c in compartment_1.chars() {
        match c {
            'a'..='z' => {
                occupancy[remap_char_to_flatten_loc(c)] = true;
            }
            'A'..='Z' => {
                occupancy[remap_char_to_flatten_loc(c)] = true;
            }
            _ => todo!(),
        };
    }
    for c in compartment_2.chars() {
        if occupancy[remap_char_to_flatten_loc(c)] {
            return Some(c);
        }
    }
    return None;
}

pub fn get_priorities() {
    let mut priorities = 0;
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let c = same_item_by_compartment(line_str.as_str()).expect("Got an unique item");
        priorities += remap_char_to_value(c);
    }
    println!("priorities: {priorities}");
}
