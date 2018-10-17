use devices;
use errors::Result;

pub fn set_color() -> Result<()> {
    for device in devices::list_devices()? {
        println!("{} {:?}", device.name(), device.set_color(devices::Color::new(0, 0, 255)));
    }

    Ok(())
}
