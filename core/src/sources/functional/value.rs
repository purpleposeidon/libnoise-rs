use super::constants::PERMUTATION_TABLE_SIZE;
use crate::utils::ptable::PermutationTable;

pub(crate) fn noise1d(perm: &PermutationTable, point: [f64; 1]) -> f64 {
    let x = point[0];
    // origin of hypercube in which input lies
    let mut x0 = x.floor() as usize;
    // smoothed distance from hypercube origin
    let dxs = smoothstep_3(x - x0 as f64);
    // get values from hypercube corners
    x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let f0 = unsafe { perm.hash1d(x0) } as f64;
    let f1 = unsafe { perm.hash1d(x0 + 1) } as f64;
    // interpolate values from hypercube corners
    let xf = lerp(f0, f1, dxs);
    normalize(xf)
}

pub(crate) fn noise2d(perm: &PermutationTable, point: [f64; 2]) -> f64 {
    let x = point[0];
    let y = point[1];
    // origin of hypercube in which input lies
    let mut x0 = x.floor() as usize;
    let mut y0 = y.floor() as usize;
    // smoothed distance from hypercube origin
    let dxs = smoothstep_3(x - x0 as f64);
    let dys = smoothstep_3(y - y0 as f64);
    // get values from hypercube corners
    x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE);
    y0 = y0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let f00 = unsafe { perm.hash2d(x0, y0) } as f64;
    let f01 = unsafe { perm.hash2d(x0, y0 + 1) } as f64;
    let f10 = unsafe { perm.hash2d(x0 + 1, y0) } as f64;
    let f11 = unsafe { perm.hash2d(x0 + 1, y0 + 1) } as f64;
    // interpolate values from hypercube corners
    let xf0 = lerp(f00, f10, dxs);
    let xf1 = lerp(f01, f11, dxs);
    let yf = lerp(xf0, xf1, dys);
    normalize(yf)
}

pub(crate) fn noise3d(perm: &PermutationTable, point: [f64; 3]) -> f64 {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    // origin of hypercube in which input lies
    let mut x0 = x.floor() as usize;
    let mut y0 = y.floor() as usize;
    let mut z0 = z.floor() as usize;
    // smoothed distance from hypercube origin
    let dxs = smoothstep_3(x - x0 as f64);
    let dys = smoothstep_3(y - y0 as f64);
    let dzs = smoothstep_3(z - z0 as f64);
    // get values from hypercube corners
    x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE);
    y0 = y0.rem_euclid(PERMUTATION_TABLE_SIZE);
    z0 = z0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let f000 = unsafe { perm.hash3d(x0, y0, z0) } as f64;
    let f001 = unsafe { perm.hash3d(x0, y0, z0 + 1) } as f64;
    let f010 = unsafe { perm.hash3d(x0, y0 + 1, z0) } as f64;
    let f011 = unsafe { perm.hash3d(x0, y0 + 1, z0 + 1) } as f64;
    let f100 = unsafe { perm.hash3d(x0 + 1, y0, z0) } as f64;
    let f101 = unsafe { perm.hash3d(x0 + 1, y0, z0 + 1) } as f64;
    let f110 = unsafe { perm.hash3d(x0 + 1, y0 + 1, z0) } as f64;
    let f111 = unsafe { perm.hash3d(x0 + 1, y0 + 1, z0 + 1) } as f64;
    // interpolate values from hypercube corners
    let xf00 = lerp(f000, f100, dxs);
    let xf01 = lerp(f001, f101, dxs);
    let xf10 = lerp(f010, f110, dxs);
    let xf11 = lerp(f011, f111, dxs);
    let yf0 = lerp(xf00, xf10, dys);
    let yf1 = lerp(xf01, xf11, dys);
    let zf = lerp(yf0, yf1, dzs);
    normalize(zf)
}

pub(crate) fn noise4d(perm: &PermutationTable, point: [f64; 4]) -> f64 {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    let w = point[3];
    // origin of hypercube in which input lies
    let mut x0 = x.floor() as usize;
    let mut y0 = y.floor() as usize;
    let mut z0 = z.floor() as usize;
    let mut w0 = w.floor() as usize;
    // smoothed distance from hypercube origin
    let dxs = smoothstep_3(x - x0 as f64);
    let dys = smoothstep_3(y - y0 as f64);
    let dzs = smoothstep_3(z - z0 as f64);
    let dws = smoothstep_3(w - w0 as f64);
    // get values from hypercube corners
    x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE);
    y0 = y0.rem_euclid(PERMUTATION_TABLE_SIZE);
    z0 = z0.rem_euclid(PERMUTATION_TABLE_SIZE);
    w0 = w0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let f0000 = unsafe { perm.hash4d(x0, y0, z0, w0) } as f64;
    let f0001 = unsafe { perm.hash4d(x0, y0, z0, w0 + 1) } as f64;
    let f0010 = unsafe { perm.hash4d(x0, y0, z0 + 1, w0) } as f64;
    let f0011 = unsafe { perm.hash4d(x0, y0, z0 + 1, w0 + 1) } as f64;
    let f0100 = unsafe { perm.hash4d(x0, y0 + 1, z0, w0) } as f64;
    let f0101 = unsafe { perm.hash4d(x0, y0 + 1, z0, w0 + 1) } as f64;
    let f0110 = unsafe { perm.hash4d(x0, y0 + 1, z0 + 1, w0) } as f64;
    let f0111 = unsafe { perm.hash4d(x0, y0 + 1, z0 + 1, w0 + 1) } as f64;
    let f1000 = unsafe { perm.hash4d(x0 + 1, y0, z0, w0) } as f64;
    let f1001 = unsafe { perm.hash4d(x0 + 1, y0, z0, w0 + 1) } as f64;
    let f1010 = unsafe { perm.hash4d(x0 + 1, y0, z0 + 1, w0) } as f64;
    let f1011 = unsafe { perm.hash4d(x0 + 1, y0, z0 + 1, w0 + 1) } as f64;
    let f1100 = unsafe { perm.hash4d(x0 + 1, y0 + 1, z0, w0) } as f64;
    let f1101 = unsafe { perm.hash4d(x0 + 1, y0 + 1, z0, w0 + 1) } as f64;
    let f1110 = unsafe { perm.hash4d(x0 + 1, y0 + 1, z0 + 1, w0) } as f64;
    let f1111 = unsafe { perm.hash4d(x0 + 1, y0 + 1, z0 + 1, w0 + 1) } as f64;
    // interpolate values from hypercube corners
    let xf000 = lerp(f0000, f1000, dxs);
    let xf001 = lerp(f0001, f1001, dxs);
    let xf010 = lerp(f0010, f1010, dxs);
    let xf011 = lerp(f0011, f1011, dxs);
    let xf100 = lerp(f0100, f1100, dxs);
    let xf101 = lerp(f0101, f1101, dxs);
    let xf110 = lerp(f0110, f1110, dxs);
    let xf111 = lerp(f0111, f1111, dxs);
    let yf00 = lerp(xf000, xf100, dys);
    let yf01 = lerp(xf001, xf101, dys);
    let yf10 = lerp(xf010, xf110, dys);
    let yf11 = lerp(xf011, xf111, dys);
    let zf0 = lerp(yf00, yf10, dzs);
    let zf1 = lerp(yf01, yf11, dzs);
    let wf = lerp(zf0, zf1, dws);
    normalize(wf)
}

fn normalize(x: f64) -> f64 {
    2.0 / PERMUTATION_TABLE_SIZE as f64 * x - 1.0
}

fn smoothstep_3(t: f64) -> f64 {
    t * t * (t * (-2.0) + 3.0)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}
