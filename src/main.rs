use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_sys::esp_random;

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();

    println!("Configuring output channels");

    let peripherals = Peripherals::take()?;
    let mut channel_r = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &config::TimerConfig::new().frequency(25.kHz().into()),
        )?,
        peripherals.pins.gpio2,
    )?;
    let mut channel_g = LedcDriver::new(
        peripherals.ledc.channel1,
        LedcTimerDriver::new(
            peripherals.ledc.timer1,
            &config::TimerConfig::new().frequency(25.kHz().into()),
        )?,
        peripherals.pins.gpio1,
    )?;
    let mut channel_b = LedcDriver::new(
        peripherals.ledc.channel2,
        LedcTimerDriver::new(
            peripherals.ledc.timer2,
            &config::TimerConfig::new().frequency(25.kHz().into()),
        )?,
        peripherals.pins.gpio0,
    )?;

    println!("Starting duty-cycle loop");

    let max_duty = channel_r.get_max_duty();
    loop {
        unsafe {
            let duty_r = esp_random() / (u32::MAX / max_duty);
            channel_r.set_duty(duty_r)?;
            let duty_g = esp_random() / (u32::MAX / max_duty);
            channel_g.set_duty(duty_g)?;
            let duty_b = esp_random() / (u32::MAX / max_duty);
            channel_b.set_duty(duty_b)?;
            println!("Duty = {duty_r},{duty_g},{duty_b}");
        }
        FreeRtos::delay_ms(500);
    }

    // Ok(())
}