#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cyw43_pio::PioSpi;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::{Ipv4Address, Ipv4Cidr, StaticConfigV4};
use embassy_rp::{
    gpio::{AnyPin, Flex, Level, Output},
    pio::Pio,
};

use heapless::Vec;
use pico_wifi::{configure_network, WifiConfiguration};
mod tcpserver;

use temp_sensor::dht11::Dht11;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // init network
    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, pico_wifi::Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    let wifi_config = WifiConfiguration {
        wifi_ssid: WIFI_SSID,
        wifi_password: Some(WIFI_PASSWORD),
        ipv4: Some(StaticConfigV4 {
            address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 1, 6), 24),
            gateway: Some(Ipv4Address::new(192, 168, 0, 1)),
            dns_servers: Vec::new(),
        }),
        ipv6: None,
    };

    let (ctrl, stack) =
        configure_network(&spawner, pwr, spi, wifi_config).await;

    {
        let mut data = Flex::new(AnyPin::from(p.PIN_14));
        data.set_pull(embassy_rp::gpio::Pull::Up);
        *(temp_sensor::SENSOR.lock().await) = Some(Dht11 { data });
    }

    //let _ = spawner.spawn(tcpserver::listen(stack, ctrl));
    tcpserver::listen(stack, ctrl).await;
}
