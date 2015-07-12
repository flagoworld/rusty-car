pub struct LINFrame
{
    id: u8
}

impl LINFrame
{
    fn new() -> LINFrame
    {
        LINFrame { id: 0u8 }
    }

    pub fn id(&self) -> u8
    {
        return self.id;
    }
}