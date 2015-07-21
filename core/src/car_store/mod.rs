use std::fmt;

pub struct CClimate
{
    thermostat: u8,
    heater_on: bool,
    fan_speed: u8
}

pub struct CAudio
{
    volume: u8
}

pub struct CSensors
{
    ecu_mph: u8,
    ecu_rpm: u16,
    ecu_afratio: i16,

    temp_coolant: u16,
    temp_interior: u8,
    temp_exterior: u8,
    temp_oil: u16,

    bar_fuel: u16,
    bar_map: u16,
    bar_oil: u16,

    throttle_position: u8,

    low_coolant: bool,
    low_oil: bool,

    battery_voltage: u8
}

pub struct CStore
{
    climate: CClimate,
    audio: CAudio,
    sensors: CSensors
}

impl CStore
{
    pub fn new() -> CStore
    {
        CStore
        {
            climate: CClimate
            {
                thermostat: 60,
                heater_on: false,
                fan_speed: 0
            },

            audio: CAudio
            {
                volume: 0
            },

            sensors: CSensors
            {
                ecu_mph: 0,
                ecu_rpm: 0,
                ecu_afratio: 0,

                temp_coolant: 0,
                temp_interior: 0,
                temp_exterior: 0,
                temp_oil: 0,

                bar_fuel: 0,
                bar_map: 0,
                bar_oil: 0,

                throttle_position: 0,

                low_coolant: false,
                low_oil: false,

                battery_voltage: 0
            }
        }
    }
}

impl fmt::Debug for CStore
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "CStore: (
    climate: {:?},
    audio: {:?},
    sensors: {:?}
)", self.climate, self.audio, self.sensors)
    }
}

impl fmt::Debug for CClimate
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "CClimate: (
        thermostat: {:?},
        heater_on: {:?},
        fan_speed: {:?}
    )", self.thermostat, self.heater_on, self.fan_speed)
    }
}

impl fmt::Debug for CAudio
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "CAudio: (
        volume: {:?}
    )", self.volume)
    }
}

impl fmt::Debug for CSensors
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "CSensors: (
        ecu_mph: {:?},
        ecu_rpm: {:?},
        ecu_afratio: {:?},
        temp_coolant: {:?},
        temp_interior: {:?},
        temp_exterior: {:?},
        temo_oil: {:?},
        bar_fuel: {:?},
        bar_map: {:?},
        bar_oil: {:?},
        throttle_position: {:?},
        low_coolant: {:?},
        low_oil: {:?},
        battery_voltage: {:?}
    )", self.ecu_mph, self.ecu_rpm, self.ecu_afratio, self.temp_coolant, self.temp_interior, self.temp_exterior, self.temp_oil, self.bar_fuel, self.bar_map, self.bar_oil, self.throttle_position, self.low_coolant, self.low_oil, self.battery_voltage)
    }
}