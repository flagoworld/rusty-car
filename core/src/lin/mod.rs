extern crate serial;
extern crate time;

use std::env;
use std::io;

use self::time::Duration;

use std::io::prelude::*;
use self::serial::prelude::*;
use self::frame::LINFrame;

mod frame;

const MSEC_SEC: f32 = 1000f32;

pub struct LINOptions
{
    serial_port: String,
    baud_rate: f32,
    break_bytes: f32,
    frame_padding_percent: f32,
    inter_frame_space: f32
}

impl Default for LINOptions
{
    fn default() -> LINOptions
    {
        LINOptions
        {
            serial_port: "/dev/ttyAMA0".to_string(),
            baud_rate: 19200f32,
            break_bytes: 2f32,
            frame_padding_percent: 0.4f32,
            inter_frame_space: 10f32
        }
    }
}

pub struct LINMaster
{
    options: LINOptions,
    byte_time: f32,
    break_time: f32,
    frame_bytes: f32,
    frame_time: f32,
    serial: Option<serial::posix::TTYPort>,
    last_frame_data: Vec<u8>,

    current_frame: i32,
    last_frame: Option<LINFrame>,

    schedule: Vec<LINFrame>,
    schedule_event_collision: Vec<LINFrame>,
    schedule_sporadic: Vec<LINFrame>
}

impl Default for LINMaster
{
    fn default() -> LINMaster
    {
        LINMaster
        {
            options: Default::default(),
            byte_time: 0f32,
            break_time: 0f32,
            frame_bytes: 0f32,
            frame_time: 0f32,
            serial: None,
            last_frame_data: vec![],

            current_frame: 0i32,
            last_frame: None,

            schedule: vec![],
            schedule_event_collision: vec![],
            schedule_sporadic: vec![]
        }
    }
}

impl LINMaster
{
    pub fn new(options: LINOptions) -> LINMaster
    {
        let mut master: LINMaster = Default::default();

        master.byte_time = MSEC_SEC / options.baud_rate * 8f32;
        master.break_time = master.byte_time * options.break_bytes;
        master.frame_bytes = options.break_bytes + 1f32 + 8f32 + 1f32; // break + sync + data + checksum
        master.frame_time = master.byte_time * master.frame_bytes * (1f32 + options.frame_padding_percent); // spec says to give 40% padding

        master.current_frame = 0;

        master.options = options;

        return master;
    }

    pub fn start(&mut self)
    {
        let mut serial_port = serial::open(&self.options.serial_port).unwrap();
        setup_serial_port(&mut serial_port);
        self.serial = Some(serial_port);
    }
}

fn setup_serial_port<T: SerialPort>(port: &mut T) -> io::Result<()>
{
    try!(port.reconfigure(&|settings|
    {
        try!(settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.set_timeout(Duration::milliseconds(1000)));

    Ok(())
}