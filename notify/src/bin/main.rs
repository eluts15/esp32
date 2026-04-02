#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Pull},
    main,
};
use esp_println::println;

/// The door state, derived from the reed switch GPIO level.
///
/// Wiring logic (NC reed switch, external 10K pull-up):
///   - Door CLOSED → switch contact closed → GPIO pulled LOW
///   - Door OPEN   → switch contact open   → GPIO pulled HIGH via resistor
#[derive(Clone, Copy, PartialEq)]
enum DoorState {
    Open,
    Closed,
}

impl DoorState {
    /// Read the current door state from the GPIO pin level.
    fn from_level(level: Level) -> Self {
        match level {
            Level::High => DoorState::Open,
            Level::Low => DoorState::Closed,
        }
    }

    /// Human-readable label for logging.
    fn as_str(&self) -> &'static str {
        match self {
            DoorState::Open => "Door opened",
            DoorState::Closed => "Door closed",
        }
    }
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // GPIO18 configured as input, no internal pull resistor.
    // Pull-up is handled externally by the 10K resistor to 3V3.
    let config = InputConfig::default().with_pull(Pull::None);
    let reed_pin = Input::new(peripherals.GPIO18, config);

    let delay = Delay::new();

    // Read and log the initial door state on boot.
    let mut last_state = DoorState::from_level(reed_pin.level());
    println!("[boot] {}", last_state.as_str());

    loop {
        let current_state = DoorState::from_level(reed_pin.level());

        if current_state != last_state {
            println!("{}", current_state.as_str());
            last_state = current_state;
        }

        // Poll every 50ms. Debounce will be added in the next iteration.
        delay.delay_millis(50);
    }
}
