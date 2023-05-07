#![no_main]

use libfuzzer_sys::fuzz_target;

use svgbob::to_svg_string_with_settings;
use svgbob::Settings;

fuzz_target!(|data: &[u8]| {
    let input = String::from_utf8_lossy(data);

    // Fast settings for rapid fuzzing
    let settings = Settings {
        include_backdrop: false,
        include_styles: false,
        include_defs: false,
        ..Settings::default()
    };

    let _ = to_svg_string_with_settings(&input, &settings);
});