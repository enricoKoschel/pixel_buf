#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct Rgba {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Default for Rgba {
	fn default() -> Self {
		Rgba {
			r: 0,
			g: 0,
			b: 0,
			a: 255,
		}
	}
}

#[derive(Clone)]
pub struct PixelBuf {
	size: [usize; 2],
	pixels: Vec<Rgba>,
}

impl From<[u8; 4]> for Rgba {
	#[must_use]
	fn from(array: [u8; 4]) -> Self {
		Self {
			r: array[0],
			g: array[1],
			b: array[2],
			a: array[3],
		}
	}
}

impl Rgba {
	#[must_use]
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}

	pub fn black() -> Self {
		Self::new(0, 0, 0, 255)
	}

	pub fn white() -> Self {
		Self::new(255, 255, 255, 255)
	}
}

impl PixelBuf {
	#[must_use]
	pub fn new(size: [usize; 2]) -> Self {
		if size[0] == 0 || size[1] == 0 {
			panic!("size must be non-zero");
		}

		Self {
			size,
			pixels: vec![Rgba::default(); size[0] * size[1]],
		}
	}

	#[must_use]
	pub fn new_from_fn<F: Fn(usize, usize) -> Rgba>(size: [usize; 2], f: F) -> Self {
		let mut buf = Self::new(size);

		for y in 0..size[1] {
			for x in 0..size[0] {
				buf.set_pixel(x, y, f(x, y));
			}
		}

		buf
	}

	#[must_use]
	pub fn new_test_image(size: [usize; 2]) -> Self {
		Self::new_from_fn(size, |x, y| match (x + y) % 4 {
			0 => Rgba::new(0, 0, 0, 255),
			1 => Rgba::new(255, 0, 0, 255),
			2 => Rgba::new(0, 255, 0, 255),
			3 => Rgba::new(0, 0, 255, 255),
			_ => unreachable!(),
		})
	}

	#[must_use]
	pub fn get_pixel(&self, x: usize, y: usize) -> &Rgba {
		&self[(x, y)]
	}

	pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgba) {
		self[(x, y)] = color;
	}

	#[must_use]
	pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
		x < self.size[0] && y < self.size[1]
	}

	#[must_use]
	pub fn get_scaled_size(&self, scale: usize) -> [usize; 2] {
		[self.size[0] * scale, self.size[1] * scale]
	}

	#[must_use]
	pub fn get_size(&self) -> [usize; 2] {
		self.size
	}

	#[must_use]
	pub fn get_scaled_buf(&self, scale: usize) -> Vec<u8> {
		let (scaled_width, scaled_height) = (self.size[0] * scale, self.size[1] * scale);

		let mut pixels = vec![0; scaled_width * scaled_height * 4];

		let mut i = 0;
		for y in 0..scaled_height {
			for x in 0..scaled_width {
				let x = x / scale;
				let y = y / scale;

				let pixel = self.get_pixel(x, y);

				pixels[i] = pixel.r;
				pixels[i + 1] = pixel.g;
				pixels[i + 2] = pixel.b;
				pixels[i + 3] = pixel.a;

				i += 4;
			}
		}

		pixels
	}

	pub fn clear(&mut self, color: Rgba) {
		for pixel in &mut self.pixels {
			*pixel = color.clone();
		}
	}
}

impl core::ops::Index<(usize, usize)> for PixelBuf {
	type Output = Rgba;

	#[must_use]
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		let (x, y) = index;

		if !self.is_in_bounds(x, y) {
			panic!("Pixel {:?} out of bounds {:?}", (x, y), self.size);
		}

		let index = y * self.size[0] + x;

		&self.pixels[index]
	}
}

impl core::ops::IndexMut<(usize, usize)> for PixelBuf {
	#[must_use]
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		let (x, y) = index;

		if !self.is_in_bounds(x, y) {
			panic!("Pixel {:?} out of bounds {:?}", (x, y), self.size);
		}

		let index = y * self.size[0] + x;

		&mut self.pixels[index]
	}
}
