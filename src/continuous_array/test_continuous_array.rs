use std::fs::File;
use std::io::Write;

pub fn test_continuous_array(name: &str, mut array: Vec<i32>, answer: usize) -> Box<[u8]> {
    use crate::continuous_array;
    use std::time;

    let start = time::Instant::now();

    array.sort();
    let correct = continuous_array::num_changes(&array, array.len()) == answer;

    let duration = start.elapsed();

    let length = array.len();

    let out = format!(
        "### {name}:
     passed: {correct}
     length: {length}
       time: {duration:?}

"
    );

    Box::from(out.as_bytes())
}

pub fn continuous_array_loader() {
    let mut f = File::create("test_results.md").expect("Failed to access file");

    let vec1 = vec![4, 2, 5, 3];
    let vec2 = vec![1, 2, 3, 5, 6];
    let vec3 = vec![1, 10, 100, 1000];
    let vec4 = vec![40, 15, 47, 49, 35, 13, 26];
    let vec5 = vec![
        6871, 3056, 8843, 3642, 4544, 300, 6054, 6345, 2161, 1107, 1957, 6273, 2799, 6665, 2000,
        1483, 3148, 1655, 7468, 4904, 9195, 9500, 7041, 7335, 8494, 386, 7754, 8739, 6893, 1015,
        6411, 641, 4631, 6156, 7146, 4461, 9517, 960, 7328, 5314, 3846, 9943, 59, 3310, 9637, 3296,
        7056, 4873, 5226, 3943, 5488, 4193, 8906, 259, 626, 9800, 9725, 8610, 901, 2467, 3722,
        9322, 9500, 1348, 8727, 5618, 8641, 6579, 9689, 1776, 6421, 7690, 6414, 8600, 4285, 3513,
        3210, 7290, 8918, 882, 2099, 6472, 4902, 553, 6420, 4586, 3478, 5130, 7785, 3679, 2301,
        5460, 4285, 9409, 1160, 7447, 3127, 8771, 9863, 9452, 9879, 207, 3792, 5044, 9256, 8011,
        8119, 8815, 6762, 2255, 3754, 4301, 5534, 5667, 3672, 1063, 4667, 7711, 6225, 9914, 7845,
        1180, 7461, 6807, 7219, 5071, 3185, 8433, 2367, 3397, 1608, 7779, 3893, 4172, 4470, 2793,
        502, 4781, 2659, 7601, 784, 2267, 5121, 9394, 2582, 3479, 4087, 2110, 7723, 6859, 3423,
        1512, 9040, 2479, 1704, 8963, 7801, 6430, 4575, 2263, 3525, 3325, 3491, 476, 8585, 3243,
    ];
    let vec6 = vec![
        6324, 1245, 1888, 6406, 9996, 8979, 483, 8760, 8447, 5278, 3327, 1542, 7005, 6255, 4428,
        4231, 435, 5403, 3373, 1960, 7347, 7837, 1110, 3280, 4144, 8151, 1853, 2499, 4836, 3189,
        5318, 3388, 465, 2239, 9130, 4173, 5177, 9886, 9384, 323, 1771, 3818, 995, 1616, 4907,
        7744, 4138, 5387, 620, 9968, 7914, 822, 8018, 9771, 489, 4521, 5764, 4026, 1310, 6438,
        1188, 3992, 1231, 8105, 8747, 2127, 9949, 6967, 8837, 8248, 2438, 349, 8017, 9839, 4985,
        897, 6464, 752, 9053, 1267, 7582, 797, 3318, 8974, 4642, 7092, 5712, 2891, 2756, 4995,
        5141, 4916, 8804, 5692, 4807, 5718, 5311, 369, 2767, 8169, 4204, 8735, 9421, 8605, 9097,
        216, 6822, 6122, 625, 2991, 3559, 743, 2374, 6588, 9307, 3905, 308, 6730, 1867, 4780, 4586,
        5033, 5785, 533, 7899, 3953, 8128, 5794, 2395, 9911, 1605, 1674, 8201, 7160,
    ];

    f.write_all(&test_continuous_array("1", vec1, 0))
        .expect("Failed to write to file");
    f.write_all(&test_continuous_array("2", vec2, 1))
        .expect("Failed to write to file");
    f.write_all(&test_continuous_array("3", vec3, 3))
        .expect("Failed to write to file");
    f.write_all(&test_continuous_array("4", vec4, 5))
        .expect("Failed to write to file");
    f.write_all(&test_continuous_array("5", vec5, 158))
        .expect("Failed to write to file");
    f.write_all(&test_continuous_array("6", vec6, 128))
        .expect("Failed to write to file");

    big_test(f);
}

pub fn big_test(mut f: File) {
    use crate::inputs;

    let vec_input = inputs::fucking_massive_vec();

    f.write_all(&test_continuous_array("BIG", vec_input, 99989))
        .expect("Failed to write to file");
}
