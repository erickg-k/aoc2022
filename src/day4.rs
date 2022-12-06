use std::io;
use std::vec::Vec;

fn analyze_assignment(assignment: &str) -> (i32, Vec<i32>) {
    let seg: Vec<i32> = assignment
        .split('-')
        .map(|e| e.parse::<i32>().expect("parse as an i32"))
        .collect();
    assert_eq!(seg.len(), 2);
    return (seg[1] - seg[0] + 1, seg);
}

fn segment_inclusion(segment: &str) -> bool {
    let pairs: Vec<&str> = segment.split(',').collect();
    assert_eq!(pairs.len(), 2);
    let (len1, seg1) = analyze_assignment(pairs[0]);
    let (len2, seg2) = analyze_assignment(pairs[1]);
    if len1 <= len2 {
        return seg2[1] >= seg1[1] && seg2[0] <= seg1[0];
    } else {
        return seg1[1] >= seg2[1] && seg1[0] <= seg2[0];
    }
}

fn segment_overlap(segment: &str) -> bool {
    let pairs: Vec<&str> = segment.split(',').collect();
    assert_eq!(pairs.len(), 2);
    let (_len1, seg1) = analyze_assignment(pairs[0]);
    let (_len2, seg2) = analyze_assignment(pairs[1]);
    return seg1[1] >= seg2[0] && seg1[0] <= seg2[1];
}

pub fn count_segments_inclusions() {
    let mut cnt = 0;
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");

        if segment_inclusion(line_str.as_str()) {
            cnt += 1;
        }
    }
    println!("Count: {cnt}");
}

pub fn count_segments_overlap() {
    let mut cnt = 0;
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");

        if segment_overlap(line_str.as_str()) {
            cnt += 1;
        }
    }
    println!("Count: {cnt}");
}
