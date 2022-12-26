use crate::char_bins;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::vec::Vec;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Conlist<T> {
    Head(T),
    Con(Vec<Conlist<T>>),
}

#[derive(Debug)]
enum Token {
    ListBegin,
    ListEnd,
    Number(i32),
}

fn get_number_from_queue(queue: &mut VecDeque<i32>) -> Option<i32> {
    if queue.len() > 0 {
        let mut number = 0;
        while let Some(digit) = queue.pop_front() {
            number = number * 10 + digit;
        }
        Some(number)
    } else {
        None
    }
}

fn get_tokens_streams(input: &mut impl Read) -> Vec<VecDeque<Token>> {
    let mut streams = Vec::new();

    for line in BufReader::new(input).lines() {
        let line_str = line.expect("IO failed reading data");
        if line_str == "" {
        } else {
            let mut tokens = VecDeque::new();
            let mut number_queue = VecDeque::new();
            for c in line_str.chars() {
                match c {
                    '[' => tokens.push_back(Token::ListBegin),
                    ']' => {
                        if let Some(number) = get_number_from_queue(&mut number_queue) {
                            tokens.push_back(Token::Number(number));
                        }
                        tokens.push_back(Token::ListEnd);
                    }
                    ',' => {
                        if let Some(number) = get_number_from_queue(&mut number_queue) {
                            tokens.push_back(Token::Number(number));
                        }
                    }
                    _ => number_queue.push_back(char_bins::remap_char_to_i32(c)),
                };
            }
            streams.push(tokens);
        }
    }
    streams
}

fn parse_list_contents(tokens: &mut VecDeque<Token>) -> Vec<Conlist<i32>> {
    let mut elems = vec![];

    let mut tok = tokens.pop_front().expect("list contents");
    if let Token::ListEnd = tok {
        return elems;
    }

    loop {
        if let Token::Number(i) = tok {
            elems.push(Conlist::Head(i));
        } else if let Token::ListBegin = tok {
            elems.push(Conlist::Con(parse_list_contents(tokens)));
        } else {
            panic!("expected list element: {:?}", tok);
        }

        if tokens.len() <= 0 {
            break;
        }
        tok = tokens.pop_front().expect("after element");
        if let Token::ListEnd = tok {
            break;
        }
    }

    elems
}

fn get_packets(input: &mut impl Read) -> Vec<(Conlist<i32>, Conlist<i32>)> {
    let mut packets = Vec::new();
    let mut packet_pairs = Vec::new();

    let tokens_stream = get_tokens_streams(input);
    for mut stream in tokens_stream {
        packet_pairs.push(parse_list_contents(&mut stream).pop().unwrap());

        if packet_pairs.len() >= 2 {
            let second = packet_pairs.pop().unwrap();
            packets.push((packet_pairs.pop().unwrap(), second));
        }
    }
    packets
}

impl<T: std::cmp::PartialEq + std::cmp::PartialOrd + Copy> PartialOrd for Conlist<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Conlist::Head(na) => match other {
                Conlist::Head(nb) => {
                    if na < nb {
                        Some(Ordering::Less)
                    } else if na == nb {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                Conlist::Con(_) => Conlist::Con(vec![Conlist::Head(*na)]).partial_cmp(&other),
            },
            Conlist::Con(con_a) => match other {
                Conlist::Head(nb) => self.partial_cmp(&Conlist::Con(vec![Conlist::Head(*nb)])),
                Conlist::Con(con_b) => {
                    let len_a = con_a.len();
                    let len_b = con_b.len();
                    if len_a > 0 && len_b > 0 {
                        let head_result = con_a[0].partial_cmp(&con_b[0]);
                        if head_result != Some(Ordering::Equal) {
                            head_result
                        } else {
                            con_a[1..].partial_cmp(&con_b[1..])
                        }
                    } else {
                        Some(len_a.cmp(&len_b))
                    }
                }
            },
        }
    }
}

pub fn get_distress_signal() {
    let packets = get_packets(&mut io::stdin());

    let mut sum = 0;
    for i in 0..packets.len() {
        let index = i + 1;
        println!("========={}========", index);
        let success = &packets[i].0 < &packets[i].1;
        if success {
            sum += index;
        }
        println!("{:?}\n\n", success);
    }
    println!("sum={}", sum);
}

pub fn get_distress_signal_decorder_key() {
    let packets = get_packets(&mut io::stdin());
    let mut flatten = Vec::new();
    for (a, b) in packets {
        flatten.push(a);
        flatten.push(b);
    }
    let divider_a = Conlist::Con(vec![Conlist::Con(vec![Conlist::Head(2)])]);
    let divider_b = Conlist::Con(vec![Conlist::Con(vec![Conlist::Head(6)])]);
    flatten.push(divider_a.clone());
    flatten.push(divider_b.clone());
    flatten.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let result = (flatten
        .binary_search_by(|probe| probe.partial_cmp(&divider_a).unwrap())
        .unwrap()
        + 1)
        * (flatten
            .binary_search_by(|probe| probe.partial_cmp(&divider_b).unwrap())
            .unwrap()
            + 1);
    println!("result={:?}", result);
}
