pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();

    let mut d = vec![vec![0; c2.len() + 1]; c1.len() + 1];

    for i in 0..=c1.len() {
        d[i][0] = i;
    }

    for j in 0..=c2.len() {
        d[0][j] = j;
    }

    for i in 1..=c1.len() {
        for j in 1..=c2.len() {
            let cost = usize::from(c1[i - 1] != c2[j - 1]);

            d[i][j] = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);
        }
    }

    d[c1.len()][c2.len()]
}

fn test1(s1: &str, s2: &str, expected: usize, total: &mut i32, good: &mut i32) {
    let got = levenshtein_distance(s1, s2);

    println!("Test: levenshtein_distance(\"{}\", \"{}\")", s1, s2);
    print!("\nexpected: {}\ngot: {}\n", expected, got);
    *total += 1;
    if expected == got {
        *good += 1;
        println!("Ok\n");
    } else {
        println!("Failed\n");
    }
}

pub fn levenshtein_distance_test() {
    println!("Testing levenshtein_distance:");

    let mut total = 0;
    let mut good = 0;

    test1("kitten", "sitting", 3, &mut total, &mut good);
    test1("cat", "cat", 0, &mut total, &mut good);
    test1("cat", "bat", 1, &mut total, &mut good);
    test1("cat", "car", 1, &mut total, &mut good);
    test1("cat", "dog", 3, &mut total, &mut good);
    test1("hello", "hallo", 1, &mut total, &mut good);
    test1("hello", "hell", 1, &mut total, &mut good);
    test1("hello", "helloo", 1, &mut total, &mut good);
    test1("book", "back", 2, &mut total, &mut good);
    test1("kitten", "sitting", 3, &mut total, &mut good);
    test1("saturday", "sunday", 3, &mut total, &mut good);
    test1("flaw", "lawn", 2, &mut total, &mut good);
    test1("gumbo", "gambol", 2, &mut total, &mut good);
    test1("abc", "xyz", 3, &mut total, &mut good);
    test1("abc", "abcd", 1, &mut total, &mut good);
    test1("abcd", "abc", 1, &mut total, &mut good);
    test1("abc", "acb", 2, &mut total, &mut good);
    test1("abcdef", "abcfde", 2, &mut total, &mut good);
    test1("intention", "execution", 5, &mut total, &mut good);
    test1("distance", "difference", 5, &mut total, &mut good);
    test1("levenshtein", "frankenstein", 6, &mut total, &mut good);
    test1("a", "b", 1, &mut total, &mut good);
    test1("a", "aa", 1, &mut total, &mut good);
    test1("aa", "a", 1, &mut total, &mut good);
    test1("abc", "a", 2, &mut total, &mut good);
    test1("a", "abc", 2, &mut total, &mut good);
    test1("abcd", "wxyz", 4, &mut total, &mut good);
    test1("aaaa", "bbbb", 4, &mut total, &mut good);
    test1("aaaa", "aaa", 1, &mut total, &mut good);
    test1("aaa", "aaaaaa", 3, &mut total, &mut good);
    test1("mississippi", "misisipi", 3, &mut total, &mut good);
    test1("programming", "programer", 4, &mut total, &mut good);
    test1("rust", "trust", 1, &mut total, &mut good);
    test1("developer", "development", 3, &mut total, &mut good);
    test1("algorithm", "logarithm", 3, &mut total, &mut good);
    test1("abcdef", "uvwxyz", 6, &mut total, &mut good);
    test1("one two", "one  two", 1, &mut total, &mut good);
    test1("hello world", "hello", 6, &mut total, &mut good);
    test1("12345", "12345", 0, &mut total, &mut good);
    test1("12345", "123", 2, &mut total, &mut good);
    test1("ABC", "abc", 3, &mut total, &mut good);
    test1(
        "The quick brown fox",
        "The quick fox",
        6,
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
