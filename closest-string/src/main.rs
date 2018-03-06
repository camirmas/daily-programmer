use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = std::env::args().nth(1).unwrap();

    closest_string(&fname);
}

fn closest_string(fname: &str) -> String {
    let lines: Vec<_> = BufReader::new(File::open(fname).unwrap())
                        .lines()
                        .map(|r| r.unwrap())
                        .skip(1)
                        .collect();

    let res =
        lines
        .iter()
        .min_by_key(|s1| total_distance(s1, &lines))
        .unwrap()
        .to_string();

    res
}

fn total_distance(s: &str, lines: &Vec<String>) -> usize {
    lines
    .iter()
    .map(|s2| hamming_distance(s, s2).unwrap())
    .sum::<usize>()
}

fn hamming_distance(s1: &str, s2: &str) -> Result<usize, String> {
    if s1.len() != s2.len() {
        return Err("Undefined for sequences of unequal length".to_string());
    }
    let s1_iter = s1.chars();
    let s2_iter = s2.chars();
    let s = s1_iter.zip(s2_iter);

    let mut sum = 0;
    for (c1, c2) in s {
        if c1 != c2 {
            sum += 1;
        }
    }

    Ok(sum)
}

#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance("karolin", "kerstin"), Ok(3));
    assert_eq!(
        hamming_distance("a", "ab"),
        Err("Undefined for sequences of unequal length".to_string())
    );
}

#[test]
fn test_closest_string() {
    assert_eq!(closest_string("src/input_short.txt"), "ATTCTACAACT");
    assert_eq!(closest_string("src/input_long.txt"), "TTAACTCCCATTATATATTATTAATTTACCC");
}
