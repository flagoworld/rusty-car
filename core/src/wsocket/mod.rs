use websocket::{Message, Sender};
use std::io::Write;
use websocket::stream::WebSocketStream;
use rand;

pub fn route(sender: &mut Sender<WebSocketStream>, message: Message)
{
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
                _ => return
            }.to_string());

            sender.send_message(message).unwrap();
        }
        _ => sender.send_message(message).unwrap(),
    }
}