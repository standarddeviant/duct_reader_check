use duct::cmd;
use log::{debug, error, info};
use std::io::Read;

fn main() {
    env_logger::init();
    let mut buf: Vec<u8> = vec![];
    let mut single_byte = [0u8];

    let mut reader = cmd!("python", "test.py").reader().unwrap();
    info!("Started command reader:\n{reader:?}");

    loop {
        match reader.read(&mut single_byte) {
            Ok(_good) => {
                buf.push(single_byte[0]);
                if let Ok(chks) = std::str::from_utf8(&buf) {
                    debug!("chks = {chks}");
                    if chks.contains("hello to stdout") {
                        info!("Breaking from wait-start loop");
                        break;
                    }
                }
            }
            Err(_bad) => {
                error!("{_bad}");
            }
        }
    }

    loop {
        match reader.try_wait() {
            Ok(Some(_good)) => {
                info!("Breaking from wait-finish, output = {_good:?}");
                break;
            }
            Ok(None) => {
                // NOTE: still running, nothing to do but sleep
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            Err(_bad) => {
                error!("Error in wait-finish: {_bad}");
                break;
            }
        }
    }
    //     match reader.read(&mut single_byte) {
    //         Ok(_good) => {
    //             buf.push(single_byte[0]);
    //             if let Ok(chks) = std::str::from_utf8(&buf) {
    //                 debug!("chks = {chks}");
    //                 if chks.contains("hello to stdout") {
    //                     info!("Breaking from initial loop");
    //                     break;
    //                 }
    //             }
    //         }
    //         Err(_bad) => {
    //             error!("{_bad}");
    //         }
    //     }
    // }

    info!("Quitting time...")
}
