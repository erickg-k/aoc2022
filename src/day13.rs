/// tree styled nested list implementation
/// I should use a conlist.
use crate::char_bins;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::rc::Rc;
use std::rc::Weak;
use std::vec::Vec;

#[derive(Clone, Debug)]
enum Payload {
    List(Rc<RefCell<Node>>), // sub_level node
    Number(i32),             // well, a number
    None,
}

#[derive(Debug)]
struct Node {
    payload: Payload,
    sibling: Option<Rc<RefCell<Node>>>,
    level: i32,
    parent: Weak<RefCell<Node>>,
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

fn get_packets(input: &mut impl Read) -> Vec<(Rc<RefCell<Node>>, Rc<RefCell<Node>>)> {
    let mut packets = Vec::new();
    let mut packet_pairs = Vec::new();

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

            let mut node = Rc::new(RefCell::new(Node {
                payload: Payload::None,
                level: 0,
                sibling: None,
                parent: Weak::new(),
            }));
            let root = Rc::clone(&node);
            while let Some(token) = tokens.pop_front() {
                let cur = Rc::clone(&node);
                match token {
                    Token::ListBegin => {
                        let next_node = Rc::new(RefCell::new(Node {
                            payload: Payload::None,
                            sibling: None,
                            level: (*cur).borrow().level + 1,
                            parent: Rc::downgrade(&cur),
                        }));
                        let tmp_next = Rc::clone(&next_node);
                        let mut node_borrow = (*cur).borrow_mut();
                        match &node_borrow.payload {
                            Payload::None => {
                                // always create a empty node when found a new list to populate the new stuff there.
                                node_borrow.payload = Payload::List(next_node);
                            }
                            _ => {
                                // higher level list had been filled, and it moved back to the parent node so we need a new sibling with node pointer.
                                let tmp_next_node = Rc::clone(&next_node);
                                let sibling_node = Rc::new(RefCell::new(Node {
                                    payload: Payload::List(next_node),
                                    sibling: None,
                                    level: node_borrow.level,
                                    parent: Weak::clone(&node_borrow.parent),
                                }));
                                (*tmp_next_node).borrow_mut().parent = Rc::downgrade(&sibling_node);
                                node_borrow.sibling = Some(Rc::clone(&sibling_node));
                            }
                        }
                        node = tmp_next;
                    }
                    Token::Number(n) => {
                        let mut node_borrow = (*cur).borrow_mut();
                        match &node_borrow.payload {
                            Payload::None => {
                                node_borrow.payload = Payload::Number(n);
                            }
                            _ => {
                                let next_node = Rc::new(RefCell::new(Node {
                                    payload: Payload::Number(n),
                                    level: node_borrow.level,
                                    sibling: None,
                                    parent: Weak::clone(&node_borrow.parent),
                                }));
                                node_borrow.sibling = Some(Rc::clone(&next_node));
                                node = Rc::clone(&next_node);
                            }
                        }
                    }
                    Token::ListEnd => {
                        let parent = (*node).borrow().parent.upgrade().unwrap();
                        node = parent;
                    }
                }
            }
            packet_pairs.push(root);
        }

        if packet_pairs.len() >= 2 {
            let second = packet_pairs.pop().unwrap();
            packets.push((packet_pairs.pop().unwrap(), second));
        }
    }
    packets
}

#[allow(dead_code)]
fn print_packet(root: Rc<RefCell<Node>>) {
    let mut s = Vec::from([Rc::clone(&root)]);
    while let Some(node) = s.pop() {
        if let Some(sibling) = &(*node).borrow().sibling {
            s.push(Rc::clone(sibling));
        }

        let node_borrow = &(*node).borrow();
        let blanks = "  ".repeat(node_borrow.level as usize);
        print!("{}", blanks);

        match &node_borrow.payload {
            Payload::List(list_node) => {
                println!(">");
                s.push(Rc::clone(&list_node));
            }
            Payload::Number(n) => {
                println!("{},", n);
            }
            _ => continue,
        }
    }
}

fn compare_packet_heads(a: &Payload, b: &Payload) -> bool {
    // println!("a={:?} b={:?}", &a, &b);
    match a {
        Payload::None => match b {
            _ => {
                return true;
            }
        },
        Payload::List(a_next_level) => match b {
            Payload::None => {
                return true;
            }
            Payload::List(b_next_level) => {
                return compare_packets(Rc::clone(&a_next_level), Rc::clone(&b_next_level));
            }
            Payload::Number(_) => {
                return false;
            }
        },
        Payload::Number(n) => match b {
            Payload::None => {
                return true;
            }
            Payload::List(_) => {
                return true;
            }
            Payload::Number(nb) => {
                return n <= nb;
            }
        },
    }
}

