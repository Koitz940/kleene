mod concat;
use concat::concat_test;

mod power_concat;
use power_concat::power_concat_test;

mod kleene_star_generator;
use kleene_star_generator::kleene_star_generator_test;

mod levenshtein_distance;
use levenshtein_distance::levenshtein_distance_test;

mod kleene_star_acceptor;
use kleene_star_acceptor::kleene_star_acceptor_test;

fn main() {
    concat_test();
    power_concat_test();
    kleene_star_generator_test();
    levenshtein_distance_test();
    kleene_star_acceptor_test();
}
