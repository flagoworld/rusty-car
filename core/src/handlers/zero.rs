use lin::frame::LINFrameHandler;

pub struct Zero;

impl Zero
{
	pub fn new() -> Zero
	{
		Zero
	}
}

impl LINFrameHandler for Zero
{
	fn boxed_new(&self) -> Box<LINFrameHandler>
	{
		Box::new(Zero::new())
	}

    fn response_data(&self) -> Vec<u8>
    {
		return vec![1, 2, 3];
    }

	fn handle_response(&self, data: &Vec<u8>)
	{
		println!("HANDLE: {:?}", data);
	}
}
