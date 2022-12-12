use colored::Colorize;

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

    pub fn find_shorted_path(map: &Vec<Vec<i32>>, current_pos: (i32, i32), goal_pos: (i32, i32)) -> i32 {
        let mut open_cells = vec![Cell { pos: current_pos, path_cell: None, path_count: 0 }];
        let mut closed_cells: Vec<Cell> = vec![];

        while open_cells.len() > 0 {
            open_cells.sort_by(|cell1, cell2|
                {
                    (Day12::get_distance_to_goal(cell1.pos, goal_pos) + cell1.path_count).cmp(&(Day12::get_distance_to_goal(cell2.pos, goal_pos) + cell2.path_count)).reverse()
                });

            let current_cell = open_cells.pop().unwrap();

            Day12::handle_cell(map, (current_cell.pos.0 + 1, current_cell.pos.1), &current_cell, &mut open_cells, &mut closed_cells);
            Day12::handle_cell(map, (current_cell.pos.0 - 1, current_cell.pos.1), &current_cell, &mut open_cells, &mut closed_cells);
            Day12::handle_cell(map, (current_cell.pos.0, current_cell.pos.1 + 1), &current_cell, &mut open_cells, &mut closed_cells);
            Day12::handle_cell(map, (current_cell.pos.0, current_cell.pos.1 - 1), &current_cell, &mut open_cells, &mut closed_cells);

            closed_cells.push(current_cell);
        }

        let mut cells = vec![];

        let mut last_pos = goal_pos;
        if closed_cells.iter().find(|other| other.pos == last_pos).is_none() {
            return -1;
        }

        loop {
            cells.push(last_pos.clone());
            let cell = closed_cells.iter().find(|other| other.pos == last_pos).unwrap();

            if cell.path_cell.is_none() {
                break;
            }

            last_pos = cell.path_cell.unwrap();
        }

        cells.len() as i32 - 1
    }

    pub fn handle_cell(map: &Vec<Vec<i32>>, pos: (i32, i32), current_cell: &Cell, open_cells: &mut Vec<Cell>, closed_cells: &mut Vec<Cell>) {
        if Day12::valid_move(map, pos, Day12::get_value(map, current_cell.pos)) {
            let mut cell_to_open = open_cells.iter_mut().find(|cell| cell.pos == pos);
            if cell_to_open.is_none() {
                cell_to_open = closed_cells.iter_mut().find(|cell| cell.pos == pos);
            }

            match cell_to_open {
                None => {
                    open_cells.push(Cell {
                        pos,
                        path_cell: Some(current_cell.pos),
                        path_count: current_cell.path_count + 1,
                    })
                }
                Some(cell) => {
                    if cell.path_count > (current_cell.path_count + 1) {
                        cell.path_count = current_cell.path_count + 1;
                        cell.path_cell = Some(current_cell.pos);
                    }
                }
            }
        }
    }

    pub fn get_distance_to_goal(current_pos: (i32, i32), goal_pos: (i32, i32)) -> i32 {
        (current_pos.0 - goal_pos.0).abs() + (current_pos.1 - goal_pos.1).abs()
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
        Day12::find_shorted_path(&map, start_pos, goal_pos).to_string()
    }

    fn part_2(input: &str) -> String {
        let (map, _, goal_pos) = Day12::generate_map(input);

        let mut smallest = -1;

        for (y, line) in map.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                if *value != 0 {
                    continue;
                }

                let path = Day12::find_shorted_path(&map, (y as i32, x as i32), goal_pos);

                if path == -1 {
                    continue;
                }

                if smallest == -1 || path < smallest {
                    smallest = path;
                }
            }
        }

        smallest.to_string()
    }
}

pub struct Cell {
    pub pos: (i32, i32),
    pub path_cell: Option<(i32, i32)>,
    pub path_count: i32,
}