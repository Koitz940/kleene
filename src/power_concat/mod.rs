pub fn power_concat(s1: &str, m: usize) -> String {
    let mut ret = String::new();
    ret.reserve(m * s1.len());
    for _ in 0..m {
        ret.push_str(s1);
    }

    ret
}

fn test1(s1: &str, m: usize, s: &str, total: &mut i32, good: &mut i32) {
    print!("Test: power_concat(\"{}\", {}) ", s1, m);

    let got = power_concat(s1, m);
    print!("\nexpected: \"{}\"\ngot: \"{}\"\n", s, got);

    if got == s {
        println!("Ok\n");
        *good += 1;
    } else {
        println!("Failed\n");
    }
    *total += 1;
}

pub fn power_concat_test() {
    println!("Testing concat:");

    let mut total = 0;
    let mut good = 0;

    test1("Hello ", 0, "", &mut total, &mut good);
    test1("Hello ", 10, "Hello Hello Hello Hello Hello Hello Hello Hello Hello Hello ", &mut total, &mut good);
    test1("", 0, "", &mut total, &mut good);
    test1("", 200, "", &mut total, &mut good);
    test1("빈 칸", 5, "빈 칸빈 칸빈 칸빈 칸빈 칸", &mut total, &mut good);
    test1(" ", 5, "     ", &mut total, &mut good);

    if total == 0 {
        println!("No tests were made...");
    } else {
        println!("Correct: {}/{}, {}% accuracy", good, total, 100. * (good as f32) / (total as f32));
    }
    println!("\n\n");
}