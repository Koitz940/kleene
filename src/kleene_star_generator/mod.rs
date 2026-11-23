use std::{iter::zip, print, println};

fn generate(sigma: &[char], m: usize, cur: &mut String, ret: &mut Vec<String>) {
    ret.push(cur.clone());

    if m != 0 {
        for &c in sigma {
            cur.push(c);
            generate(sigma, m - 1, cur, ret);
            cur.pop();
        }
    }
}

pub fn generate_kleene_star(sigma: &str, m: usize) -> Vec<String> {
    if m == 0 || sigma.is_empty() {
        return vec![String::new()];
    }

    let mut chars = Vec::new();

    for c in sigma.chars() {
        if !chars.contains(&c) {
            chars.push(c);
        }
    }

    let mut cur = String::with_capacity(m);
    let mut ret = Vec::new();

    generate(&chars, m, &mut cur, &mut ret);

    ret
}

fn test1(sigma: &str, m: usize, expected: Vec<String>, total: &mut i32, good: &mut i32) {
    let got = generate_kleene_star(sigma, m);
    *total += 1;

    println!("Test: kleene_star(\"{}\", {}) ", sigma, m);
    print!("expected: {{");
    for (i, a) in expected.iter().enumerate() {
        print!("\"{a}\"");
        if 1 + i == expected.len() {
            print!("}}");
        } else {
            print!(", ");
        }
    }
    print!("\ngot: {{");
    for (i, a) in got.iter().enumerate() {
        print!("\"{a}\"");
        if i + 1 == got.len() {
            print!("}}");
        } else {
            print!(", ");
        }
    }
    print!("\n");

    if got.len() != expected.len() {
        println!("Failed\n");
        return;
    }

    for (a, b) in zip(&got, &expected) {
        if a != b {
            println!("Failed\n");
            return;
        }
    }

    println!("Ok\n");
    *good += 1;
}

pub fn kleene_star_generator_test() {
    println!("Testing kleene_star_generator:");

    let mut total = 0;
    let mut good = 0;

    test1("", 0, vec!["".to_string()], &mut total, &mut good);
    test1("a", 0, vec!["".to_string()], &mut total, &mut good);
    test1(
        "a",
        1,
        vec!["".to_string(), "a".to_string()],
        &mut total,
        &mut good,
    );
    test1(
        "a",
        2,
        vec!["".to_string(), "a".to_string(), "aa".to_string()],
        &mut total,
        &mut good,
    );
    test1(
        "ab",
        2,
        vec![
            "".to_string(),
            "a".to_string(),
            "aa".to_string(),
            "ab".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "bb".to_string(),
        ],
        &mut total,
        &mut good,
    );
    test1(
        "ab",
        3,
        vec![
            "".to_string(),
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aab".to_string(),
            "ab".to_string(),
            "aba".to_string(),
            "abb".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "baa".to_string(),
            "bab".to_string(),
            "bb".to_string(),
            "bba".to_string(),
            "bbb".to_string(),
        ],
        &mut total,
        &mut good,
    );
    test1(
        "abc",
        1,
        vec![
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ],
        &mut total,
        &mut good,
    );

    test1(
        "abc",
        2,
        vec![
            "".to_string(),
            "a".to_string(),
            "aa".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "bb".to_string(),
            "bc".to_string(),
            "c".to_string(),
            "ca".to_string(),
            "cb".to_string(),
            "cc".to_string(),
        ],
        &mut total,
        &mut good,
    );

    if total == 0 {
        println!("No tests were made...");
    } else {
        println!(
            "Correct: {}/{}, {}% accuracy",
            good,
            total,
            100. * (good as f32) / (total as f32)
        );
    }
    println!("\n\n");
}
