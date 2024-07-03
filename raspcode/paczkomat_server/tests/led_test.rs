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

}