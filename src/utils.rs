pub fn to_3d(pos: usize, width: usize, height: usize) -> (usize, usize, usize) {
    (pos % width, (pos / width) % height, pos / (width * height))
}

pub fn to_1d(x: usize, y: usize, z: usize, width: usize, height: usize) -> usize {
    (z * width * height) + (y * width) + x
}