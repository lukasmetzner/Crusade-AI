use std::{thread::sleep, time::Duration};

use crusader_lib::Crusader;

fn main() {
    let cursader = Crusader::new().unwrap();
    loop {
        println!("{}", cursader);
        sleep(Duration::from_millis(500));
    }
}
