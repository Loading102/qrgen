// use rand::Rng;
// use std::io;

#[derive(Clone)]
#[derive(PartialEq)]
enum Module {
    Data(bool),
    Meta(bool),
    Constant(bool),
    Unset
}

fn get_pixel(point: &Module) -> bool {
    match point {
        Module::Data(pixel) => *pixel,
        Module::Meta(pixel) => *pixel,
        Module::Constant(pixel) => *pixel,
        // Module::Unset => true
        // Module::Unset => false
        Module::Unset => panic!("Encountered unset modules in the qr code!")
    }
}

fn new_qr_code(version: usize) -> Vec<Vec<Module>> {
    let alignment_pattern_distance = [1, 1, 1, 1, 1, 1, 16, 18, 20, 22, 24, 26, 28, 20, 22, 24, 24, 26, 28, 28, 22, 24, 24, 26, 26, 28, 28, 24, 24, 26, 26, 26, 26, 26, 24, 26, 26, 26, 28, 28];
    // zaczyna się od version 7
    let verinf = [0x07C94, 0x085BC, 0x09A99, 0x0A4D3, 0x0BBF6, 0x0C762, 0x0D847, 0x0E60D, 0x0F928, 0x10B78, 0x1145D, 0x12A17, 0x13532, 0x149A6, 0x15683, 0x168C9, 0x177EC, 0x18EC4, 0x191E1, 0x1AFAB, 0x1B08E, 0x1CC1A, 0x1D33F, 0x1ED75, 0x1F250, 0x209D5, 0x216F0, 0x228BA, 0x2379F, 0x24B0B, 0x2542E, 0x26A64, 0x27541, 0x28C69];

    let size = (version*4) + 1 + 2*8; // payload + 2*finder pattern + 2*quiet zone
    let mut qr = vec![vec![Module::Unset; size]; size];
    let apc = if version == 1 { 1 } else {version/7 + 2}; // root of theoretical count of alignment codes
    let alignment_pattern_distance = alignment_pattern_distance[version-1];
    let verinf = if version >= 7 { verinf[version-7] } else { 0 };
    // +++ CONSTRUCTING THE STATIC ELEMENTS +++
    // add finder patterns
    for (i, j) in [(0, 0), (size-7, 0), (0, size-7)] {
        for k in 0..9 {
            if 1<=i+k  && i+k<size+1 &&   j<size+1 && 1<=j   { qr[i+k-1][ j-1 ] = Module::Constant(false);}
            if 1<=i    &&   i<size+1 && j+k<size/*&& 0<=j+k*/{ qr[ i-1 ][ j+k ] = Module::Constant(false);}
            if/*0<=i+k&&*/ i+k<size  && j+7<size/*&& 0<=j+7*/{ qr[ i+k ][ j+7 ] = Module::Constant(false);}
            if/*0<=i+7&&*/ i+7<size  && j+k<size+1 && 1<=j+k { qr[ i+7 ][j+k-1] = Module::Constant(false);}
        }
        for k in 0..7 {
            for l in 0..7 {
                qr[i+k][j+l] = Module::Constant(true);
            }
        }
        for k in 2..6 {
            qr[i+k-1][j+1] = Module::Constant(false);
            qr[i+1][j+k] = Module::Constant(false);
            qr[i+k][j+5] = Module::Constant(false);
            qr[i+5][j+k-1] = Module::Constant(false);
        }
    }
    // add timing patterns
    for i in 3..(size-6)/2 {
        qr[i*2][6] = Module::Constant(true);
        qr[i*2+1][6] = Module::Constant(false);
        qr[6][i*2] = Module::Constant(true);
        qr[6][i*2+1] = Module::Constant(false);
    }
    // add alignment patterns
    {
        fn mark_alignment_pattern( qr: &mut Vec<Vec<Module>>, x:usize, y:usize) {

            for i in x-2..=x+2 {
                for j in y-2..=y+2 {
                    qr[i][j] = Module::Constant(true);
                }
            }
            for i in x-1..=x+1 {
                for j in y-1..=y+1 {
                    qr[i][j] = Module::Constant(false);
                }
            }
            qr[x][y] = Module::Constant(true);
        
        }
        let padding = size-7;
        for i in 0..apc-1 {
            for j in 0..apc-1 {
                mark_alignment_pattern(&mut qr, padding-i*alignment_pattern_distance, padding-j*alignment_pattern_distance);
            }
        }
        for i in 1..apc-1 {
            mark_alignment_pattern(&mut qr, size-padding-1, padding-i*alignment_pattern_distance);
            mark_alignment_pattern(&mut qr, padding-i*alignment_pattern_distance, size-padding-1);
        }
    }

    // +++ CONSTRUCTING THE METADATA ELEMENTS
    // reserve space for format information
    {
        for i in 0..6 {
            qr[i][8] = Module::Meta(false);
            qr[8][i] = Module::Meta(false);
        }
        qr[7][8] = Module::Meta(false);
        qr[8][7] = Module::Meta(false);
        qr[8][8] = Module::Meta(false);
        for i in size-8..size {
            qr[i][8] = Module::Meta(false);
            qr[8][i] = Module::Meta(false);
        }
        qr[size-8][8] = Module::Constant(true)

    }
    // mark the version information
    if version >= 7 {
        for i in 0..6 {
            qr[size-11][i] = Module::Meta(0 != (verinf & (1<<(i*3+0))));
            qr[size-10][i] = Module::Meta(0 != (verinf & (1<<(i*3+1))));
            qr[size- 9][i] = Module::Meta(0 != (verinf & (1<<(i*3+2))));

            qr[i][size-11] = Module::Meta(0 != (verinf & (1<<(i*3+0))));
            qr[i][size-10] = Module::Meta(0 != (verinf & (1<<(i*3+1))));
            qr[i][size- 9] = Module::Meta(0 != (verinf & (1<<(i*3+2))));
        }
    }

    qr  // return the base qr code
}

