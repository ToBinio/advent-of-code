use std::cmp::Ordering;

use crate::advent_of_code::day::{Day, Year};

pub struct Day12;

impl Day12 {
    pub fn generate_map(input: &str) -> (Vec<Vec<i32>>, (i32, i32), (i32, i32)) {
        let mut map = vec![];
        let mut start_pos = (0, 0);
        let mut goal_pos = (0, 0);

        for (x, line) in input.lines().enumerate() {
            let mut elements = vec![];
            for (y, char) in line.chars().enumerate() {
                let value = match char {
                    'S' => {
                        start_pos = (x as i32, y as i32);
                        0
                    }
                    'E' => {
                        goal_pos = (x as i32, y as i32);
                        25
                    }
                    _ => {
                        char as i32 - 97
                    }
                };

                elements.push(value);
            }

            map.push(elements);
        }

        (map, start_pos, goal_pos)
    }

    pub fn find_shorted_path(map: &Vec<Vec<i32>>, current_pos: (i32, i32), goal_pos: (i32, i32), smallest_path: &mut i32, path: &mut Vec<(i32, i32)>, never_list: &mut Vec<(i32, i32)>) -> bool {
        if never_list.contains(&current_pos) {
            return false;
        }
        if path.contains(&current_pos) {
            return *path.get(path.len() - 2).unwrap() != current_pos;
        }

        if current_pos == goal_pos {
            if *smallest_path == -1 || path.len() < *smallest_path as usize {
                *smallest_path = path.len() as i32;
            }

            println!("{}", path.len());
            return true;
        }
        path.push(current_pos);

        let current_height = Day12::get_value(map, current_pos);

        let mut valid_path_found = false;
        let mut valid_positions = vec![];

        if Day12::valid_move(map, (current_pos.0 + 1, current_pos.1), current_height) {
            valid_positions.push((current_pos.0 + 1, current_pos.1));
        }

        if Day12::valid_move(map, (current_pos.0 - 1, current_pos.1), current_height) {
            valid_positions.push((current_pos.0 - 1, current_pos.1));
        }

        if Day12::valid_move(map, (current_pos.0, current_pos.1 + 1), current_height) {
            valid_positions.push((current_pos.0, current_pos.1 + 1));
        }

        if Day12::valid_move(map, (current_pos.0, current_pos.1 - 1), current_height) {
            valid_positions.push((current_pos.0, current_pos.1 - 1));
        }

        valid_positions.sort_by(|pos1, pos2|
            {
                let height1 = Day12::get_value(map, *pos1);
                let height2 = Day12::get_value(map, *pos2);

                let ordering = height1.cmp(&height2).reverse();

                if ordering != Ordering::Equal {
                    return ordering;
                }

                ((pos1.0 - goal_pos.0).abs() + (pos1.1 - goal_pos.1).abs()).cmp(&((pos2.0 - goal_pos.0).abs() + (pos2.1 - goal_pos.1).abs()))
            }
        );

        for pos in valid_positions {
            if Day12::find_shorted_path(map, pos, goal_pos, smallest_path, path, never_list) {
                valid_path_found = true;
            }
        }

        path.remove(path.iter().position(|element| *element == current_pos).unwrap());

        if !valid_path_found {
            never_list.push(current_pos);
            println!("{}", never_list.len());
        }

        valid_path_found
    }

    pub fn get_value(map: &Vec<Vec<i32>>, pos: (i32, i32)) -> i32 {
        *map.get(pos.0 as usize).unwrap().get(pos.1 as usize).unwrap()
    }

    pub fn valid_move(map: &Vec<Vec<i32>>, pos: (i32, i32), height: i32) -> bool {
        if (map.len() as i32) > pos.0 && pos.0 >= 0 && (map.get(0).unwrap().len() as i32) > pos.1 && pos.1 >= 0 {
            return (height + 1) >= Day12::get_value(map, pos);
        }

        return false;
    }
}

impl Day for Day12 {
    fn get_date() -> (Year, i32) {
        (Year::Year2022, 12)
    }

    fn part_1(input: &str) -> String {
        let (map, start_pos, goal_pos) = Day12::generate_map(input);
        let mut smallest_path = -1;
        Day12::find_shorted_path(&map, start_pos, goal_pos, &mut smallest_path, &mut vec![], &mut vec![]);

        smallest_path.to_string()
    }

    fn part_2(input: &str) -> String {
        todo!()
    }
}