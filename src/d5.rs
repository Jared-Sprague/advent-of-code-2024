// #![allow(unused)]

//! A solution to day 5 year 2024.
//! https://adventofcode.com/2024/day/5

use std::collections::{HashMap, HashSet};

use indexmap::IndexSet;

type Model = UpdatePack;
type Answer = u64;

#[derive(Debug)]
pub struct UpdatePack {
    rules: HashMap<u32, Rule>,
    updates: Vec<IndexSet<u32>>,
}

impl UpdatePack {
    pub fn get_valid_updates(&self) -> Vec<&IndexSet<u32>> {
        let mut valid_updates: Vec<&IndexSet<u32>> = vec![];

        for update in &self.updates {
            if UpdatePack::is_valid_update(update, &self.rules) {
                valid_updates.push(update);
            }
        }

        valid_updates
    }

    pub fn get_invalid_updates(&self) -> Vec<IndexSet<u32>> {
        let mut invalid_updates: Vec<IndexSet<u32>> = vec![];

        for update in &self.updates {
            if !UpdatePack::is_valid_update(update, &self.rules) {
                invalid_updates.push(update.clone());
            }
        }

        invalid_updates
    }

    pub fn is_valid_update(update: &IndexSet<u32>, rules: &HashMap<u32, Rule>) -> bool {
        for (index, page_num) in update.iter().enumerate() {
            let rule = rules.get(page_num).unwrap();

            if !UpdatePack::is_valid_before(page_num, index, &rule.before, update).0 {
                return false;
            }

            if !UpdatePack::is_valid_after(page_num, index, &rule.after, update).0 {
                return false;
            }
        }

        true
    }

    pub fn reorder_update(update: &mut IndexSet<u32>, rules: &HashMap<u32, Rule>) -> IndexSet<u32> {
        for (index, page_num) in update.iter().enumerate() {
            let rule = rules.get(page_num).unwrap();

            let is_valid_before =
                UpdatePack::is_valid_before(page_num, index, &rule.before, update);
            if !is_valid_before.0 {
                update.swap_indices(index, is_valid_before.1);
                return UpdatePack::reorder_update(update, rules);
            }

            let is_valid_after = UpdatePack::is_valid_after(page_num, index, &rule.after, update);
            if !is_valid_after.0 {
                update.swap_indices(index, is_valid_after.1);
                return UpdatePack::reorder_update(update, rules);
            }
        }

        update.clone()
    }

    pub fn is_valid_before(
        page_num: &u32,
        page_num_index: usize,
        before: &HashSet<u32>,
        update: &IndexSet<u32>,
    ) -> (bool, usize, u32) {
        // validate this page nums index is < pages it needs to come before
        for before_page_num in before {
            // get index of this before_page_num
            if let Some(before_index) = update.get_index_of(before_page_num) {
                if page_num_index >= before_index {
                    // println!("update: {:?}", update);
                    // println!("Invalid: page_num: {page_num} index: {page_num_index}, before_page_num: {before_page_num}, before_index: {before_index}");
                    return (false, before_index, *before_page_num);
                }
            }
        }

        (true, 0, 0)
    }

    pub fn is_valid_after(
        page_num: &u32,
        page_num_index: usize,
        after: &HashSet<u32>,
        update: &IndexSet<u32>,
    ) -> (bool, usize, u32) {
        // validate this page nums index is > pages it needs to come after
        for after_page_num in after {
            // get index of this before_page_num
            if let Some(after_index) = update.get_index_of(after_page_num) {
                if page_num_index <= after_index {
                    // println!("update: {:?}", update);
                    // println!("Invalid: page_num: {page_num} index: {page_num_index}, after_page_num: {after_page_num}, after_index: {after_index}");
                    return (false, after_index, *after_page_num);
                }
            }
        }

        (true, 0, 0)
    }
}

#[derive(Debug)]
pub struct Rule {
    before: HashSet<u32>,
    after: HashSet<u32>,
}

pub fn parse(input: String) -> Model {
    let input = input.trim();

    // first split rules from updates
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut rules: HashMap<u32, Rule> = HashMap::new();
    let mut updates: Vec<IndexSet<u32>> = vec![];

    // parse out rules
    let rule_lines: Vec<&str> = parts[0].split("\n").collect();
    for line in rule_lines {
        let rule_parts: Vec<&str> = line.split("|").collect();
        let left = rule_parts[0].parse::<u32>().unwrap();
        let right = rule_parts[1].parse::<u32>().unwrap();

        // update the before rules
        if let Some(rule) = rules.get_mut(&left) {
            rule.before.insert(right);
        } else {
            let mut before: HashSet<u32> = HashSet::new();
            let after: HashSet<u32> = HashSet::new();
            before.insert(right);
            rules.insert(left, Rule { before, after });
        }

        // update the after rules
        if let Some(rule) = rules.get_mut(&right) {
            rule.after.insert(left);
        } else {
            let before: HashSet<u32> = HashSet::new();
            let mut after: HashSet<u32> = HashSet::new();
            after.insert(left);
            rules.insert(right, Rule { before, after });
        }
    }

    // dbg!(&rules);

    // now parse the updates
    let update_lines: Vec<&str> = parts[1].split("\n").collect();
    for line in update_lines {
        let pages: Vec<&str> = line.split(",").collect();
        let mut update: IndexSet<u32> = IndexSet::new();
        pages.iter().for_each(|p| {
            let page_num = p.parse::<u32>().unwrap();
            update.insert(page_num);
        });

        updates.push(update);
    }

    UpdatePack { rules, updates }
}

pub fn part1(model: Model) -> Answer {
    let valid_updates = model.get_valid_updates();
    let mut total: u64 = 0;
    for update in valid_updates {
        // get middle
        let middle_index = update.len() / 2;
        // dbg!(update);
        // println!("{middle_index}")
        total += *update.get_index(middle_index).unwrap() as u64;
    }
    total
}

pub fn part2(model: Model) -> Answer {
    let mut invalid_updates = model.get_invalid_updates();
    let mut reordered_updates: Vec<IndexSet<u32>> = vec![];
    let mut total = 0;

    // invalid_updates.iter_mut().for_each(|u|);

    for update in invalid_updates.iter_mut() {
        let reordered_update = UpdatePack::reorder_update(update, &model.rules);
        reordered_updates.push(reordered_update);
    }

    for update in reordered_updates {
        // get middle
        let middle_index = update.len() / 2;
        // dbg!(update);
        // println!("{middle_index}")
        total += *update.get_index(middle_index).unwrap() as u64;
    }
    total
}
