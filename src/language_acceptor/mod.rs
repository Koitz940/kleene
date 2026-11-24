pub fn my_rg_acceptor(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let mut i = 0;

    if s.len() < 8 {
        return false;
    }

    if s[i] != 'a' {
        return false;
    }
    i += 1;

    if s[i] != 'b' {
        return false;
    }
    i += 1;

    if s[i] != 'a' && s[i] != 'b' {
        return false;
    }
    i += 1;

    for _ in 0..3 {
        if s[i] != 'a' {
            return false;
        }
        i += 1;
    }

    while i + 2 < s.len() {
        if s[i] != 'a' && s[i] != 'b' {
            return false;
        }
        i += 1;
    }

    if s[i] != 'a' {
        return false;
    }
    i += 1;

    s[i] == 'b'
}

fn test1(s: &str, expected: bool, total: &mut i32, good: &mut i32) {
    print!("Test: my_rg_acceptor(\"{}\") ", s);

    let got = my_rg_acceptor(s);
    print!("\nexpected: {}\ngot: {}\n", expected, got);

    if got == expected {
        println!("Ok\n");
        *good += 1;
    } else {
        println!("Failed\n");
    }
    *total += 1;
}

pub fn my_rg_acceptor_test() {
    println!("Testing my_rg_acceptor:");

    let mut total = 0;
    let mut good = 0;

    test1("abaaaab", false, &mut total, &mut good);
    test1("abaaaaab", true, &mut total, &mut good);
    test1("abbaaabab", true, &mut total, &mut good);
    test1("abaa aaab", false, &mut total, &mut good);
    test1("abaabaaab", false, &mut total, &mut good);
    test1("ababb aaab", false, &mut total, &mut good);
    test1("abaaaabab", true, &mut total, &mut good);
    test1("abbaaaab", true, &mut total, &mut good);
    test1("abbaaab", false, &mut total, &mut good);
    test1("ababaaaab", false, &mut total, &mut good);
    test1("", false, &mut total, &mut good);
    test1("ababaaaabAB", false, &mut total, &mut good);
    test1("abcaaaab", false, &mut total, &mut good);
    test1("hello", false, &mut total, &mut good);

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
