use chip8::core::chip;
use chip8::core::registers::Register;

#[test]
fn basic_chip_test() {
    let mut chip = chip::Chip8::init();
    chip.store_register(Register::V0, 0x10);
    assert_eq!(chip.get_register(Register::V0), 0x10);

    chip.reset();

    assert_eq!(chip.get_register(Register::V0), 0x00);
}
