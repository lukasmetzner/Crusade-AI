use std::thread::sleep;
use std::{ffi::c_void, time::Duration};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use windows::Win32::Foundation::HANDLE;
use windows::{
    core::Error,
    Win32::System::Threading::{
        OpenProcess, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    },
};
use windows_sys::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

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

pub struct Crusader {
    handle: HANDLE,
}

impl std::fmt::Display for Crusader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = String::default();
        builder += "######## Resources ########\n";
        builder += format!("Gold: {}\n", self.get_current_gold_amount().unwrap()).as_str();
        builder += format!("Wood: {}\n", self.get_current_wood_amount().unwrap()).as_str();
        builder += format!("Stone: {}\n", self.get_current_stone_amount().unwrap()).as_str();
        builder += format!("Iron: {}\n", self.get_current_iron_amount().unwrap()).as_str();
        builder += format!("Hops: {}\n", self.get_current_hops_amount().unwrap()).as_str();
        builder += format!("Pitch: {}\n", self.get_current_pitch_amount().unwrap()).as_str();
        builder += format!("Ale: {}\n", self.get_current_ale_amount().unwrap()).as_str();
        builder += format!("Flour: {}\n", self.get_current_flour_amount().unwrap()).as_str();
        builder += "######## Peasants ########\n";
        builder += format!("Peasants: {}\n", self.get_current_peasant_amount().unwrap()).as_str();
        builder += format!("Max Peasants: {}\n", self.get_max_peasant_amount().unwrap()).as_str();
        write!(f, "{}", builder)
    }
}

impl Crusader {
    pub fn new(process_name: String) -> Result<Crusader, Error> {
        let s = System::new_all();
        let pid = s
            .processes()
            .iter()
            .filter(|f| {
                f.1.name()
                    .to_lowercase()
                    .contains(process_name.to_lowercase().as_str())
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

    pub fn get_current_flour_amount(&self) -> Result<u32, Error> {
        self.read_memory(FLOUR)
    }

    pub fn set_current_flour_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(FLOUR, amount)
    }

    pub fn get_current_ale_amount(&self) -> Result<u32, Error> {
        self.read_memory(ALE)
    }

    pub fn set_current_ale_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(ALE, amount)
    }

    pub fn get_current_pitch_amount(&self) -> Result<u32, Error> {
        self.read_memory(PITCH)
    }

    pub fn set_current_pitch_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(PITCH, amount)
    }

    pub fn get_current_hops_amount(&self) -> Result<u32, Error> {
        self.read_memory(HOPS)
    }

    pub fn set_current_hops_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(HOPS, amount)
    }

    pub fn get_current_iron_amount(&self) -> Result<u32, Error> {
        self.read_memory(IRON)
    }

    pub fn set_current_iron_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(IRON, amount)
    }

    pub fn get_current_stone_amount(&self) -> Result<u32, Error> {
        self.read_memory(STONE)
    }

    pub fn set_current_stone_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(STONE, amount)
    }

    pub fn get_current_wood_amount(&self) -> Result<u32, Error> {
        self.read_memory(WOOD)
    }

    pub fn set_current_wood_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(WOOD, amount)
    }

    pub fn get_current_gold_amount(&self) -> Result<u32, Error> {
        self.read_memory(GOLD)
    }

    pub fn set_current_gold_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(GOLD, amount)
    }

    pub fn get_max_peasant_amount(&self) -> Result<u32, Error> {
        self.read_memory(MAX_PEASANTS)
    }

    pub fn set_max_peasant_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(MAX_PEASANTS, amount)
    }

    pub fn get_current_peasant_amount(&self) -> Result<u32, Error> {
        self.read_memory(CURRENT_PEASANTS)
    }

    pub fn set_current_peasant_amount(&self, amount: u32) -> Result<(), Error> {
        self.write_memory(CURRENT_PEASANTS, amount)
    }

    fn read_memory(&self, address: *const c_void) -> Result<u32, Error> {
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

    fn write_memory(&self, address: *const c_void, value: u32) -> Result<(), Error> {
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

fn main() {
    let cursader = Crusader::new("Stronghold".to_string()).unwrap();
    loop {
        println!("{}", cursader);
        sleep(Duration::from_millis(500));
    }
}
