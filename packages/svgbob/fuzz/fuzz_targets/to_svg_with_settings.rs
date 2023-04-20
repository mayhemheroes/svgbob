#![no_main]

use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary::Arbitrary;

use svgbob::to_svg_with_settings;
use svgbob::Settings;

#[derive(Arbitrary)]
struct TestInput {
    ascii: String,
    settings: Settings,
}

fuzz_target!(|data: &[u8]| {
    let input = TestInput::arbitrary(data);
    let _ = to_svg_with_settings(&input.ascii, &input.settings);
});