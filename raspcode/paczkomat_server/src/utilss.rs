#[cfg(target_os = "linux")]
use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
const KEYPAD: [[char; 4]; 4] = [
    ['1', '2', '3', 'A'],
    ['4', '5', '6', 'B'],
    ['7', '8', '9', 'C'],
    ['*', '0', '#', 'D'],
];

const MCP23017_ADDRESS: u16 = 0x20; // I2C address of the MCP23017
const IODIRB: u8 = 0x01;    // IODIRB register: sets GPB0-GPB7 as input/output
const GPPUB: u8 = 0x0D;     // GPPUB register: enables pull-up resistors on GPB
const GPIOB: u8 = 0x13;     // GPIOB register: reads inputs from GPB
const OLATB: u8 = 0x15;

#[cfg(target_os = "linux")]
pub fn scan_keypad(i2c: &mut I2c) -> Option<char> {
    for col in 0..4 {
        // Set the current column low (active) and others high (inactive)
        let col_mask = !(1 << (col + 4)) & 0xF0; // Only affect GPB4-GPB7
        if i2c.smbus_write_byte(OLATB, col_mask).is_err() {
            eprintln!("Failed to write column data to MCP23017");
            return None;
        }

        // Read the state of the rows (GPB0-GPB3)
        let row_state = match i2c.smbus_read_byte(GPIOB) {
            Ok(state) => state & 0x0F, // Mask to keep only lower 4 bits (rows)
            Err(_) => {
                eprintln!("Failed to read row data from MCP23017");
                return None;
            }
        };

        // Check if any row is low (key pressed)
        for row in 0..4 {
            if row_state & (1 << row) == 0 {
                // Key detected at row `row` and column `col`
                return Some(KEYPAD[row][col]);
            }
        }
    }

    // Reset columns to high after scanning
    if i2c.smbus_write_byte(OLATB, 0xF0).is_err() {
        eprintln!("Failed to reset columns on MCP23017");
    }

    None // No key detected
}