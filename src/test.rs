use crate::builder::RapidFuzzBuilder;

#[test]
pub fn test() {
    let builder = RapidFuzzBuilder::new()
        .without_pre_suite()
        .without_pre_test()
        .without_post_test()
        .without_post_suite();
    builder.run();
}
