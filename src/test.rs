use rapid_fuzz::RapidFuzzBuilder;

pub fn main() {
    let builder = RapidFuzzBuilder::new()
        .without_pre_suite()
        .without_pre_test()
        .without_post_test()
        .without_post_suite();
    builder.run();
}
