use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals};
use log::info;
pub fn main() {
    let peripherals = Peripherals::take().unwrap();
    let pin = PinDriver::input(peripherals.pins.gpio33).unwrap();
    loop {
        if pin.is_low() {
            info!("obstacle found");
        }
        FreeRtos::delay_ms(100u32);
    }
}
