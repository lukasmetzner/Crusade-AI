use std::ffi::c_void;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use windows::Win32::Foundation::HANDLE;
use windows::{
    core::Error,
    Win32::System::Threading::{
        OpenProcess, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    },
};
use windows_sys::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

use rusqlite::Connection;
use std::{thread::sleep, time::Duration};

const GOLD: *const c_void = 0x0115FCF8 as _;
const WOOD: *const c_void = 0x0115FCC4 as _;
const STONE: *const c_void = 0x0115FCCC as _;
const IRON: *const c_void = 0x0115FCD4 as _;
const HOPS: *const c_void = 0x0115FCC8 as _;
const PITCH: *const c_void = 0x0115FCD8 as _;
const ALE: *const c_void = 0x0115FCF4 as _;
const FLOUR: *const c_void = 0x0115FCFC as _;

const CURRENT_PEASANTS: *const c_void = 0x00F78F18 as _;
const MAX_PEASANTS: *const c_void = 0x0115F860 as _;

const TICK: *const c_void = 0x0117CADC as _;

pub struct Crusader {
    handle: HANDLE,
}

impl std::fmt::Display for Crusader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = String::default();
        builder += "######## Peasants ########\n";
        builder += format!("Current Tick: {}\n", self.get_tick().unwrap()).as_str();
        builder += "######## Resources ########\n";
        builder += format!("Gold: {}\n", self.get_gold().unwrap()).as_str();
        builder += format!("Wood: {}\n", self.get_wood().unwrap()).as_str();
        builder += format!("Stone: {}\n", self.get_stone().unwrap()).as_str();
        builder += format!("Iron: {}\n", self.get_iron().unwrap()).as_str();
        builder += format!("Hops: {}\n", self.get_hops().unwrap()).as_str();
        builder += format!("Pitch: {}\n", self.get_pitch().unwrap()).as_str();
        builder += format!("Ale: {}\n", self.get_ale().unwrap()).as_str();
        builder += format!("Flour: {}\n", self.get_flour().unwrap()).as_str();
        builder += "######## Peasants ########\n";
        builder += format!("Peasants: {}\n", self.get_peasant().unwrap()).as_str();
        builder += format!("Max Peasants: {}\n", self.get_max_peasants().unwrap()).as_str();
        write!(f, "{}", builder)
    }
}

impl Crusader {
    pub fn new() -> Result<Crusader, Error> {
        let s = System::new_all();
        let pid = s
            .processes()
            .iter()
            .filter(|f| {
                f.1.name()
                    .to_lowercase()
                    .contains("Stronghold".to_lowercase().as_str())
            })
            .last()
            .unwrap()
            .0
            .as_u32();
        unsafe {
            let handle = OpenProcess(
                PROCESS_VM_WRITE | PROCESS_VM_OPERATION | PROCESS_VM_READ,
                false,
                pid,
            )?;
            Ok(Crusader { handle })
        }
    }

    pub fn get_tick(&self) -> Result<u32, Error> {
        self._read_memory(TICK)
    }

    pub fn get_flour(&self) -> Result<u32, Error> {
        self._read_memory(FLOUR)
    }

    pub fn get_ale(&self) -> Result<u32, Error> {
        self._read_memory(ALE)
    }

    pub fn get_pitch(&self) -> Result<u32, Error> {
        self._read_memory(PITCH)
    }

    pub fn get_hops(&self) -> Result<u32, Error> {
        self._read_memory(HOPS)
    }

    pub fn get_iron(&self) -> Result<u32, Error> {
        self._read_memory(IRON)
    }

    pub fn get_stone(&self) -> Result<u32, Error> {
        self._read_memory(STONE)
    }

    pub fn get_wood(&self) -> Result<u32, Error> {
        self._read_memory(WOOD)
    }

    pub fn get_gold(&self) -> Result<u32, Error> {
        self._read_memory(GOLD)
    }

    pub fn get_max_peasants(&self) -> Result<u32, Error> {
        self._read_memory(MAX_PEASANTS)
    }

    pub fn get_peasant(&self) -> Result<u32, Error> {
        self._read_memory(CURRENT_PEASANTS)
    }

    fn _read_memory(&self, address: *const c_void) -> Result<u32, Error> {
        let mut buffer = [u8::default(); 4];
        let mut bytes_read: usize = usize::default();
        unsafe {
            ReadProcessMemory(
                self.handle.0,
                address,
                buffer.as_mut_ptr().cast(),
                4,
                &mut bytes_read,
            );
        }
        Ok(u32::from_le_bytes(buffer))
    }

    fn _write_memory(&self, address: *const c_void, value: u32) -> Result<(), Error> {
        let mut bytes_written: usize = usize::default();
        unsafe {
            WriteProcessMemory(
                self.handle.0,
                address,
                value.to_le_bytes().as_mut_ptr().cast(),
                4,
                &mut bytes_written,
            );
        }
        Ok(())
    }
}

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

pub fn extract_data() {
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
            Err(_) => {
                continue;
            }
        }
        println!("{}", cursader);
        sleep(Duration::from_millis(2000));
    }
}
