fn main() {
    let mut handle = rusb::open_device_with_vid_pid(0x1d50, 0x6122).unwrap();

    // let device = handle.device();
    // for interface in device.active_config_descriptor().unwrap().interfaces() {
    //     for idesc in interface.descriptors() {
    //         for edesc in idesc.endpoint_descriptors() {
    //             if edesc.address() == 0x83 {
    //                 println!("{:#x?}", idesc);
    //                 println!("{:#x?}", edesc.transfer_type());
    //             }
    //         }
    //     }
    // }

    let sec = core::time::Duration::from_secs(1);

    handle.set_auto_detach_kernel_driver(true).unwrap();
    handle.claim_interface(0x01).unwrap();
    println!(
        "{:#x?}",
        handle.write_control(
            0x21,   // bmRequestType for Set_Report
            0x09,   // bRequest: SET_REPORT
            0x0200, // wValue: Output (02), report id null
            0x0001, // wIndex: interface id
            &[0b00000111],
            sec
        )
    );

    std::thread::sleep(sec);

    // println!(
    //     "{:#x?}",
    //     handle.write_interrupt(
    //         0x83,
    //         &[0, 0,0,0,1,1,1,1],
    //         core::time::Duration::from_secs(1)
    //     )
    // )
}
