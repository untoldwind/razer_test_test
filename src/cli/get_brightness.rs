use devices;
use errors::Result;

pub fn get_brightness() -> Result<()> {
    for device in devices::list_devices()? {
        println!("{} {:?}", device.name(), device.get_brightness());
    }

    Ok(())
}
