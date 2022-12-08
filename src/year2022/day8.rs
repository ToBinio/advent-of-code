use crate::advent_of_code::day::{Day, Year};

pub struct Day8;

impl Day8 {
    pub fn make_grid(input: &str) -> Vec<Vec<i32>> {
        let mut grid = vec![];

        for line in input.lines() {
            grid.push(vec![]);
            for value in line.chars() {
                let value = value.to_string().parse::<i32>().unwrap();

                grid.last_mut().unwrap().push(value);
            }
        }

        grid
    }

    pub fn get_value(pos: (usize, usize), grid: &Vec<Vec<i32>>) -> i32 {
        *grid.get(pos.1).unwrap().get(pos.0).unwrap()
    }

    pub fn is_visible(pos: (usize, usize), grid: &Vec<Vec<i32>>) -> bool {
        let size = (grid.len(), grid.get(0).unwrap().len());

        let height = Day8::get_value(pos, grid);

        let mut is_visible = true;

        for x in 0..pos.0 {
            if height <= Day8::get_value((x, pos.1), grid) {
                is_visible = false;
                break;
            }
        }

        if is_visible {
            return true;
        }

        is_visible = true;

        for x in (pos.0 + 1)..size.0 {
            if height <= Day8::get_value((x, pos.1), grid) {
                is_visible = false;
                break;
            }
        }

        if is_visible {
            return true;
        }

        is_visible = true;

        for y in 0..pos.1 {
            if height <= Day8::get_value((pos.0, y), grid) {
                is_visible = false;
                break;
            }
        }

        if is_visible {
            return true;
        }

        is_visible = true;

        for y in (pos.1 + 1)..size.1 {
            if height <= Day8::get_value((pos.0, y), grid) {
                is_visible = false;
                break;
            }
        }

        return is_visible;
    }


    pub fn get_scenic_score(pos: (usize, usize), grid: &Vec<Vec<i32>>) -> i32 {
        let size = (grid.len(), grid.get(0).unwrap().len());

        let tree_height = Day8::get_value(pos, grid);

        let mut score = (0, 0, 0, 0);

        for x in (0..pos.0).rev() {
            let height = Day8::get_value((x, pos.1), grid);

            score.0 += 1;

            if height >= tree_height {
                break;
            }
        }

        for x in (pos.0 + 1)..size.0 {
            let height = Day8::get_value((x, pos.1), grid);

            score.1 += 1;

            if height >= tree_height {
                break;
            }
        }

        for y in (0..pos.1).rev() {
            let height = Day8::get_value((pos.0, y), grid);

            score.2 += 1;

            if height >= tree_height {
                break;
            }
        }

        for y in (pos.1 + 1)..size.1 {
            let height = Day8::get_value((pos.0, y), grid);

            score.3 += 1;

            if height >= tree_height {
                break;
            }
        }

        let count_score = score.0 * score.1 * score.2 * score.3;

        count_score
    }
}

impl Day for Day8 {
    fn get_date() -> (Year, i32) {
        (Year::Year2022, 8)
    }

    fn part_1(input: &str) -> String {
        let grid = Day8::make_grid(input);

        let mut count = 0;

        for x in 0..grid.len() {
            for y in 0..grid.get(0).unwrap().len() {
                if Day8::is_visible((x, y), &grid) {
                    count += 1;
                }
            }
        }

        count.to_string()
    }

    fn part_2(input: &str) -> String {
        let grid = Day8::make_grid(input);

        let mut best = 0;

        for x in 0..grid.len() {
            for y in 0..grid.get(0).unwrap().len() {
                let score = Day8::get_scenic_score((x, y), &grid);

                if best < score {
                    best = score;
                }
            }
        }

        best.to_string()
    }
}