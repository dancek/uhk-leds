fn main() {
    let text = std::env::args().nth(1).unwrap();
    assert!(text.len() == 3);

    let mut command_data = [0x21, 0, 0, 0];
    command_data[1..].clone_from_slice(&text.as_bytes());

    let mut handle = rusb::open_device_with_vid_pid(0x1d50, 0x6122).unwrap();

    handle.set_auto_detach_kernel_driver(true).unwrap();
    handle.claim_interface(0x00).unwrap();

    handle
        .write_interrupt(0x02, &command_data, core::time::Duration::from_secs(1))
        .unwrap();
}
