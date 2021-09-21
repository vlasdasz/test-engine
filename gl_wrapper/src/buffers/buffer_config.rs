#[cfg(any(target_os = "ios", target_os = "android"))]
use gles31_sys::*;
use tools::*;

#[derive(Debug)]
pub struct BufferConfig {
    size:        u8,
    vertex_size: u8,
    config:      [u8; 3],
}

impl BufferConfig {
    fn stride_for_index(&self, index: u8) -> u8 {
        if index == 0 {
            return 0;
        }
        if index == 1 {
            return self.vertex_size - self.config[1] - self.config[2];
        }
        self.vertex_size - self.config[2]
    }
}

impl BufferConfig {
    const fn new(first: u8, second: u8, third: u8) -> BufferConfig {
        //const_assert!(first > 0); // check
        let config: [u8; 3] = [first, second, third];
        let mut size = 1;
        if second > 0 {
            size += 1
        }
        if third > 0 {
            size += 1
        }
        let vertex_size = first + second + third;

        BufferConfig {
            size,
            vertex_size,
            config,
        }
    }
}

impl BufferConfig {
    pub fn size(&self) -> u8 { self.vertex_size }
    pub fn set_pointers(&self) {
        const GLFLOAT_SIZE: u8 = std::mem::size_of::<GLT!(GLfloat)>() as u8;
        for i in 0..self.size {
            GL!(EnableVertexAttribArray, i.into());
            GL!(
                VertexAttribPointer,
                i.into(),
                self.config[i as usize] as i32,
                GLC!(FLOAT),
                0,
                (self.vertex_size * GLFLOAT_SIZE) as GLT!(GLint),
                (self.stride_for_index(i) * GLFLOAT_SIZE) as *const GLT!(GLvoid)
            );
        }
    }
}

impl BufferConfig {
    pub const _2: BufferConfig = BufferConfig::new(2, 0, 0);
    pub const _2_2: BufferConfig = BufferConfig::new(2, 2, 0);
    pub const _3_3: BufferConfig = BufferConfig::new(3, 3, 0);
    pub const _3_3_2: BufferConfig = BufferConfig::new(3, 3, 2);
    pub const _3_3_3: BufferConfig = BufferConfig::new(3, 3, 3);
    pub const _3_3_4: BufferConfig = BufferConfig::new(3, 3, 4);
}
