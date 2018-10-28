extern crate atcoder_client;

use atcoder_client::contests::{get_standings, StandingsData};
use atcoder_client::users::get_history;
use std::collections::HashSet;
use std::env;

fn solved(s: &StandingsData, task_id: &str) -> bool {
    if let Some(t) = s.result(task_id) {
        t.is_solved()
    } else {
        false
    }
}

fn get_rated_contest_ids(user_id: &str) -> Vec<String> {
    let user = get_history(user_id).unwrap();
    user.iter()
        .filter(|c| c.is_rated)
        .map(|c| c.contest_id().unwrap().to_string())
        .collect()
}

fn get_solved(user_id: &str, contest_id: &str) -> HashSet<String> {
    let standings = get_standings(contest_id).unwrap();
    let tasks = standings.task_ids();
    let s = standings
        .standings()
        .iter()
        .filter(|s| s.user_id() == user_id)
        .next()
        .unwrap();
    tasks
        .iter()
        .filter(|t| solved(s, t))
        .map(|t| t.to_string())
        .collect()
}

fn get_tasks_with_performance(contest_id: &str) -> Vec<(String, Option<i32>)> {
    let standings = get_standings(contest_id).unwrap();
    let tasks = standings.task_ids();
    let mut prefix = Vec::new();
    let mut res = Vec::new();
    for t in &tasks {
        // TODO prefix should be A, B, D if C and D has same point and A and B has less point
        prefix.push(t);

        let user = standings
            .standings()
            .iter()
            .filter(|s| s.is_rated)
            .filter(|s| prefix.iter().all(|p| solved(s, p)))
            .map(|s| s.user_id())
            .last();

        let history = user.map(|u| get_history(u).unwrap());

        let competition = history.and_then(|h| {
            h.into_iter()
                .filter(|c| c.contest_id().unwrap_or("") == contest_id)
                .next()
        });

        let performance = competition.map(|c| c.performance);

        res.push((t.to_string(), performance));
    }
    res
}

fn get_unsolved_tasks(user_id: &str) -> Vec<(String, String, i32)> {
    let contest_ids = get_rated_contest_ids(&user_id);
    let mut res = Vec::new();
    for contest_id in contest_ids {
        eprintln!("{}", contest_id);
        let solved = get_solved(&user_id, &contest_id);
        let tasks = get_tasks_with_performance(&contest_id);
        for (t, p) in tasks {
            if !solved.contains(&t) {
                if let Some(p) = p {
                    res.push((contest_id.to_string(), t, p));
                }
            }
        }
    }
    res.sort_by_key(|(_, _, p)| *p);
    res
}

fn main() {
    let user_id = match env::args().nth(1) {
        Some(x) => x,
        None => panic!("user_id should be specified as argument"),
    };
    for (c, t, p) in get_unsolved_tasks(&user_id) {
        println!("{} {} {}", c, t, p);
    }
}
