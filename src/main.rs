mod concat;
use concat::concat_test;

mod power_concat;
use power_concat::power_concat_test;

mod kleene_star_generator;
use kleene_star_generator::kleene_star_generator_test;

mod levenshtein_distance;

fn main() {
    concat_test();
    power_concat_test();
    kleene_star_generator_test();
}
