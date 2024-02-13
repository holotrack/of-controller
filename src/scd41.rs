pub struct SCD41 {
    addr: u16,
    i2c: I2c<'static, I2C1, Async>,

    co2: u16,
    crc_co2: u8,

    temperature: u16,
    crc_temperature: u8,

    humidity: u16,
    crc_humidity: u8,
}