fn write_payload_to_qr(qr: &mut Vec<Vec<Module>>, payload: Vec<u8>) {
    let size = qr.len();
    let version = (size-1-2*8)/4;
    let mut bitwise_pointer = 7;
    let mut bytewise_pointer = 0;
    if payload.len() != get_capacity(&version) {
        panic!("Payload size different than the qr code version allows!");
    }
    let mut out_of_bounds = false;
    let mut up = true;

    for i in 1..=size {
        if (i%2==1) == (size-i<=7) {continue;}
        for j in 0..size {
            let j = if up {size-j-1} else {j};
            for k in 0..=1 {
                if qr[j][size-i-k] == Module::Unset {
                    if !out_of_bounds {
                        qr[j][size-i-k] = Module::Data(payload[bytewise_pointer] & (1<<bitwise_pointer) != 0);
                        if bitwise_pointer == 0 {
                            bitwise_pointer = 8;
                            bytewise_pointer += 1;
                            if bytewise_pointer >= payload.len() {
                                out_of_bounds = true;
                            }
                        }
                        bitwise_pointer -= 1;
                    } else {
                        qr[j][size-i-k] = Module::Data(false);
                    }
                }
            }
        }
        up = !up;
    }

}

fn get_capacity(version: &usize) -> usize {
    if *version == 1 {
        26
    } else if *version < 7 {
        ((version*4+17)*(version*4+17)-3*8*8-5*5-2*(version*4+1)-31)/8
    } else {
        ((version*4+17)*(version*4+17)-3*8*8-2*(version*4+1)-67-((version/7 + 2)*(version/7 + 2)-3)*5*5 + (version/7)*2*5)/8
    }
}

fn set_error_correction_level(qr: &mut Vec<Vec<Module>>, error_correction_level: usize) {
    let size = qr.len();
    qr[8][0]      = Module::Meta((error_correction_level-1) & 2 == 0);
    qr[size-1][8] = Module::Meta((error_correction_level-1) & 2 == 0);
    qr[8][1]      = Module::Meta((error_correction_level-1) & 1 != 0);
    qr[size-2][8] = Module::Meta((error_correction_level-1) & 1 != 0);
}

fn write_mask(qr: &mut Vec<Vec<Module>>, mask: usize) { // this operation with the same parameters is its own opposite
    fn flip_data_module(modu: &mut Module) {
        if let Module::Data(val) = modu {*val = !*val}
    }    
    let size = qr.len();
    for i in 0..size {
        for j in 0..size {
            if match mask {
                0 => {(i+j)%2 == 0},
                1 => {i%2 == 0},
                2 => {j%3 == 0},
                3 => {(i+j)%3 == 0},
                4 => {((i/2)+(j/3))%2==0},
                5 => {(i*j)%2+(i*j)%3==0},
                6 => {((i*j)%2+(i*j)%3)%2==0},
                7 => {((i+j)%2+(i*j)%3)%2==0},
                _ => panic!("Invalid mask!")
            } {flip_data_module(&mut qr[i][j]);}
        }
    }

    fn flip_meta_module(condition: bool, modu: &mut Module) {
        if let Module::Meta(val) = modu {*val = *val^condition;}
    }
    let ecl = {
        (if let Module::Meta(val) = qr[8][0] {if val {0} else {2}} else {panic!("expected Module::Meta at qr[8][0]")}) + 
        (if let Module::Meta(val) = qr[8][1] {if val {1} else {0}} else {panic!("expected Module::Meta at qr[8][1]")})
    };
    let foinf = [0x5412, 0x5125, 0x5E7C, 0x5B4B, 0x45F9, 0x40CE, 0x4F97, 0x4AA0, 0x77C4, 0x72F3, 0x7DAA, 0x789D, 0x662F, 0x6318, 0x6C41, 0x6976, 0x1689, 0x13BE, 0x1CE7, 0x19D0, 0x0762, 0x0255, 0x0D0C, 0x083B, 0x355F, 0x3068, 0x3F31, 0x3A06, 0x24B4, 0x2183, 0x2EDA, 0x2BED];
    let foinf = foinf[ecl*8+mask];
    for i in 0..=5 {
        flip_meta_module(foinf & 1<<i != 0, &mut qr[i][8]);
        flip_meta_module(foinf & 1<<i != 0, &mut qr[8][size-1-i]);
    }
    for i in 6..=7 {
        flip_meta_module(foinf & 1<<i != 0, &mut qr[i+1][8]);
        flip_meta_module(foinf & 1<<i != 0, &mut qr[8][size-1-i]);
    }
    flip_meta_module(foinf & 1<<8 != 0, &mut qr[8][7]);
    flip_meta_module(foinf & 1<<8 != 0, &mut qr[size-7][8]);
    for i in 9..=12 {
        flip_meta_module(foinf & 1<<i != 0, &mut qr[8][14-i]);
        flip_meta_module(foinf & 1<<i != 0, &mut qr[size-15+i][8]);
    }
    // we don't write the error correction level data, since it's already there

}

