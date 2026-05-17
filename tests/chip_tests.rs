use chip8::core::chip;

#[test]
fn basic_chip_test() {
    let mut chip = chip::Chip8::init();
    chip.reset();
}
