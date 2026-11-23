use std::collections::HashSet;

pub fn kleene_star_acceptor(sigma: &str, s: &str) -> bool {
    let set: HashSet<char> = sigma.chars().collect();
    for c in s.chars() {
        if !set.contains(&c) {
            return false;
        }
    }

    true
}

fn test1(sigma: &str, s: &str, expected: bool, total: &mut i32, good: &mut i32) {
    let got = kleene_star_acceptor(sigma, s);

    println!("Test: kleene_star_acceptor(\"{}\", \"{}\")", sigma, s);
    print!("\nexpected: {}\ngot: {}\n", expected, got);
    *total += 1;
    if expected == got {
        *good += 1;
        println!("Ok\n");
    } else {
        println!("Failed\n");
    }
}

pub fn kleene_star_acceptor_test() {
    println!("Testing kleene_star_acceptor:");

    let mut total = 0;
    let mut good = 0;

    test1("abc", "abc", true, &mut total, &mut good);
    test1("abc", "a", true, &mut total, &mut good);
    test1("abc", "bca", true, &mut total, &mut good);
    test1("abc", "abccba", true, &mut total, &mut good);
    test1("abc", "", true, &mut total, &mut good);
    test1("abc", "aaa", true, &mut total, &mut good);

    test1("abc", "d", false, &mut total, &mut good);
    test1("abc", "abcd", false, &mut total, &mut good);
    test1("abc", "abdc", false, &mut total, &mut good);
    test1("abc", "xyz", false, &mut total, &mut good);
    test1("abc", "a b", false, &mut total, &mut good);
    test1("abc", "A", false, &mut total, &mut good);

    test1("0123456789", "1234567890", true, &mut total, &mut good);
    test1("0123456789", "123abc", false, &mut total, &mut good);
    test1("aeiou", "aeiou", true, &mut total, &mut good);
    test1("aeiou", "hello", false, &mut total, &mut good);
    test1(
        "abcdefghijklmnopqrstuvwxyz",
        "hello",
        true,
        &mut total,
        &mut good,
    );
    test1(
        "abcdefghijklmnopqrstuvwxyz",
        "Hello",
        false,
        &mut total,
        &mut good,
    );

    test1("!@#$%", "!@#%", true, &mut total, &mut good);
    test1("!@#$%", "!@#?", false, &mut total, &mut good);
    test1("abc", "123", false, &mut total, &mut good);
    test1("a", "aaaaaa", true, &mut total, &mut good);
    test1("", "abc", false, &mut total, &mut good);
    test1("", "", true, &mut total, &mut good);
    test1("hello world!", "hl", true, &mut total, &mut good);
    test1("hello world!", "ec", false, &mut total, &mut good);

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
