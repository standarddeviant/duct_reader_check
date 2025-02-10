use duct::cmd;
use log::{debug, error, info};
use std::io::Read;

use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

fn main() {
    let mut builder = Builder::new();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: {}: {}",
                buf.timestamp(),
                record.level(),
                record.args()
            )
        })
        .filter_level(LevelFilter::Debug)
        .init();

    let mut buf: Vec<u8> = vec![];
    let mut single_byte = [0u8];

    let mut fake_stdin_wr = std::fs::File::create("./fake_stdin").unwrap();
    let fake_stdin_rd = std::fs::File::open("./fake_stdin").unwrap();
    let mut reader = cmd!("python3", "-u", "test.py")
        .stdin_file(fake_stdin_rd)
        .stderr_to_stdout()
        .reader()
        .unwrap();

    info!("Started command reader:\n{reader:?}");

    let wait_until_str = "hello to stdout";

    info!("Waiting for -->{wait_until_str}<--");
    let mut idle_count = 0;
    loop {
        match reader.read(&mut single_byte) {
            Ok(_good) => {
                if _good > 0 {
                    buf.push(single_byte[0]);
                    if let Ok(chks) = std::str::from_utf8(&buf) {
                        debug!("chks = {chks}");
                        if chks.contains(wait_until_str) {
                            info!("Breaking from wait-start loop");
                            break;
                        }
                    }
                }
                // if _good == 0 {
                //     std::thread::sleep(std::time::Duration::from_millis(20));
                //     idle_count += 1;
                //     if idle_count > 100 {
                //         break;
                //         //
                //     }
                // }
            }
            Err(_bad) => {
                error!("{_bad}");
            }
        }
    }

    info!("sleeping 2s on rust side (fake delay)");
    std::thread::sleep(std::time::Duration::from_secs(2));

    let tmpv = vec![0];
    match fake_stdin_wr.write(tmpv.as_slice()) {
        Ok(_good) => {
            info!("Wrote {_good} bytes to child process");
        }
        Err(_bad) => {
            info!("_bad = {_bad}");
        }
    }
    fake_stdin_wr.flush();

    info!("starting loop");
    loop {
        match reader.read(&mut single_byte) {
            Ok(num_bytes) => {
                if num_bytes > 0 {
                    buf.push(single_byte[0]);
                    continue;
                }
            }
            Err(_bad) => {
                error!("hmmm... {_bad}");
            }
        }
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

    if let Ok(chks) = std::str::from_utf8(&buf) {
        info!("final state of (stdout) buf = \n{chks}");
    }

    info!("Quitting time...")
}
