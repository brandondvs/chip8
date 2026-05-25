use chip8::core::config::Config;
use chip8::core::emulator::Emulator;

fn main() {
    let config = Config::load().expect("Unable to load configuration file");
    let mut emulator = Emulator::init(&config).expect("Unable to load the emulator");
    emulator.run();
}
