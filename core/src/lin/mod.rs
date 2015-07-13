extern crate serial;
extern crate time;

use std::io;

use self::time::Duration;

use std::io::prelude::*;
use self::serial::prelude::*;
use self::frame::LINFrame;
use std::sync::mpsc::channel;
use std::thread;
use std::collections::LinkedList;
use std::rc::Rc;

pub mod frame;

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
            baud_rate: /*600f32,*/ 19200f32,
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

    current_frame: usize,

    schedule: Vec<Rc<LINFrame>>,
    schedule_event_collision: Vec<Rc<LINFrame>>,
    schedule_sporadic: Vec<Rc<LINFrame>>,

    queue: LinkedList<Rc<LINFrame>>
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

            current_frame: 0,

            schedule: vec![],
            schedule_event_collision: vec![],
            schedule_sporadic: vec![],

            queue: LinkedList::new()
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

    pub fn add_frame(&mut self, frame: LINFrame)
    {
        self.schedule.push(Rc::new(frame));
    }

    pub fn start(&mut self)
    {
        let mut serial_port = serial::open(&self.options.serial_port).unwrap();
        setup_serial_port(&mut serial_port, &self.options);
        self.serial = Some(serial_port);

        let (tx, rx) = channel();
        let frame_time = self.frame_time;
        let worker = thread::spawn(move ||
        {
            loop
            {
                thread::sleep_ms(frame_time as u32);
                if tx.send(()).is_err()
                {
                    break;
                }
            }
        });

        loop
        {
            match rx.recv()
            {
                _ => self.begin_frame()
            }
        }

        worker.join();
    }

    fn begin_frame(&mut self)
    {
        if self.queue.len() > 0
        {
            self.handle_received_data();
        }

        if self.queue.len() == 0
        {
            self.queue_next_frame();
        }

        self.process_frame();
    }

    fn handle_received_data(&mut self)
    {
        let frame = self.queue.pop_front().unwrap();
        let mut data = vec![];

        self.serial.as_mut().unwrap().read(&mut data[..]);

        if frame.request_frame
        {
            if match frame.frame_type { frame::Type::EventTriggered => true, _ => false }
            {
                if self.detect_frame_collision(&data)
                {
                    let collision_frames = &frame.collision_frames;

                    for id in collision_frames
                    {
                        self.queue.push_back(Rc::new(LINFrame::new(*id, frame.frame_type, frame.request_frame, vec![], frame.handler.boxed_new())));
                    }
                }else
                {
                    frame.handler.handle_response(&data);
                }
            }else
            {
                frame.handler.handle_response(&data);
            }
        }
    }

    fn queue_next_frame(&mut self)
    {
        let frame = self.schedule[self.current_frame].clone();

        self.current_frame += 1;

        if self.current_frame >= self.schedule.len()
        {
            self.current_frame = 0;
        }

        self.queue.push_back(frame);
    }

    fn process_frame(&mut self)
    {
        let frame = self.queue.front().unwrap();

        let sync_byte: u8 = 0x55;
        let protected_identifier = (||
        {
            let mut byte = frame.id;
            byte += (((byte >> 0) & 0x01) ^ ((byte >> 1) & 0x01) ^ ((byte >> 2) & 0x01) ^ ((byte >> 4) & 0x01)) as u8;
            byte += 1u8 - (((byte >> 1) & 0x01) ^ ((byte >> 3) & 0x01) ^ ((byte >> 4) & 0x01) ^ ((byte >> 5) & 0x01)) as u8;

            byte
        })();

        let mut buf: Vec<u8> = vec![0; self.options.break_bytes as usize];

        buf.push(sync_byte);
        buf.push(protected_identifier);


        if !frame.request_frame
        {
            let data = frame.handler.response_data();
            let mut sum = protected_identifier;

            for b in data
            {
                buf.push(b);
                sum = (sum + b) % 0xff;
            }

            sum = !sum & 0xff;

            buf.push(sum);
        }

        //println!("VEC: {:?}", buf);
        self.serial.as_mut().unwrap().write(&buf[..]);
    }

    pub fn detect_frame_collision(&self, data: &Vec<u8>) -> bool
    {
        false
    }
}

fn setup_serial_port<T: SerialPort>(port: &mut T, options: &LINOptions) -> io::Result<()>
{
    try!(port.reconfigure(&|settings|
    {
        try!(settings.set_baud_rate(serial::BaudOther(options.baud_rate as usize)));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.set_timeout(Duration::milliseconds(1000)));

    Ok(())
}