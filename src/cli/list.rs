use devices;
use errors::Result;

pub fn list_devices() -> Result<()> {
    for device in devices::list_devices()? {
        println!("{}", device.name());
    }

    Ok(())
}
