trait Temperature {
    fn get_temperature(&self) -> f64;
}

struct CelsiusSensor {
    temperature: f64
}

impl CelsiusSensor {
    fn new(temperature: f64) -> Self {
        CelsiusSensor { temperature }
    }

    fn get_temperature_celsius(&self) -> f64 {
        self.temperature
    }
}

struct CelsiusToFahrenheitAdapter {
    sensor: CelsiusSensor
}

impl Temperature for CelsiusToFahrenheitAdapter {
    fn get_temperature(&self) -> f64 {
        self.sensor.get_temperature_celsius() * 9.0 / 5.0 + 32.0
    }
}

fn main() {
    let celsius_sensor = CelsiusSensor::new(25.0);
    let adapter = CelsiusToFahrenheitAdapter {
        sensor: celsius_sensor,
    };

    println!("Temperature in Fahrenheit: {}", adapter.get_temperature());
}
