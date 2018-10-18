use devices;
use errors::Result;

pub fn set_color() -> Result<()> {
    for device in devices::list_devices()? {
        println!(
            "{} {:?}",
            device.name(),
            device.set_color(devices::Color::new(255, 255, 255))
        );
    }

    Ok(())
}
