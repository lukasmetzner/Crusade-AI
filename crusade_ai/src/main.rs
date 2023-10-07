use std::{thread::sleep, time::Duration};

use crusade_ai_lib::Crusader;

fn main() {
    let cursader = Crusader::new("Stronghold".to_string()).unwrap();
    loop {
        println!("{}", cursader);
        sleep(Duration::from_millis(500));
    }
}
