#![no_main]

use libfuzzer_sys::fuzz_target;

use svgbob::to_svg_string_compressed;

fuzz_target!(|data: &[u8]| {
    let input = String::from_utf8_lossy(data);
    let _ = to_svg_string_compressed(&input);
});