use crate::map;

pub const BLACK:Color = Color::Rgb(0, 0, 0);
pub const WHITE:Color = Color::Rgb(255, 255, 255);
pub const AMBER:Color = Color::Rgb(245, 158, 11);
pub const GREEN:Color = Color::Rgb(34, 197, 94);
pub const BLUE:Color = Color::Rgb(0, 0, 254);
pub const RED:Color = Color::Rgb(255, 10, 94);
pub const TEAL:Color = Color::Rgb(20, 184, 166);
pub const INDIGO:Color = Color::Rgb(99, 102, 241);
pub const PINK:Color = Color::Rgb(236, 72, 153);
pub const TRANSPARENT:Color = Color::Rgba(0, 0, 0,0);

/// Represents a color. 
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Color{
	Rgb(u8,u8,u8),
	Rgba(u8,u8,u8,u8),
	/// If an invalid hex code is used, it will default back to 
	/// white. Use the `hex!` macro to validate hex codes at compile time.
	Hex(&'static str)
} 

impl Color {
	// TODO test this and add other variants
	/// Convert any variant of [`Color`] to the `Rgb` variant
	pub fn as_rgb(&self) -> Self{
		let [r,g,b,_] = self.to_rgba();
		Self::Rgb(r, g, b)
	}

	/// Convert a [`Color`] into a hex string  
	/// # Example
	/// ```
	/// use helium_core::color::Color;
	/// 
	/// let color = Color::Rgb(255,255,255);
	/// 
	/// assert_eq!(color.into_hex_string(),format!("#ffffff"))
	/// ```
	/// **NOTE** than this does not do any color conversion,
	/// so invalid hex codes will returned as is
	pub fn into_hex_string(&self) -> String {
		match self {
			Self::Hex(hex) => hex.to_string(),
			Self::Rgb(r,g,b)|
			Self::Rgba(r,g,b,_) => {
				format!("#{:x}{:x}{:x}", r, g, b)
			}
		}
	}

	/// Parse any type of color to rgba values
	pub fn to_rgba(&self) -> [u8;4] {		
		match self {
			Self::Rgb(r,g,b) => [*r,*g,*b,100],
			Self::Rgba(r,g,b,mut a) => {
				if a > 100 {a = 100}
				[*r,*g,*b,a]
			},
			Self::Hex(color) => {
				Color::hex_to_rgba(&color).unwrap_or([255,255,255,100])
			}
		}
	}

	/// Convert a hex color to an rgba color. Returns an error if an invalid hex code
	/// is provided.  
	/// Examples of invalid hexcodes
	/// - Strings that don't begin with `#`
	/// - Any string that isn't six characters in length
	/// - Any string that isn't is hexadecimal format
	pub fn hex_to_rgba(hex:&str) -> Result<[u8;4],String>{
		// TODO add custom error
		
		let hex_code = hex.strip_prefix("#").ok_or("Invalid hex code: missing `#` at the start of hex")?;

		if hex_code.len() != 6 {
			return Err("Invalid hex code: Hex colors should be 6 characters in length".into());
		}

		let (red,green,blue) = (
			&hex_code[0..2],
			&hex_code[2..4],
			&hex_code[4..6],
		);

		let r = u8::from_str_radix(red, 16).map_err(|err|{format!("Failed to parse hex code:{err}")})?;
		let g = u8::from_str_radix(green, 16).map_err(|err|{format!("Failed to parse hex code:{err}")})?;
		let b = u8::from_str_radix(blue, 16).map_err(|err|{format!("Failed to parse hex code:{err}")})?;

		Ok([r,g,b,100])
	}

	/// Normalize the colors to a 0 - 1 scale.
	pub fn normalize(&self) -> [f32;4] {
		let rgba = self.to_rgba();

		let r = map(rgba[0] as f32, [0.0,255.0], [0.0,1.0]);
		let g = map(rgba[1] as f32, [0.0,255.0], [0.0,1.0]);
		let b = map(rgba[2] as f32, [0.0,255.0], [0.0,1.0]);
		let a = map(rgba[3] as f32, [0.0,100.0], [0.0,1.0]);
		
		[r,g,b,a]
	}
}

// TODO also impl try from string
impl ToString for Color {
	/// Convert this color into 
	fn to_string(&self) -> String {
		todo!()		
	}
}

impl Default for Color {
	/// The default color is `Transparent` i.e. Rgba(0,0,0,0)
	fn default() -> Self {
		Self::Rgba(0,0,0,0)
	}
}

#[cfg(test)]
mod test{
    use super::*;

	#[test]
	fn test_valid_hex_colors(){
		// Invalid hex colors are defaulted to white
		let color = Color::Hex("#".into());
		assert_eq!(color.to_rgba(),[255,255,255,100]);
		
		// Valid hex codes
		let color = Color::Hex("#ffffff".into());
		assert_eq!(color.to_rgba(),[255,255,255,100]);
		
		let color = Color::Hex("#faba32".into());
		assert_eq!(color.to_rgba(),[250,186,50,100]);
		
		let color = Color::Hex("#345af0".into());
		assert_eq!(color.to_rgba(),[52,90,240,100]);
	}

	#[test]
	fn test_hex_conversion_errors(){
		assert_eq!(Color::hex_to_rgba(""),Err("Invalid hex code: missing `#` at the start of hex".into()));
		assert_eq!(Color::hex_to_rgba("#"),Err("Invalid hex code: Hex colors should be 6 characters in length".into()));
		assert_eq!(Color::hex_to_rgba("ffffff"),Err("Invalid hex code: missing `#` at the start of hex".into()));
	}
}