fn get_minimal_version(data_size_in_bytes:usize, error_correction_level: usize) -> usize {
    let ecl_version_raw_capacity = [
        [19, 34, 55, 80, 108, 136, 156, 194, 232, 274, 324, 370, 428, 461, 523, 589, 647, 721, 795, 861, 932, 1006, 1094, 1174, 1276, 1370, 1468, 1531, 1631, 1735, 1843, 1955, 2071, 2191, 2306, 2434, 2566, 2702, 2812, 2956],
        [16, 28, 44, 64, 86, 108, 124, 154, 182, 216, 254, 290, 334, 365, 415, 453, 507, 563, 627, 669, 714, 782, 860, 914, 1000, 1062, 1128, 1193, 1267, 1373, 1455, 1541, 1631, 1725, 1812, 1914, 1992, 2102, 2216, 2334],
        [13, 22, 34, 48, 62, 76, 88, 110, 132, 154, 180, 206, 244, 261, 295, 325, 367, 397, 445, 485, 512, 568, 614, 664, 718, 754, 808, 871, 911, 985, 1033, 1115, 1171, 1231, 1286, 1354, 1426, 1502, 1582, 1666],
        [9, 16, 26, 36, 46, 60, 66, 86, 100, 122, 140, 158, 180, 197, 223, 253, 283, 313, 341, 385, 406, 442, 464, 514, 538, 596, 628, 661, 701, 745, 793, 845, 901, 961, 986, 1054, 1096, 1142, 1222, 1276],
    ];
    let version_raw_capacity = ecl_version_raw_capacity[error_correction_level-1];
    if version_raw_capacity[39] < data_size_in_bytes && error_correction_level == 1 {panic!("No single qr code can contain this much data!")}
    if version_raw_capacity[39] < data_size_in_bytes {panic!("No single qr code at this error correction level can contain this much data!")}
    let mut version = 63;
    for i in 0..=5 {
        let i = 5-i;
        version ^= 1<<i;
        if version > 39 {continue;}
        if version_raw_capacity[version] < data_size_in_bytes {
            version ^= 1<<i;
        }
    }
    version+1
}

fn main() {
    // let mut version = String::new();
    // io::stdin().read_line(&mut version).expect("Should be an intiger from between 1 and 40 (inclusive)");
    // let version: u8 = version.trim().parse().expect("Should be an intiger from between 1 and 40 (inclusive)");
    
    let data: Vec<i8> = vec![0; 2956];
    let ecl = 1; // (1..=4)
    let mask = 3; // (0..=7)
    // let qz = 4; // quiet zone size, currently broken if not 4
    
    
    
    
    
    // generate a new qr code
    let version = get_minimal_version(data.len(), ecl); // (1..=40)

    println!("{version}");

    let mut qr: Vec<Vec<Module>> = new_qr_code(version);
    set_error_correction_level(&mut qr, ecl);
    // generate some payload
    let payload: Vec<u8> = vec![0; get_capacity(&version)];
    write_payload_to_qr(&mut qr, payload);
    write_mask(&mut qr, mask);

    // display the qr code
    let size = (version*4) + 1 + 2*8;
    //      add padding for vertical parity
    qr.push(vec![Module::Constant(true); size]);
    for i in 0..(size+1)/2 {
        for j in 0..size {
            print!("{}", 
                match (get_pixel(&qr[i*2][j]), get_pixel(&qr[i*2+1][j])) {
                    (false, false) => "█",
                    (false, true) => "▀",
                    (true, false) => "▄",
                    (true, true) => " "
                }
            );
        }
        print!("\n");
    }
}