fn split_head(node: Rc<RefCell<Node>>) -> (Payload, Option<Rc<RefCell<Node>>>) {
    let borrow = (*node).borrow();
    let cons = match &borrow.sibling {
        Some(sibling) => Some(Rc::clone(sibling)),
        None => None,
    };
    return (borrow.payload.clone(), cons);
}

fn compare_packets(a: Rc<RefCell<Node>>, b: Rc<RefCell<Node>>) -> bool {
    let (head_a, maybe_con_a) = split_head(Rc::clone(&a));
    let (head_b, maybe_con_b) = split_head(Rc::clone(&b));

    // do one more unwrap when compare a list to a non-list
    let head_comparison = if let Payload::List(sub_a) = &head_a {
        if let Payload::Number(_) = &head_b {
            compare_packet_heads(&(*sub_a).borrow().payload, &head_b)
        } else {
            compare_packet_heads(&head_a, &head_b)
        }
    } else if let Payload::Number(_) = &head_a {
        if let Payload::List(sub_b) = &head_b {
            compare_packet_heads(&head_a, &(*sub_b).borrow().payload)
        } else {
            compare_packet_heads(&head_a, &head_b)
        }
    } else {
        compare_packet_heads(&head_a, &head_b)
    };
    println!(
        "head_a={:?} head_b={:?}, head_comparison={}",
        &head_a, &head_b, head_comparison
    );

    if !head_comparison {
        return false;
    }
    if let Some(con_a) = maybe_con_a {
        if let Some(con_b) = maybe_con_b {
            return compare_packets(Rc::clone(&con_a), Rc::clone(&con_b));
        } else {
            return false;
        }
    }
    true
}

pub fn get_distress_signal() {
    let packets = get_packets(&mut io::stdin());

    let mut sum = 0;
    for i in 0..packets.len() {
        let index = i + 1;
        println!("\n\n\n========{}========", index);
        print_packet(Rc::clone(&packets[i].0));
        println!("\n");
        print_packet(Rc::clone(&packets[i].1));
        let success = compare_packets(Rc::clone(&packets[i].0), Rc::clone(&packets[i].1));
        if success {
            sum += index;
        }
        println!("{}", success);
    }
    println!("sum={}", sum);

    // println!("{:?}", packets);
    // print_packet(Rc::clone(&packets[7].1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = get_packets(&mut "[1,1,3,1,1]\n[1,1,5,1,1]\n".as_bytes());
        assert!(compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }

    #[test]
    fn test_case_2() {
        let s = get_packets(&mut "[[1],[2,3,4]]\n[[1],4]\n".as_bytes());
        assert!(compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }

    #[test]
    fn test_case_3() {
        let s = get_packets(&mut "[9]\n[[8,7,6]]\n".as_bytes());
        assert!(!compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }

    #[test]
    fn test_case_4() {
        let s = get_packets(&mut "[[4,4],4,4]\n[[4,4],4,4,4]\n".as_bytes());
        assert!(compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }

    #[test]
    fn test_case_5() {
        let s = get_packets(&mut "[7,7,7,7]\n[7,7,7]\n".as_bytes());
        assert!(!compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }

    #[test]
    fn test_case_6() {
        let s = get_packets(&mut "[]\n[3]\n".as_bytes());
        assert!(compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }

    #[test]
    fn test_case_7() {
        let s = get_packets(&mut "[[[]]]\n[[]]\n".as_bytes());
        assert!(!compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }

    #[test]
    fn test_case_8() {
        let s = get_packets(
            &mut "[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n".as_bytes(),
        );
        assert!(!compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }

    #[test]
    fn test_case_9() {
        let s = get_packets(
            &mut "[[8,[0],[[1],[1,9,4,5],1,5,[2,3,8,8,10]]],[8],[[[10]],[[3,5,3],5,[6],[6,10,7]],[6,[4],[3,6],[6,5,6,1,3],[1,9,10]]],[[4,[8,1,10,3],[9,9]]],[6]]\n[[[8],8,[],[4]],[8,[[10,8,8],2,1,7,0],[[],10,[1,6,3],7],6],[3],[[8,10,9]],[[[],9,10,[0,7,2,10,10]],10,[]]]\n".as_bytes(),
        );
        assert!(compare_packets(Rc::clone(&s[0].0), Rc::clone(&s[0].1)));
    }
}
// [1, 2, 4, 5, 8, 10, 12, 13, 14, 17, 18, 21, 24, 25, 28, 29, 31, 32, 34, 35, 36, 37, 39, 40, 44, 47, 52, 53, 54, 56, 57, 58, 60, 62, 63, 64, 65, 67, 69, 70, 71, 72, 76, 77, 78, 81, 84, 87, 90, 91, 92, 94, 95, 99, 100, 103, 106, 108, 111, 112, 113, 115, 116, 121, 123, 124, 127, 130, 136, 138, 139, 142, 143, 146, 148, 149]
