use devices::{self, Color};
use errors::Result;

pub fn set_color(color: Color) -> Result<()> {
    for device in devices::list_devices()? {
        println!(
            "{} {:?}",
            device.name(),
            device.set_color(color)
        );
    }

    Ok(())
}
