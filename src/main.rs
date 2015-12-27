extern crate libusb;
//extern crate rand;

use std::time::Duration;
//use std::thread::sleep_ms;

use libusb::{Context, Device, DeviceHandle};
//use rand::{Rng, thread_rng};

fn main() {
    let mut context = Context::new().unwrap();

    let vendor_id = 0x24c6;
    let product_id = 0x542a;
    let mut device = find_device(&mut context, vendor_id, product_id).unwrap();

    let mut handle = device.open().unwrap();

    let config = device.config_descriptor(0).unwrap();

    /*for interface in config.interfaces() {
        println!("Found interface: {:02x}", interface.number());
        for desc in interface.descriptors() {
            println!("  Found descriptor:");
            println!("    Endpoints ({}):", desc.num_endpoints());
            for endpoint in desc.endpoint_descriptors() {
                println!("      Address: {:02x}", endpoint.address());
                println!("      Direction: {:?}", endpoint.direction());
                println!("      Transfer type: {:?}", endpoint.transfer_type());
            }
        }
    }*/

    if handle.kernel_driver_active(config.number()).unwrap() {
        println!("Kernel driver active");
        return;
    } else {
        println!("No kernel driver active");
    }

    let interface = 0x00;
    handle.claim_interface(interface).ok();

    let timeout = Duration::from_secs(10);

    /*let startup_message = */read_buffer(&mut handle, timeout);

    /*write_buffer(&mut handle, &[0x04, 0x20, 0x01, 0x00], timeout);
    read_buffer(&mut handle, timeout);
    write_buffer(&mut handle, &[0x01, 0x20, 0x01, 0x09, 0x00, 0x04, 0x20, 0x3a, 0x00, 0x00, 0x00, 0x83, 0x00], timeout);
    read_buffer(&mut handle, timeout);
    write_buffer(&mut handle, &[0x01, 0x20, 0x01, 0x09, 0x00, 0x04, 0x20, 0xbd, 0x00, 0x00, 0x00, 0x00, 0x00], timeout);
    read_buffer(&mut handle, timeout);*/

    write_buffer(&mut handle, &[0x05, 0x20, 0x02, 0x09, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0x53], timeout);
    write_buffer(&mut handle, &[0x05, 0x20, 0x03, 0x01, 0x00], timeout);
    write_buffer(&mut handle, &[0x0a, 0x20, 0x04, 0x03, 0x00, 0x01, 0x14], timeout);
    read_buffer(&mut handle, timeout);

    /*loop {
        read_buffer(&mut handle, timeout);
    }*/

    /*loop {
        let mut write_buf = [0xff; 32];

        let mut write_command = 0x083d;
        write_buf[0] = (write_command >> 8) as u8;
        write_buf[1] = write_command as u8;
        write_buf[2] = 0x00;

        write_buffer(&mut handle, &write_buf, timeout);

        sleep_ms(100);

        read_buffer(&mut handle, timeout);
    }*/

    /*let mut write_buf = [0x00; 32];
    for i in 2..32 {
        write_buf[i] = startup_message[i];
    }

    let mut write_command = 0x083d;
    write_buf[0] = (write_command >> 8) as u8;
    write_buf[1] = write_command as u8;

    write_buffer(&mut handle, &write_buf, timeout);

    loop {
        let read_buf = read_buffer(&mut handle, timeout);

        sleep_ms(100);

        let mut write_buf = [0xff; 32];
        write_buf[0] = 0x08;
        write_buf[1] = 0x3d;
        write_buf[2] = 0x00;
        write_buffer(&mut handle, &write_buf, timeout);
    }*/
}

fn find_device<'a>(context: &'a mut Context, vendor_id: u16, product_id: u16) -> Option<Device<'a>> {
    for mut device in context.devices().unwrap().iter() {
        let desc = device.device_descriptor().unwrap();

        if desc.vendor_id() == vendor_id && desc.product_id() == product_id {
            return Some(device);
        }
    }

    None
}

fn read_buffer(handle: &mut DeviceHandle, timeout: Duration) -> [u8; 256] {
    let read_endpoint = 0x81;

    let mut buf = [0; 256];
    let bytes_read = handle.read_interrupt(read_endpoint, &mut buf, timeout).unwrap();
    print!("Bytes read    ({:02}): ", bytes_read);
    print_buf(&buf, bytes_read);

    buf
}

fn print_buf(buf: &[u8], num_bytes: usize) {
    for i in 0..num_bytes {
        print!("{:02x} ", buf[i]);
    }
    println!("");
}

fn write_buffer(handle: &mut DeviceHandle, buf: &[u8], timeout: Duration) {
    let write_endpoint = 0x01;

    print!("Writing bytes ({:02}): ", buf.len());
    print_buf(&buf, buf.len());
    handle.write_interrupt(write_endpoint, &buf, timeout).unwrap();
}
