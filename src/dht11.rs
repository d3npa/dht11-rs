use embassy_rp::gpio::{AnyPin, Flex};
use embassy_time::{Instant, Timer};

use core::convert::From;

macro_rules! u8_from_bits {
    ($bits:expr) => {{
        let mut result = 0;
        for (index, &bit) in $bits.iter().rev().enumerate() {
            if bit == 1 {
                result |= 1 << index;
            }
        }
        result
    }};
}

pub struct Dht11 {
    pub data: Flex<'static, AnyPin>,
}

impl Dht11 {
    pub async fn send_start(&mut self) {
        self.data.set_as_output();

        self.data.set_high();
        Timer::after_millis(80).await;

        self.data.set_low();
        Timer::after_millis(20).await;

        self.data.set_high();
        Timer::after_micros(20).await; // dht11 should respond within 20-40us
    }

    pub async fn read_response(&mut self) -> Dht11Packet {
        self.data.set_as_input();

        while self.data.is_high() {
            continue; // blocking here. maybe not great practice
        }

        while self.data.is_low() {
            continue;
        }

        while self.data.is_high() {
            continue;
        }

        // start signal is over. read data now

        let mut buf = [255u8; 40];
        for i in 0..40 {
            // low for 50us
            while self.data.is_low() {
                continue;
            }

            let start = Instant::now();

            while self.data.is_high() {
                continue;
            }

            let end = Instant::now();

            let difference = end.as_micros() - start.as_micros();
            if difference < 30 {
                buf[i] = 0;
            } else if difference < 80 {
                buf[i] = 1;
            }
        }

        return Dht11Packet::from(buf);
    }
}

#[derive(Copy, Clone, Default)]
pub struct Dht11Packet {
    pub humidity_integral: u8,
    pub humidity_decimal: u8,
    pub temperature_integral: u8,
    pub temperature_decimal: u8,
    pub checksum: u8,
}

impl From<[u8; 40]> for Dht11Packet {
    fn from(value: [u8; 40]) -> Self {
        Self {
            humidity_integral: u8_from_bits!(value[0..8]),
            humidity_decimal: u8_from_bits!(value[8..16]),
            temperature_integral: u8_from_bits!(value[16..24]),
            temperature_decimal: u8_from_bits!(value[24..32]),
            checksum: u8_from_bits!(value[32..40]),
        }
    }
}
