extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate serial;
extern crate time;
extern crate websocket;

use std::thread;
use std::sync::Arc;

use lin::LINOptions;
use lin::LINMaster;
use lin::frame;
use lin::frame::LINFrame;
use lin::frame::handlers::zero::Zero;

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};
use router::Router;

use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;
use websocket::stream::WebSocketStream;

extern crate rand;

use car_store::CStore;

mod lin;
mod rest;
mod car_store;
mod wsocket;

fn main()
{
    let store = Arc::new(CStore::new());

    let rest_thread = thread::Builder::new().name("rusty-car-rest".to_string()).spawn(move ||
    {
        let mut router = Router::new();
        let mut store = store.clone();

        setup_router(&mut router, &mut store);

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

    let ws_thread = thread::Builder::new().name("rusty-car-websocket".to_string()).spawn(move ||
    {
        let server = Server::bind("127.0.0.1:2794").unwrap();

    	for connection in server
        {
    		// Spawn a new thread for each connection.
    		thread::spawn(move ||
            {
    			let request = connection.unwrap().read_request().unwrap(); // Get the request
    			let headers = request.headers.clone(); // Keep the headers so we can check them

    			request.validate().unwrap(); // Validate the request

    			let mut response = request.accept(); // Form a response

    			if let Some(&WebSocketProtocol(ref protocols)) = headers.get()
                {
    				if protocols.contains(&("rust-websocket".to_string()))
                    {
    					// We have a protocol we want to use
    					response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
    				}
    			}

    			let mut client = response.send().unwrap(); // Send the response

    			let ip = client.get_mut_sender()
    				.get_mut()
    				.peer_addr()
    				.unwrap();

    			println!("Connection from {}", ip);

    			let message = Message::Text("Hello".to_string());
    			client.send_message(message).unwrap();

    			let (mut sender, mut receiver) = client.split();

    			for message in receiver.incoming_messages()
                {
    				let message = message.unwrap();

    				// wsocket::route(&mut sender, message);

                    match message
                    {
                        Message::Close(_) =>
                        {
                            let message = Message::Close(None);
                            sender.send_message(message).unwrap();
                            println!("Client disconnected");
                            return;
                        }
                        Message::Ping(data) =>
                        {
                            let message = Message::Pong(data);
                            sender.send_message(message).unwrap();
                        }
                        Message::Text(data) =>
                        {
                            let mph: u8 = (rand::random::<u8>() % 100) + 1;
                            let rpm: u16 = (rand::random::<u16>() % 8000) + 1;
                            let message = Message::Text(match data.as_ref()
                            {
                                "mph" => format!("mph:{}", mph),
                                "rpm" => format!("rpm:{}", rpm),
                                _ => continue
                            }.to_string());

                            sender.send_message(message).unwrap();
                        }
                        _ => sender.send_message(message).unwrap(),
                    }
    			}
    		});
    	}
    });

    let rest_thread_output = rest_thread.unwrap().join().unwrap();
    let lin_thread_output = lin_thread.unwrap().join().unwrap();
    let ws_thread_output = ws_thread.unwrap().join().unwrap();

    println!("REST OUTPUT: {:?}", rest_thread_output);
    println!("LIN OUTPUT: {:?}", lin_thread_output);
}

fn load_frames(master: &mut LINMaster)
{
    master.add_frame(LINFrame::new(0, frame::Type::Unconditional, true, vec![], Box::new(Zero::new())));
}

fn setup_router(router: &mut router::Router, store: &mut Arc<CStore>)
{
    for(method, action, handler) in rest::get_handlers()
    {
        let m = match method
        {
            rest::RestMethod::GET => iron::method::Get,
            rest::RestMethod::POST => iron::method::Post,
            rest::RestMethod::PUT => iron::method::Put,
            rest::RestMethod::DELETE => iron::method::Delete
        };

        let connection_handler = ConnectionHandler { handler: handler, store: store.clone() };

        router.route(m, action, connection_handler);
    }
}

struct ConnectionHandler
{
    handler: fn(&mut Arc<CStore>, &mut Request) -> IronResult<Response>,
    store: Arc<CStore>
}

impl iron::Handler for ConnectionHandler
{
    fn handle(&self, req: &mut Request) -> IronResult<Response>
    {
        let handler = self.handler;
        let mut store = self.store.clone();

        handler(&mut store, req)
    }
}