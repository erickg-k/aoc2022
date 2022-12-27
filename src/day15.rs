use crate::matrix::manhattan_distance_i64;
use crate::matrix::Point;
use std::cmp;
use std::collections::HashSet;
use std::io;
use std::ops::Range;
use std::vec::Vec;

#[derive(Debug)]
struct BoundedSensor {
    sensor: Point<i64>,
    beacon: Point<i64>,
}

#[derive(Debug)]
struct ManhattanSensor {
    sensor: Point<i64>,
    distance: i64,
}

fn get_locations() -> (Vec<BoundedSensor>, Point<i64>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut sensors = Vec::new();
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let parts: Vec<&str> = line_str.split(": ").collect();
        let mut sensor_str = parts[0];
        sensor_str = &sensor_str[12..sensor_str.len()];
        let sensor_point: Vec<i64> = sensor_str
            .split(", y=")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut beacon_str = parts[1];
        beacon_str = &beacon_str[23..beacon_str.len()];
        let beacon_point: Vec<i64> = beacon_str
            .split(", y=")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        sensors.push(BoundedSensor {
            sensor: Point::<i64> {
                x: sensor_point[0],
                y: sensor_point[1],
            },
            beacon: Point::<i64> {
                x: beacon_point[0],
                y: beacon_point[1],
            },
        });
        max_x = cmp::max(max_x, sensor_point[0]);
        max_x = cmp::max(max_x, beacon_point[0]);
        max_y = cmp::max(max_y, sensor_point[1]);
        max_y = cmp::max(max_y, beacon_point[1]);
    }
    (sensors, Point::<i64> { x: max_x, y: max_y })
}

fn get_manhattan_sensors(sensors: &Vec<BoundedSensor>) -> Vec<ManhattanSensor> {
    let mut manhattan = Vec::new();
    for sensor in sensors {
        let distance = manhattan_distance_i64(&sensor.sensor, &sensor.beacon);
        manhattan.push(ManhattanSensor {
            sensor: sensor.sensor.clone(),
            distance,
        });
    }
    manhattan
}

fn merge_ranges(ranges: &Vec<Range<i64>>) -> Vec<Range<i64>> {
    let mut sorted = ranges.clone();
    sorted.sort_by(|a, b| a.start.cmp(&b.start));

    let mut res = Vec::new();
    if sorted.len() == 0 {
        return res;
    }

    res.push(sorted[0].clone());
    for inteval in &sorted[1..] {
        let top = res.first().unwrap().clone();
        if top.end < inteval.start {
            // no overlap
            res.push(inteval.clone());
        } else if top.end == inteval.start || top.end < inteval.end {
            let new_inteval = top.start..inteval.end;
            res.pop();
            res.push(new_inteval);
        }
    }
    res
}

const TARGET_Y: i64 = 2000000;

pub fn get_num_positions_no_beacon() {
    let (sensors, _) = get_locations();
    let manhattan = get_manhattan_sensors(&sensors);
    let mut ranges = Vec::new();
    for bounded_manhattan in manhattan {
        if ((bounded_manhattan.sensor.y - bounded_manhattan.distance)
            ..=(bounded_manhattan.sensor.y + bounded_manhattan.distance))
            .contains(&TARGET_Y)
        {
            // found a sensor in range of TARGET_Y
            let used_diff_y = i64::abs(bounded_manhattan.sensor.y - TARGET_Y);
            let rest_x = bounded_manhattan.distance - used_diff_y;
            ranges.push(
                (bounded_manhattan.sensor.x - rest_x)..(bounded_manhattan.sensor.x + rest_x + 1),
            );
        }
    }
    ranges = merge_ranges(&ranges);

    let mut sum = 0;
    let x_points: Vec<i64> = sensors
        .iter()
        .flat_map(|b| [&b.sensor, &b.beacon])
        .filter(|p| p.y == TARGET_Y)
        .map(|p| p.x)
        .collect();
    let x_point_set: HashSet<i64> = x_points.into_iter().collect();
    for r in ranges {
        let mut seg_sum = r.end - r.start;
        for x_point in &x_point_set {
            if r.contains(&x_point) {
                seg_sum -= 1;
            }
        }
        sum += seg_sum;
    }
    println!("sum={}", &sum);
}

const SEARCH_BOUND: i64 = 4000000;

pub fn get_distress_beacon() {
    let (sensors, _) = get_locations();
    let manhattan = get_manhattan_sensors(&sensors);

    for target_y in 0..=SEARCH_BOUND {
        let mut ranges = Vec::new();
        for bounded_manhattan in &manhattan {
            if ((bounded_manhattan.sensor.y - bounded_manhattan.distance)
                ..=(bounded_manhattan.sensor.y + bounded_manhattan.distance))
                .contains(&target_y)
            {
                // found a sensor in range of TARGET_Y
                let used_diff_y = i64::abs(bounded_manhattan.sensor.y - target_y);
                let rest_x = bounded_manhattan.distance - used_diff_y;
                ranges.push(
                    (bounded_manhattan.sensor.x - rest_x)
                        ..(bounded_manhattan.sensor.x + rest_x + 1),
                );
            }
        }
        ranges = merge_ranges(&ranges);
        println!("target_y={} {:?}", target_y, ranges);
        if ranges.len() == 1 && ranges[0].start < 0 && ranges[0].end > SEARCH_BOUND {
            continue;
        }

        // this O(n^2) is slow but we can cut lots of branches above.
        for target_x in 0..=SEARCH_BOUND {
            let mut matched = false;
            for range in &ranges {
                if !matched && range.contains(&target_x) {
                    matched = true;
                }
            }
            if !matched {
                println!("x={} y={}", target_x, target_y);
                println!("score={}", 4000000 * target_x + target_y);
                return;
            }
        }
    }
}
