use devices;
use errors::Result;

pub fn list_devices() -> Result<()> {
    for device in devices::list_devices()? {
        println!("{} {:?} {:?}", device.name(), device.get_product(), device.get_serial());
    }

    Ok(())
}
