use cyw43::{Control, NetDriver};
use embassy_net::{tcp::TcpSocket, Stack};
use embassy_time::Duration;
use embedded_io_async::Write;

pub async fn listen(
    stack: &'static Stack<NetDriver<'static>>,
    mut ctrl: Control<'static>,
) -> ! {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(2)));

        ctrl.gpio_set(0, false).await;

        if socket.accept(1234).await.is_err() {
            continue;
        }

        ctrl.gpio_set(0, true).await;

        if let Some(sensor) = temp_sensor::SENSOR.lock().await.as_mut() {
            sensor.send_start().await;
            let packet = sensor.read_response().await;
            let buf = [
                packet.humidity_integral,
                packet.humidity_decimal,
                packet.temperature_integral,
                packet.temperature_decimal,
                packet.checksum,
            ];

            if socket.write_all(&buf).await.is_err() {
                continue;
            }

            if socket.flush().await.is_err() {
                continue;
            }
        }
    }
}
