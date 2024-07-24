#![no_std]

use dht11::Dht11;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex,
};
pub mod dht11;

type Dht11Mutex = Mutex<CriticalSectionRawMutex, Option<Dht11>>;
pub static SENSOR: Dht11Mutex = Mutex::new(None);
