#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[cfg(target_os = "linux")]
    fn led_test() {
        use rust_gpiozero::*;
        let mut locker = LED::new(27);        
        locker.on();
        loop {
  
        }

    }

    #[test]
    #[cfg(target_os = "linux")]
    fn all_led_test() {
        use rust_gpiozero::*;
        use std::thread::sleep;
        use std::time::Duration;
        let gpios: Vec<u8> = vec![27, 23, 22];
        for gpio in gpios {
            let mut led = LED::new(gpio);
            led.on();
            sleep(Duration::from_secs(30));
            led.off()
        }
        assert_eq!(4,4)
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_gpio_activity() {
        use rust_gpiozero::*;
        use std::thread::sleep;
        use std::time::Duration;
        let mut led = LED::new(27);
        led.on();
        sleep(Duration::from_secs(10));
        led.off();

        let mut led_test = LED::new(27);
        led.on();
        sleep(Duration::from_secs(10));
        led.off();
        assert_eq!(4,4)
    }


    #[test]
    #[cfg(target_os = "linux")]
    fn variable_gpio_activity() {
        use rust_gpiozero::*;
        use std::thread::sleep;
        use std::time::Duration;
        tokio::spawn(async move {
            let mut led = LED::new(27);
            led.on();
            loop {
                
            }
          });
        let led = LED::new(23);
        led.on();
        sleep(Duration::from_secs(10));
        led.off();

        assert_eq!(4,4)

    }

    #[test]
    #[cfg(target_os = "linux")]
    fn loop_test() {
        use rust_gpiozero::*;
        use std::thread::sleep;
        use std::time::Duration;
        let mut  number = 10;
        let led = LED::new(27);
        number = number - 1;
        
        led.on();
        loop {
            number = number - 1;
            sleep(Duration::from_secs(10));
            if number == 5 { 
                led.off();
                break;
        }            
    }    
            assert_eq!(4,4);        
    }

}


