use std::collections::HashMap;

use super::chunk::{CS, Chunk, FACES, UVS};
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex};

pub type ChunkPos = (i32, i32, i32);

type Noise = Fbm<OpenSimplex>;

fn make_noise(seed: u32, frequency: f64, octaves: usize) -> Noise {
    Fbm::<OpenSimplex>::new(seed)
        .set_frequency(frequency)
        .set_octaves(octaves)
}

pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
    seed: u32,
}

impl Default for World {
    fn default() -> Self {
        let seed = rand::random::<u32>();
        Self::new(seed)
    }
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            chunks: HashMap::new(),
            seed,
        }
    }

    pub fn generate(&mut self, nx: i32, ny: i32, nz: i32) {
        let cs = CS as i32;
        let world_h = (ny * cs) as f32;
        let terrain = make_noise(self.seed, 0.010, 4);
        let cave_a = make_noise(self.seed + 1, 0.020, 2);
        let cave_b = make_noise(self.seed + 2, 0.020, 2);

        for cx in 0..nx {
            for cz in 0..nz {
                let mut heights = [[0i32; CS]; CS];
                for lx in 0..CS {
                    for lz in 0..CS {
                        let wx = (cx * cs + lx as i32) as f32;
                        let wz = (cz * cs + lz as i32) as f32;
                        let n = terrain.get([wx as f64, wz as f64]) as f32; // -1..1
                        let h = world_h * 0.5 + n * (world_h * 0.4);
                        heights[lx][lz] = (h as i32).clamp(4, world_h as i32 - 2);
                    }
                }

                for cy in 0..ny {
                    let mut chunk = Chunk::new();
                    for lx in 0..CS {
                        for lz in 0..CS {
                            let h = heights[lx][lz];
                            for ly in 0..CS {
                                let wx = cx * cs + lx as i32;
                                let wy = cy * cs + ly as i32;
                                let wz = cz * cs + lz as i32;

                                if wy >= h {
                                    break;
                                }
                                if wy >= 1 && is_cave(&cave_a, &cave_b, wx, wy, wz) {
                                    continue;
                                }
                                chunk.set(lx, ly, lz, 1);
                            }
                        }
                    }
                    if !chunk.is_empty() {
                        self.chunks.insert((cx, cy, cz), chunk);
                    }
                }
            }
        }
    }

    pub fn positions(&self) -> Vec<ChunkPos> {
        self.chunks.keys().copied().collect()
    }

    pub fn block(&self, x: i32, y: i32, z: i32) -> u8 {
        if y < 0 {
            return 1;
        }
        let cs = CS as i32;
        let pos = (x.div_euclid(cs), y.div_euclid(cs), z.div_euclid(cs));
        match self.chunks.get(&pos) {
            Some(chunk) => chunk.get(
                x.rem_euclid(cs) as usize,
                y.rem_euclid(cs) as usize,
                z.rem_euclid(cs) as usize,
            ),
            None => 0,
        }
    }

    pub fn build_mesh(&self, pos: ChunkPos) -> (Vec<f32>, Vec<u32>) {
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let Some(chunk) = self.chunks.get(&pos) else {
            return (vertices, indices);
        };

        let cs = CS as i32;
        let (ox, oy, oz) = (pos.0 * cs, pos.1 * cs, pos.2 * cs);

        for z in 0..CS {
            for y in 0..CS {
                for x in 0..CS {
                    if chunk.get(x, y, z) == 0 {
                        continue;
                    }

                    for (dir, corners) in &FACES {
                        let (nx, ny, nz) =
                            (x as i32 + dir[0], y as i32 + dir[1], z as i32 + dir[2]);

                        let inside =
                            (0..cs).contains(&nx) && (0..cs).contains(&ny) && (0..cs).contains(&nz);
                        let neighbor = if inside {
                            chunk.get(nx as usize, ny as usize, nz as usize)
                        } else {
                            self.block(ox + nx, oy + ny, oz + nz)
                        };
                        if neighbor != 0 {
                            continue;
                        }

                        let base = (vertices.len() / 5) as u32;
                        for (c, uv) in corners.iter().zip(UVS.iter()) {
                            vertices.extend_from_slice(&[
                                x as f32 + c[0],
                                y as f32 + c[1],
                                z as f32 + c[2],
                                uv[0],
                                uv[1],
                            ]);
                        }
                        indices.extend_from_slice(&[
                            base,
                            base + 1,
                            base + 2,
                            base + 2,
                            base + 3,
                            base,
                        ]);
                    }
                }
            }
        }

        (vertices, indices)
    }
}

const CAVE_WIDTH: f64 = 0.07;

fn is_cave(a: &Noise, b: &Noise, x: i32, y: i32, z: i32) -> bool {
    let p = [x as f64, y as f64 * 1.6, z as f64];
    if a.get(p).abs() >= CAVE_WIDTH {
        return false;
    }
    b.get(p).abs() < CAVE_WIDTH
}
