// use websocket::{Message, Sender};
// use std::io::Write;
// use websocket::stream::WebSocketStream;
//
// pub fn route<D>(sender: &mut Sender<D>, message: Message)
// {
//     match message
//     {
//         Message::Close(_) =>
//         {
//             let message = Message::Close(None);
//             sender.send_message(message).unwrap();
//             println!("Client disconnected");
//             return;
//         }
//         Message::Ping(data) =>
//         {
//             let message = Message::Pong(data);
//             sender.send_message(message).unwrap();
//         }
//         // Message::Text(data) =>
//         // {
//         //     let message = Message::Text(match &data
//         //     {
//         //         "mph" => "12",
//         //         "rpm" => "1350"
//         //     });
//         //
//         //     sender.send_message(message).unwrap();
//         // }
//         _ => sender.send_message(message).unwrap(),
//     }
// }