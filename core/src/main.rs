use std::thread;
use lin::LINOptions;
use lin::LINMaster;
use lin::frame::LINFrame as LINFrame;

mod lin;
mod rest;

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

        master.start();
    });

    let rest_thread_output = rest_thread.unwrap().join().unwrap();
    let lin_thread_output = lin_thread.unwrap().join().unwrap();

    println!("REST OUTPUT: {:?}", rest_thread_output);
    println!("LIN OUTPUT: {:?}", lin_thread_output);
}