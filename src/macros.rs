pub mod macros {
	macro_rules! load_svg {
		($name: expr) => {
			RetainedImage::from_svg_bytes(format!("{}.svg", $name), include_bytes!(concat!("cursors/", $name, ".svg"))).unwrap()
		}
	}
	pub (crate) use load_svg;
}