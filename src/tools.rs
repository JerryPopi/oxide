
use egui_extras::RetainedImage;

use crate::macros::macros::load_svg;

pub struct ToolIcons {
	pub drag: RetainedImage,
	pub select: RetainedImage
}

impl ToolIcons {
	pub fn new() -> Self {
		ToolIcons {
			drag: load_svg!("drag"),
    		select: load_svg!("select2"),
		}
	}
}


#[derive(PartialEq)]
pub enum Tools {
	Drag,
	Select,
	None
}