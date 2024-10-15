use rppal::i2c::I2c;
use std::thread::sleep;
use std::time::Duration;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    // #[cfg(target_os="unix")]
    fn i2c_led_test() {
        let mut i2c = I2c::new().unwrap();
        // ustawienie adresu i2c
        i2c.set_slave_address(0x20).unwrap();    

        // ustawienie które gpio są jako input lub output dla GPA (w tym przypdku 11111010), input = 1 output = 0
        i2c.smbus_write_byte(0x00, 0xFA).unwrap(); // GPA
        // ustawienie które gpio są jako input lub output  dla GPB (w tym przypadku 11111110), input = 1 output = 0
        i2c.smbus_write_byte(0x01, 0xFE).unwrap(); // GPB
        // ustawia "output state" czyli czy pin jest aktywny dla GPA aktywny = 1 nieaktywny = 0 (w tym przypdaku 00000101), 
        i2c.smbus_write_byte(0x14, 0x05).unwrap(); // GPA
        // ustawia "output state" czyli czy pin jest aktywny dla GPB aktywny = 1 nieaktywny = 0 (w tym przypdaku 00000001)
        i2c.smbus_write_byte(0x15, 0x01).unwrap(); // GPB
        sleep(Duration::from_secs(5));
        i2c.smbus_write_byte(0x14, 0x00).unwrap();
        i2c.smbus_write_byte(0x15, 0x00).unwrap();
        assert_eq!(0, 0)
    }
}