use std::thread;
use lin::LINOptions;
use lin::LINMaster;
use lin::frame;
use lin::frame::LINFrame;
use handlers::zero::Zero;

mod lin;
mod rest;
mod handlers;

fn main()
{
    let rest_thread = thread::Builder::new().name("rusty-car-rest".to_string()).spawn(move ||
    {
        loop
        {
            println!("REST");
            thread::sleep_ms(5000);
        }
    });

    let lin_thread = thread::Builder::new().name("rusty-car-lin".to_string()).spawn(move ||
    {
        let options: LINOptions = Default::default();
        let mut master = LINMaster::new(options);
        let handlers = load_handlers();

        load_frames(&mut master, &handlers);

        master.start();
    });

    let rest_thread_output = rest_thread.unwrap().join().unwrap();
    let lin_thread_output = lin_thread.unwrap().join().unwrap();

    println!("REST OUTPUT: {:?}", rest_thread_output);
    println!("LIN OUTPUT: {:?}", lin_thread_output);
}

fn load_handlers<T: frame::LINFrameHandler>() -> Vec<T>
{
    let handlers = vec!
    [
        Zero::new()
    ];

    handlers
}

fn load_frames<T: frame::LINFrameHandler>(master: &mut LINMaster, handlers: &Vec<T>)
{
    for handler in handlers
    {
        let frame = LINFrame::new(0, frame::Type::Unconditional, true, vec![], &handler);
        master.add_frame(frame);
    }
}