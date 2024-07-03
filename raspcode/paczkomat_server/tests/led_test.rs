#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn led_test() {
        use rust_gpiozero::*;
        // use tokio;
        let mut locker = LED::new(4);
        
        loop {
            locker.on()    
        }
        // tokio::spawn(async move {
        //     locker.on();
        //     loop {
        //         // Do nothing here (or add minimal logic)
        //         tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        //     }
        // });
    }

}