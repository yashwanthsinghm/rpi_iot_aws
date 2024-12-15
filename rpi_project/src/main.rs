// uart_blocking_read.rs - Blocks while waiting for incoming serial data.

use rppal::uart::{Parity, Uart};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> ! {
    let counter = Arc::new(Mutex::new(0));
    let count = Arc::clone(&counter);

    let handle = thread::spawn(move || {
        let mut uart = Uart::new(115200, Parity::None, 8, 1).unwrap();

        // Configure read() to block until at least 1 byte is received.
        uart.set_read_mode(1, Duration::ZERO); //Duration::default()
        let cmd: [u8; 5] = [0x03, 0x50, 0x02, 0x3E, 0xC5];
        uart.write(&cmd);
        println!("mode : {}", uart.is_read_blocking());

        let mut buffer = [0u8; 1];
        let mut data = [0u8; 200];
        let mut final_data = [0u8; 200];
        let mut tags = 0;
        let mut prev_tags = 0;
        let mut checker = 0;
        let mut i = count.lock().unwrap();
        let res = uart.read(&mut buffer).is_ok();
        let mut len = 0;
        if res {
            len = buffer[0] + 2;
            data[*i] = buffer[0];
            //println!("len: {}", len);
            *i += 1;
        }
        loop {
            let res = uart.read(&mut buffer).is_ok();
            // println!("res : {} {}",res,i);
            // Fill the buffer variable with any incoming data.
            if res {
                //println!("Received byte: {:x?}", buffer);
                data[*i] = buffer[0];
                *i += 1;
                //println!("Received byte: {}", i);
            }
            if *i >= len.into() {
                tags = data[5];
                if (tags > prev_tags) && (tags != 0x81) {
                    final_data.copy_from_slice(&data);
                    checker = 0;
                    prev_tags = tags;
                } else {
                    checker += 1;
                    if (checker > 10) && (tags != 0x81) {
                        final_data.copy_from_slice(&data);
                        checker = 0;
                        prev_tags = tags;
                    } else if (checker > 10) && (tags == 0x81) {
                        final_data = [0u8; 200];
                        checker = 0;
                    }
                }
                println!("Received data byte: {:x?}", data);
                println!("Received final data byte: {:x?}", final_data);
                //thread::sleep(Duration::from_millis(100));
                uart.write(&cmd);
                println!("mode : {}", uart.is_read_blocking());
                *i = 0;
                len = 0;
                data = [0u8; 200];
                buffer = [0u8; 1];
                let res = uart.read(&mut buffer).is_ok();
                if res {
                    len = buffer[0] + 2;
                    data[*i] = buffer[0];
                    *i += 1;
                    println!("len: {}", len);
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    handle.join().unwrap();

    loop {
        println!("hello from main loop");
        //thread::sleep(Duration::from_millis(100));
    }
}
