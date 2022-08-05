#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Rgba {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Default for Rgba {
	#[inline]
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
	#[inline]
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
	pub const WHITE: Rgba = Rgba {
		r: 255,
		g: 255,
		b: 255,
		a: 255,
	};
	pub const BLACK: Rgba = Rgba {
		r: 0,
		g: 0,
		b: 0,
		a: 255,
	};

	#[inline]
	#[must_use]
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
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

	#[inline]
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

	#[inline]
	#[must_use]
	pub fn get_pixel(&self, x: usize, y: usize) -> &Rgba {
		&self[(x, y)]
	}

	#[inline]
	pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgba) {
		self[(x, y)] = color;
	}

	#[inline]
	#[must_use]
	pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
		x < self.size[0] && y < self.size[1]
	}

	#[inline]
	#[must_use]
	pub fn get_scaled_size(&self, scale: f32) -> [f32; 2] {
		[self.size[0] as f32 * scale, self.size[1] as f32 * scale]
	}

	#[inline]
	#[must_use]
	pub fn get_size(&self) -> [usize; 2] {
		self.size
	}

	#[must_use]
	pub fn get_buf(&self) -> Vec<u8> {
		let (width, height) = (self.size[0], self.size[1]);

		let mut pixels = vec![0; width * height * 4];

		let mut i = 0;
		for y in 0..height {
			for x in 0..width {
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

	#[inline]
	pub fn clear(&mut self, color: Rgba) {
		for pixel in &mut self.pixels {
			*pixel = color;
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
