extern crate iron;

use iron::Request;

fn getRoot(req: &mut router::Request)
{
    Ok(Response::with(status::Ok, "HELLO, WORLD"));
}