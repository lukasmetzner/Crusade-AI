use std::{thread::sleep, time::Duration};

use crusader_lib::Crusader;
use rusqlite::Connection;

#[derive(Debug)]
struct Resources {
    tick: u32,
    gold: u32,
    wood: u32,
    stone: u32,
    iron: u32,
    hops: u32,
    pitch: u32,
    ale: u32,
    flour: u32,
    peasants: u32,
    max_peasants: u32,
}

fn create_table(conn: &Connection) -> () {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS resources (
            tick PRIMARY KEY,
            gold INTEGER NOT NULL,
            wood INTEGER NOT NULL,
            stone INTEGER NOT NULL,
            iron INTEGER NOT NULL,
            hops INTEGER NOT NULL,
            pitch INTEGER NOT NULL,
            ale INTEGER NOT NULL,
            flour INTEGER NOT NULL,
            peasants INTEGER NOT NULL,
            max_peasants INTEGER NOT NULL
        )",
        (),
    )
    .unwrap();
}

fn main() {
    let conn = Connection::open("C:\\Users\\lukas\\crusade-ai\\db.db").unwrap();
    create_table(&conn);

    let cursader = Crusader::new().unwrap();
    loop {
        let resources = Resources {
            tick: cursader.get_tick().unwrap_or_default(),
            gold: cursader.get_gold().unwrap_or_default(),
            wood: cursader.get_wood().unwrap_or_default(),
            iron: cursader.get_iron().unwrap_or_default(),
            hops: cursader.get_hops().unwrap_or_default(),
            ale: cursader.get_ale().unwrap_or_default(),
            stone: cursader.get_stone().unwrap_or_default(),
            flour: cursader.get_flour().unwrap_or_default(),
            pitch: cursader.get_pitch().unwrap_or_default(),
            peasants: cursader.get_peasant().unwrap_or_default(),
            max_peasants: cursader.get_max_peasants().unwrap_or_default(),
        };
        let result = conn.execute(
            "INSERT INTO resources (
                tick, 
                gold, 
                wood, 
                stone, 
                iron, 
                hops, 
                pitch, 
                ale, 
                flour, 
                peasants, 
                max_peasants
            ) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);",
            (
                &resources.tick,
                &resources.gold,
                &resources.wood,
                &resources.stone,
                &resources.iron,
                &resources.hops,
                &resources.pitch,
                &resources.ale,
                &resources.flour,
                &resources.peasants,
                &resources.max_peasants,
            ),
        );
        match result {
            Ok(_) => (),
            Err(e) => {
                println!("{:?}", e);
                continue;
            },
        }
        println!("{}", cursader);
        sleep(Duration::from_millis(500));
    }
}
