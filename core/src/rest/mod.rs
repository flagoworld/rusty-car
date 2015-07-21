use std::sync::Arc;

use iron::{Request, Response, IronResult};
use iron;
use rustc_serialize::json;

use car_store::CStore;

pub enum RestMethod
{
    GET,
    POST,
    PUT,
    DELETE
}

pub fn get_handlers() -> Vec<(RestMethod, &'static str, fn(&mut Arc<CStore>, &mut Request) -> IronResult<Response>)>
{
    use self::RestMethod::{GET, POST, PUT, DELETE};

    vec!
    [
        (GET, "/", get_root),

        (GET, "/heater", get_heater),
        (PUT, "/heater", get_heater),

        (GET, "/audio", get_audio),
        (PUT, "/audio", get_audio)
    ]
}

#[derive(RustcEncodable)]
struct ServerInfo
{
    name: &'static str,
    description: &'static str,
    version: &'static str
}

const server_info_object: ServerInfo = ServerInfo
{
    name: "Rusty Car",
    description: "A touch-screen car management service with integrated LIN support for the Raspberry Pi",
    version: "0.0.1"
};

fn get_root(store: &mut Arc<CStore>, req: &mut Request) -> IronResult<Response>
{
    let response = json::encode(&server_info_object).unwrap();

    println!("{:?}", store);

    Ok(Response::with((iron::status::Ok, response)))
}

fn get_heater(store: &mut Arc<CStore>, req: &mut Request) -> IronResult<Response>
{
    Ok(Response::with((iron::status::Ok, "Get Heater Settings")))
}

fn put_heater(store: &mut Arc<CStore>, req: &mut Request) -> IronResult<Response>
{
    Ok(Response::with((iron::status::Ok, "Put Heater Settings")))
}

fn get_audio(store: &mut Arc<CStore>, req: &mut Request) -> IronResult<Response>
{
    Ok(Response::with((iron::status::Ok, "Get Audio Settings")))
}

fn put_audio(store: &mut Arc<CStore>, req: &mut Request) -> IronResult<Response>
{
    Ok(Response::with((iron::status::Ok, "Put Audio Settings")))
}