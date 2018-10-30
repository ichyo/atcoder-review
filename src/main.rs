extern crate atcoder_client;

use atcoder_client::contests::{get_standings, Standings, StandingsData};
use atcoder_client::users::{get_history, Competition};
use std::env;

struct User(Vec<Competition>);
struct Contest(String, Standings);
struct UserResult<'a>(&'a StandingsData);

impl Contest {
    fn fetch(contest_id: &str) -> Option<Contest> {
        match get_standings(contest_id) {
            Ok(x) => Some(Contest(contest_id.to_string(), x)),
            Err(_) => None,
        }
    }

    fn id(&self) -> &str {
        &self.0
    }

    fn results(&self) -> Vec<UserResult> {
        self.1
            .standings_data
            .iter()
            .map(|s| UserResult(s))
            .collect()
    }

    fn task_ids(&self) -> Vec<String> {
        self.1
            .task_info
            .iter()
            .map(|info| info.id().to_string())
            .collect()
    }
}

impl User {
    fn fetch(user_id: &str) -> Option<User> {
        match get_history(user_id) {
            Ok(h) => Some(User(h)),
            Err(_) => None,
        }
    }

    fn rated_contests(&self) -> Vec<Contest> {
        self.0
            .iter()
            .filter(|p| p.is_rated)
            .filter_map(|p| p.contest_id())
            .filter_map(|s| Contest::fetch(s))
            .collect::<Vec<_>>()
    }
}

impl<'a> UserResult<'a> {
    fn user_id(&self) -> &str {
        &self.0.user_screen_name
    }

    fn is_rated(&self) -> bool {
        self.0.is_rated
    }

    fn rating(&self) -> i32 {
        self.0.old_rating
    }

    fn has_solved(&self, task_id: &str) -> bool {
        match self.0.result(task_id) {
            Some(r) => r.is_solved(),
            None => false,
        }
    }
}

fn main() {
    let user_id = match env::args().nth(1) {
        Some(x) => x,
        None => panic!("user_id should be specified as argument"),
    };

    // TODO: argument
    let lower_rating = 2400;
    let upper_rating = 2800;

    let user = match User::fetch(&user_id) {
        Some(u) => u,
        None => panic!("unknown user id"),
    };

    let contests = user.rated_contests();

    for contest in contests {
        let task_ids = contest.task_ids();

        let results = contest.results();
        let target_results = results
            .iter()
            .filter(|r| r.is_rated())
            .filter(|r| r.rating() >= lower_rating)
            .filter(|r| r.rating() <= upper_rating)
            .collect::<Vec<_>>();

        let my_result = match results.iter().filter(|r| r.user_id() == user_id).next() {
            Some(r) => r,
            None => panic!("this shouldn't happen. something wrong"),
        };

        if target_results.is_empty() {
            eprintln!("No target user found for a contest {}. Skip.", contest.id());
            continue;
        }

        for task_id in task_ids {
            if my_result.has_solved(&task_id) {
                continue;
            }

            let solved_count = target_results
                .iter()
                .filter(|r| r.has_solved(&task_id))
                .count();
            let total_count = target_results.len();
            let solved_ratio = (solved_count as f64) / (total_count as f64);

            println!(
                "{}: {} / {} ({:.2} %)",
                task_id,
                solved_count,
                total_count,
                solved_ratio * 100.0,
            );
        }
    }
}
