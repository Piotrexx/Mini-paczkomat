#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn led_test() {
        use rust_gpiozero::*;
        let mut locker = LED::new(27);        
        locker.on();
        loop {
  
        }

    }

    #[test]
    fn all_led_test() {
        use rust_gpiozero::*;
        use std::thread::sleep;
        use std::time::Duration;
        let gpios: Vec<u8> = vec![4, 27, 22];
        for gpio in gpios {
            let mut led = LED::new(gpio);
            led.on();
            sleep(Duration::from_secs(30));
            led.off()
        }
        assert_eq!(4,4)
    }

}