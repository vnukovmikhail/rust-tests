pub const CS: usize = 16;

#[rustfmt::skip]
pub const FACES: [([i32; 3], [[f32; 3]; 4]); 6] = [
    ([0, 0, 1], [[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]]),
    ([0, 0, -1], [[1., 0., 0.], [0., 0., 0.], [0., 1., 0.], [1., 1., 0.]]),
    ([1, 0, 0], [[1., 0., 1.], [1., 0., 0.], [1., 1., 0.], [1., 1., 1.]]),
    ([-1, 0, 0], [[0., 0., 0.], [0., 0., 1.], [0., 1., 1.], [0., 1., 0.]]),
    ([0, 1, 0], [[0., 1., 1.], [1., 1., 1.], [1., 1., 0.], [0., 1., 0.]]),
    ([0, -1, 0], [[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]]),
];

pub const UVS: [[f32; 2]; 4] = [[0., 0.], [1., 0.], [1., 1.], [0., 1.]];

pub struct Chunk {
    blocks: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: vec![0; CS * CS * CS],
        }
    }

    fn idx(x: usize, y: usize, z: usize) -> usize {
        x + CS * (y + CS * z)
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, id: u8) {
        self.blocks[Self::idx(x, y, z)] = id;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
        self.blocks[Self::idx(x, y, z)]
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(|&b| b == 0)
    }
}
