
# `duct_reader_check`

This is a small example to demonstrate how to start an external process
with the `duct` crate using the `duct::cmd::reader` function.

When you run `cargo run` the expected output should look similar to:
```
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/duct_reader_check`
2025-02-09T14:49:33Z: INFO: Started command reader:
ReaderHandle { handle: Handle { inner: Child(ChildHandle { child: SharedChild { child: Mutex { data: Child { stdin: None, stdout: None, stderr: None, .. }, poisoned: false, .. }, state_lock: Mutex { data: NotWaiting, poisoned: false, .. }, state_condvar: Condvar { .. } }, command_string: "[\"python\", \"-u\", \"test.py\"]" }), result: OnceCell(Uninit), readers: Mutex { data: (None, None), poisoned: false, .. } }, reader: PipeReader(File { fd: 3, path: "pipe:[64785]", read: true, write: false }) }
2025-02-09T14:49:33Z: INFO: Waiting for -->hello to stdout<--
2025-02-09T14:49:35Z: INFO: Breaking from wait-start loop
2025-02-09T14:49:37Z: INFO: Breaking from wait-finish, output = Output { status: ExitStatus(unix_wait_status(0)), stdout: "", stderr: "" }
2025-02-09T14:49:37Z: INFO: Quitting time...
```

