use std::collections::HashMap;
use std::io;

pub fn score_by_guide() {
    let resposne_score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let game_outcome_score: HashMap<&str, HashMap<&str, i32>> = HashMap::from([
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
        let response_score = resposne_score.get(mine).expect("Got a score");
        let outcome_score = game_outcome_score
            .get(mine)
            .expect("Got an outcome")
            .get(opponent)
            .expect("Got a score");
        score += response_score + outcome_score;
    }
    println!("Score: {score}");
}
