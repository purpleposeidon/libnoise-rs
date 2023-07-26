pub(crate) const R_SQUARED: f64 = 0.5;

pub(crate) const SKEW_FACTOR_2D: f64 = 0.3660254037844386;
pub(crate) const UNSKEW_FACTOR_2D: f64 = 0.21132486540518713;

pub(crate) const GRADIENT_LUT_2D: [[f64; 2]; 4] = [[0.0, -1.0], [-1.0, 0.0], [0.0, 1.0], [1.0, 0.0]];
pub(crate) const GRADIENT_LUT_3D: [[f64; 3]; 12] = [
    [0.0, -1.0, -1.0],
    [-1.0, 0.0, -1.0],
    [-1.0, -1.0, 0.0],
    [0.0, 1.0, -1.0],
    [1.0, 0.0, -1.0],
    [1.0, -1.0, 0.0],
    [0.0, -1.0, 1.0],
    [-1.0, 0.0, 1.0],
    [-1.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 0.0],
];
pub(crate) const GRADIENT_LUT_4D: [[f64; 4]; 32] = [
    [0.0, -1.0, -1.0, -1.0],
    [-1.0, 0.0, -1.0, -1.0],
    [-1.0, -1.0, 0.0, -1.0],
    [-1.0, -1.0, -1.0, 0.0],
    [0.0, 1.0, -1.0, -1.0],
    [1.0, 0.0, -1.0, -1.0],
    [1.0, -1.0, 0.0, -1.0],
    [1.0, -1.0, -1.0, 0.0],
    [0.0, -1.0, 1.0, -1.0],
    [-1.0, 0.0, 1.0, -1.0],
    [-1.0, 1.0, 0.0, -1.0],
    [-1.0, 1.0, -1.0, 0.0],
    [0.0, 1.0, 1.0, -1.0],
    [1.0, 0.0, 1.0, -1.0],
    [1.0, 1.0, 0.0, -1.0],
    [1.0, 1.0, -1.0, 0.0],
    [0.0, -1.0, -1.0, 1.0],
    [-1.0, 0.0, -1.0, 1.0],
    [-1.0, -1.0, 0.0, 1.0],
    [-1.0, -1.0, 1.0, 0.0],
    [0.0, 1.0, -1.0, 1.0],
    [1.0, 0.0, -1.0, 1.0],
    [1.0, -1.0, 0.0, 1.0],
    [1.0, -1.0, 1.0, 0.0],
    [0.0, -1.0, 1.0, 1.0],
    [-1.0, 0.0, 1.0, 1.0],
    [-1.0, 1.0, 0.0, 1.0],
    [-1.0, 1.0, 1.0, 0.0],
    [0.0, 1.0, 1.0, 1.0],
    [1.0, 0.0, 1.0, 1.0],
    [1.0, 1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0, 0.0],
];

pub(crate) const PERMUTATION_TABLE_256_DOUBLED: [usize; 512] = [
    143, 27, 65, 112, 249, 73, 151, 49, 90, 232, 149, 239, 41, 40, 142, 37, 79, 154, 54, 78, 144,
    67, 171, 4, 169, 68, 99, 0, 82, 77, 140, 58, 241, 110, 216, 106, 155, 105, 199, 226, 81, 159,
    80, 211, 227, 125, 95, 1, 121, 252, 52, 234, 202, 102, 71, 193, 131, 219, 101, 220, 158, 231,
    38, 255, 238, 69, 25, 243, 246, 120, 130, 209, 57, 23, 93, 62, 233, 212, 128, 21, 172, 170, 14,
    115, 24, 161, 97, 114, 184, 28, 129, 118, 204, 221, 164, 7, 237, 51, 76, 18, 56, 72, 89, 136,
    251, 153, 84, 245, 213, 35, 34, 31, 146, 254, 253, 200, 139, 39, 20, 55, 224, 162, 94, 156, 45,
    108, 36, 173, 122, 19, 174, 3, 177, 8, 208, 248, 138, 75, 152, 60, 86, 22, 63, 225, 250, 229,
    12, 85, 29, 176, 242, 207, 111, 178, 191, 9, 91, 190, 163, 2, 194, 188, 5, 104, 132, 48, 116,
    133, 214, 113, 168, 83, 43, 70, 46, 109, 92, 222, 33, 223, 198, 137, 210, 175, 185, 15, 124,
    61, 240, 127, 117, 197, 119, 166, 13, 160, 74, 134, 182, 59, 196, 88, 44, 236, 244, 218, 180,
    165, 183, 247, 217, 135, 187, 107, 16, 100, 181, 157, 150, 123, 87, 50, 206, 42, 103, 147, 96,
    98, 11, 47, 228, 26, 10, 32, 66, 235, 201, 230, 126, 179, 148, 192, 53, 17, 189, 30, 64, 215,
    145, 167, 6, 203, 195, 141, 186, 205, 143, 27, 65, 112, 249, 73, 151, 49, 90, 232, 149, 239,
    41, 40, 142, 37, 79, 154, 54, 78, 144, 67, 171, 4, 169, 68, 99, 0, 82, 77, 140, 58, 241, 110,
    216, 106, 155, 105, 199, 226, 81, 159, 80, 211, 227, 125, 95, 1, 121, 252, 52, 234, 202, 102,
    71, 193, 131, 219, 101, 220, 158, 231, 38, 255, 238, 69, 25, 243, 246, 120, 130, 209, 57, 23,
    93, 62, 233, 212, 128, 21, 172, 170, 14, 115, 24, 161, 97, 114, 184, 28, 129, 118, 204, 221,
    164, 7, 237, 51, 76, 18, 56, 72, 89, 136, 251, 153, 84, 245, 213, 35, 34, 31, 146, 254, 253,
    200, 139, 39, 20, 55, 224, 162, 94, 156, 45, 108, 36, 173, 122, 19, 174, 3, 177, 8, 208, 248,
    138, 75, 152, 60, 86, 22, 63, 225, 250, 229, 12, 85, 29, 176, 242, 207, 111, 178, 191, 9, 91,
    190, 163, 2, 194, 188, 5, 104, 132, 48, 116, 133, 214, 113, 168, 83, 43, 70, 46, 109, 92, 222,
    33, 223, 198, 137, 210, 175, 185, 15, 124, 61, 240, 127, 117, 197, 119, 166, 13, 160, 74, 134,
    182, 59, 196, 88, 44, 236, 244, 218, 180, 165, 183, 247, 217, 135, 187, 107, 16, 100, 181, 157,
    150, 123, 87, 50, 206, 42, 103, 147, 96, 98, 11, 47, 228, 26, 10, 32, 66, 235, 201, 230, 126,
    179, 148, 192, 53, 17, 189, 30, 64, 215, 145, 167, 6, 203, 195, 141, 186, 205,
];