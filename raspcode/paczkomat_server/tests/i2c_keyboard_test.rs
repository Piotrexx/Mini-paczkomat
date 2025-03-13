#[cfg(target_os = "linux")]
use rppal::i2c::I2c;
use std::thread::sleep;
use std::time::Duration;
#[cfg(target_os = "linux")]
use paczkomat_server::utilss::scan_keypad;
#[cfg(target_os = "linux")]
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn keyboard_test() {
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
        let mut i2c = I2c::new().unwrap();
        i2c.set_slave_address(MCP23017_ADDRESS).unwrap();
    
        // Set GPB0-GPB3 (rows) as inputs and GPB4-GPB7 (columns) as outputs
        i2c.smbus_write_byte(IODIRB, 0x0F).unwrap();  // 0x0F = 00001111, rows = input, cols = output
    
        // Enable pull-up resistors for input pins (rows) GPB0-GPB3
        i2c.smbus_write_byte(GPPUB, 0x0F).unwrap();  // 0x0F = 00001111, enable pull-ups for rows
    
        // Initially, set all columns high (inactive)
        i2c.smbus_write_byte(OLATB, 0xF0).unwrap();  // 0xF0 = 11110000
    
        // Main loop to scan the keypad
        loop {
            if let Some(key) = scan_keypad(&mut i2c) {
                println!("Key pressed: {}", key);
            }
    
            // Short delay to debounce keypresses
            sleep(Duration::from_millis(500));
        }
        assert_eq!(0, 0)

    }
}