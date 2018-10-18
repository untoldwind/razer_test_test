use devices;
use errors::Result;

pub fn set_brightness(brightness: u8) -> Result<()> {
    for device in devices::list_devices()? {
        println!("{} {:?}", device.name(), device.set_brightness(brightness));
    }

    Ok(())
}
