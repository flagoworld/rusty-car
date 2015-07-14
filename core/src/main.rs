extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate serial;
extern crate time;

use std::thread;
use lin::LINOptions;
use lin::LINMaster;
use lin::frame;
use lin::frame::LINFrame;
use lin::frame::handlers::zero::Zero;

use iron::Iron;
use router::Router;

mod lin;
mod rest;

fn main()
{
    let rest_thread = thread::Builder::new().name("rusty-car-rest".to_string()).spawn(move ||
    {
        let mut router = Router::new();

        setup_router(&mut router);

        Iron::new(router).http("localhost:3000").unwrap();

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

        load_frames(&mut master);

        master.start();
    });

    let rest_thread_output = rest_thread.unwrap().join().unwrap();
    let lin_thread_output = lin_thread.unwrap().join().unwrap();

    println!("REST OUTPUT: {:?}", rest_thread_output);
    println!("LIN OUTPUT: {:?}", lin_thread_output);
}

fn load_frames(master: &mut LINMaster)
{
    master.add_frame(LINFrame::new(0, frame::Type::Unconditional, true, vec![], Box::new(Zero::new())));
}

fn setup_router(router: &mut router::Router)
{
    for (method, action, handler) in rest::get_handlers()
    {
        match method
        {
            rest::RestMethod::GET => router.get(action, handler),
            rest::RestMethod::POST => router.post(action, handler),
            rest::RestMethod::PUT => router.put(action, handler),
            rest::RestMethod::DELETE => router.delete(action, handler)
        };
    }
}