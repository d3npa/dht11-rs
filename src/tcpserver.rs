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
    // let mut buf = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(2)));

        ctrl.gpio_set(0, false).await;

        if socket.accept(1234).await.is_err() {
            continue;
        }

        ctrl.gpio_set(0, true).await;

        // if socket.write_all(b"[*] hello~\n").await.is_err() {
        //     continue;
        // }

        // if socket.flush().await.is_err() {
        //     continue;
        // }

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

            // let mut buf_ascii = [0xa; 41];
            // for (i, &b) in buf.iter().enumerate() {
            //     let ascii = {
            //         if b == 0 {
            //             0x30
            //         } else if b == 1 {
            //             0x31
            //         } else {
            //             0xff
            //         }
            //     };
            //     buf_ascii[i] = ascii;
            // }

            if socket.write_all(&buf).await.is_err() {
                continue;
            }

            if socket.flush().await.is_err() {
                continue;
            }
        }

        // loop {
        //     let n = match socket.read(&mut buf).await {
        //         Ok(0) => break, // eof
        //         Ok(n) => n,
        //         Err(_e) => break,
        //     };

        //     buf[n] = 0;

        //     // if let Ok(string) = crate::null_term_string(&buf) {
        //     //     if string.is_empty() {
        //     //         continue;
        //     //     }

        //     //     let status = handle_command(string.trim()).await;
        //     //     if socket.write_all(&status.mesg).await.is_err() {
        //     //         break;
        //     //     }
        //     // }
        // }
    }
}
