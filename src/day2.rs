use std::collections::HashMap;
use std::io;

pub fn score_by_guide() {
    let resposne_scores = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let game_outcome_scores: HashMap<&str, HashMap<&str, i32>> = HashMap::from([
        ("X", HashMap::from([("A", 3), ("B", 0), ("C", 6)])),
        ("Y", HashMap::from([("A", 6), ("B", 3), ("C", 0)])),
        ("Z", HashMap::from([("A", 0), ("B", 6), ("C", 3)])),
    ]);

    let mut score = 0;
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let mut iter = line_str.as_str().split_whitespace().clone();
        let opponent = iter.next().expect("Got opponent's response");
        let mine = iter.next().expect("Got my response");
        let response_score = resposne_scores.get(mine).expect("Got a score");
        let outcome_score = game_outcome_scores
            .get(mine)
            .expect("Got an outcome map")
            .get(opponent)
            .expect("Got a score");
        score += response_score + outcome_score;
    }
    println!("Score: {score}");
}

// it's easier to use string instead of hashmap
pub fn score_by_secret_guide() {
    let resposne_scores = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let game_outcome_scores = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);
    let game_outcome_response = HashMap::from([
        ("X", HashMap::from([("A", "C"), ("B", "A"), ("C", "B")])), // lose
        ("Y", HashMap::from([("A", "A"), ("B", "B"), ("C", "C")])), // draw
        ("Z", HashMap::from([("A", "B"), ("B", "C"), ("C", "A")])), // win
    ]);

    let mut score = 0;
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let mut iter = line_str.as_str().split_whitespace().clone();
        let opponent = iter.next().expect("Got opponent's response");
        let outcome = iter.next().expect("Got game outcome");
        let response = game_outcome_response
            .get(outcome)
            .expect("Got an outcome response")
            .get(opponent)
            .expect("Got a response");
        let response_score = resposne_scores.get(response).expect("get a response score");
        let outcome_score = game_outcome_scores.get(outcome).expect("get a outcome score");
        score += response_score + outcome_score;
    }
    println!("Score: {score}");
}
