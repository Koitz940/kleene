mod concat;
use concat::concat_test;

mod power_concat;
use power_concat::power_concat_test;

fn main() {
    concat_test();
    power_concat_test();
}
