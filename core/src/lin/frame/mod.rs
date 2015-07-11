pub struct LINFrame
{
    frame_type: String
}

impl LINFrame
{
    fn new() -> LINFrame
    {
        LINFrame { frame_type: "LINFrame".to_string() }
    }
}