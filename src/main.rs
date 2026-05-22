use chip8::core::emulator::Emulator;

fn main() {
    let rom_path = "test_roms/2-ibm-logo.ch8";
    let mut emulator = Emulator::init(rom_path).expect("Unable to load the emulator");
    emulator.run();
}
