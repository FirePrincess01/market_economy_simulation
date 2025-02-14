//! Contains a reference to the gpu buffer with all the entity indices
//!

pub struct EntityBufferSlice<'a> {
    buffer: &'a wgpu::Buffer,
    buffer_slice: wgpu::BufferSlice<'a>,
    size: wgpu::Extent3d,
    unpadded_bytes_per_row: u32,
    padded_bytes_per_row: u32,
}

impl<'a> EntityBufferSlice<'a> {
    pub fn new(
        buffer: &'a wgpu::Buffer,
        buffer_slice: wgpu::BufferSlice<'a>,
        size: wgpu::Extent3d,
        unpadded_bytes_per_row: u32,
        padded_bytes_per_row: u32,
    ) -> Self {
        Self {
            buffer,
            buffer_slice,
            size,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }

    fn as_u32_le(array: &[u8; 4]) -> u32 {
        ((array[0] as u32) << 0)
            + ((array[1] as u32) << 8)
            + ((array[2] as u32) << 16)
            + ((array[3] as u32) << 24)
    }

    pub fn get(&self, y: u32, x: u32) -> u32 {
        let padded_data = self.buffer_slice.get_mapped_range();

        let mut data = padded_data
            .chunks(self.padded_bytes_per_row as _)
            .map(|chunk| &chunk[..self.unpadded_bytes_per_row as _])
            .flatten();

        let index = y * self.width() * 4 + x * 4;

        let default: u8 = 0;
        let elem0 = data.nth(index as usize).unwrap_or(&default);
        let elem1 = data.nth((0) as usize).unwrap_or(&default);
        let elem2 = data.nth((0) as usize).unwrap_or(&default);
        let elem3 = data.nth((0) as usize).unwrap_or(&default);

        let elem = Self::as_u32_le(&[*elem0, *elem1, *elem2, *elem3]);

        elem
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn _height(&self) -> u32 {
        self.size.height
    }

    pub fn unmap(&self) {
        self.buffer.unmap();
    }
}

impl<'a> Drop for EntityBufferSlice<'a> {
    fn drop(&mut self) {
        self.unmap();
    }
}
