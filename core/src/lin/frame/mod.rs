pub enum Type
{
    Unconditional,
    EventTriggered,
    Sporadic
}

pub enum ID
{
    Signal = 0x00,
    DiagnosticRequest = 0x3c,
    DiagnosticResponse = 0x3d,
    UserDefined = 0x3e,
    Reserved = 0x3f
}

pub struct LINFrame
{
    id: u8,
    frame_type: Type,
    request_frame: bool,
    collision_frames: Vec<u8>,
    handler: Box<LINFrameHandler>
}

impl LINFrame
{
    fn new<T: LINFrameHandler>(id: u8, frame_type: Type, request_frame: bool, collision_frames: Vec<u8>, handler: T) -> LINFrame
    {
        LINFrame { id: id, frame_type: frame_type, request_frame: true, collision_frames: collision_frames, handler: Box::new(handler) }
    }
}

trait LINFrameHandler
{
    // Array of up to 8 bytes
    fn response_data(&self) -> Vec<u8>;

    // Only called if request_fram == true
    fn handle_response(&self, data: Vec<u8>);
}

// struct DefaultHandler
// {
//
// }
//
// impl LINFrameHandler for DefaultHandler
// {
//     fn response_data(&self) -> Vec<u8>
//     {
//         vec![];
//     }
//
//     fn handle_response(&self, data: Vec<u8>)
//     {
//
//     }
// }