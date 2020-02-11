mod chip_8;

use chip_8::CPU;

fn main() {
    // Set up render system and register input callbacks

    // Initialize the Chip8 system and load the game into the memory
    let mut cpu = CPU::new();
    cpu.load_memory(vec![0; 1024]);

    // Emulation loop

    loop {
        cpu.emulate_cycle();

        // If the draw flag is set, update the screen
        if cpu.draw_flag {
            // drawGraphics();
        }

        // Store key press state (Press and Release)
        // cpu.setKeys();
    }
}
