pub fn concat(s1: &str, s2: &str) -> String {
    s1.to_string() + s2
}

fn test1(s1: &str, s2: &str, s: &str, total: &mut i32, good: &mut i32) {
    print!("Test: concat(\"{}\", \"{}\") ", s1, s2);

    let got = concat(s1, s2);
    print!("\nexpected: \"{}\"\ngot: \"{}\"\n", s, got);

    if got == s {
        println!("Ok\n");
        *good += 1;
    } else {
        println!("Failed\n");
    }
    *total += 1;
}

pub fn concat_test() {
    println!("Testing concat:");

    let mut total = 0;
    let mut good = 0;

    test1("Hello ", "World!", "Hello World!", &mut total, &mut good);
    test1("", "Wor", "Wor", &mut total, &mut good);
    test1("", "", "", &mut total, &mut good);
    test1("     ", "Wor", "     Wor", &mut total, &mut good);
    test1("빈 칸", "Wor", "빈 칸Wor", &mut total, &mut good);

    if total == 0 {
        println!("No tests were made...");
    } else {
        println!("Correct: {}/{}, {}% accuracy", good, total, 100. * (good as f32) / (total as f32));
    }
    println!("\n\n");
}