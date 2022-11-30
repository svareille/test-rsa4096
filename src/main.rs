use hidapi::HidApi;

#[cfg(windows)]
const MESSAGE_HEADER : [u8; 5] = [0u8, 255, 255, 255, 255];
/*#[cfg(windows)]
const REPORT_SIZE: usize = 65;*/
#[cfg(unix)]
const MESSAGE_HEADER : [u8; 4] = [255u8, 255, 255, 255];
/*#[cfg(unix)]
const REPORT_SIZE: usize = 64;*/

const OK_DEVICE_IDS: [(u16, u16); 2] = [(0x16C0, 0x0486), (0x1d50, 0x60fc)];

fn main() {

    let api = HidApi::new().unwrap();
    let device = {
        let mut ok = None;
        for device in api.device_list() {
            if OK_DEVICE_IDS.contains(&(device.vendor_id(), device.product_id())) {
                if device.serial_number() == Some("1000000000") {
                    if device.usage_page() == 0xffab || device.interface_number() == 2 {
                        println!("Found Onlykey device at {}:{}", device.vendor_id(), device.product_id());
                        ok = Some(device.open_device(&api).unwrap());
                    }
                }
                else if device.usage_page() == 0xf1d0 || device.interface_number() == 1 {
                    println!("Found Onlykey device at {}:{}", device.vendor_id(), device.product_id());
                    ok = Some(device.open_device(&api).unwrap());
                }
            }
        }
        ok.expect("No device found")
    };

    const OKSIGN: u8 = 237;
    const RSA2: u8 = 2;

    let mut buf: Vec<u8> = MESSAGE_HEADER.into();
    buf.extend_from_slice(&[OKSIGN, RSA2, 32]);
    let payload: Vec<u8> = (1..=32).collect();
    println!("Sending payload: {:?}", payload);
    buf.extend_from_slice(&payload);
    
    let res = device.write(&buf).unwrap();
    println!("Wrote: {:?} byte(s)", res);

    // Read data from device
    for i in 1..=8 {
        let mut buf = [0u8; 64];
        let res = device.read_timeout(&mut buf, 22000).unwrap();
        println!("Read n°{}: {:?}, len = {}", i, &buf[..res], res);
    }

}
