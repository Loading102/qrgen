#[derive(Clone)]
enum Module {
    Data(bool),
    Constant(bool),
    QuietZone,
    Unset
}

fn get_pixel(point: &Module) -> bool {
    match point {
        Module::Data(pixel) => *pixel,
        Module::Constant(pixel) => *pixel,
        Module::QuietZone => true,
        Module::Unset => false// panic!("Encountered unset modules in the qr code!")
    }
}

fn main() {
    let version = 1;
    let size = (version*4) + 1 + 2*8 + 2*4; // data + 2*finder pattern + 2*quiet zone
    let mut qr = vec![vec![Module::Unset; size]; size];
    
    // +++ CONSTRUCTING THE STATIC ELEMENTS +++
    // add finder patterns
    for (i, j) in [(3, 3), (size-12, 3), (3, size-12)] {
        for k in 0..8 {
            qr[i+k][j] = Module::Constant(true);
            qr[i][j+k+1] = Module::Constant(true);
            qr[i+k+1][j+8] = Module::Constant(true);
            qr[i+8][j+k] = Module::Constant(true);
        }
        for k in 0..6 {
            qr[i+k+1][j+1] = Module::Constant(false);
            qr[i+1][j+k+2] = Module::Constant(false);
            qr[i+k+2][j+7] = Module::Constant(false);
            qr[i+7][j+k+1] = Module::Constant(false);
        }
        for k in 0..4 {
            qr[i+k+2][j+2] = Module::Constant(true);
            qr[i+2][j+k+3] = Module::Constant(true);
            qr[i+k+3][j+6] = Module::Constant(true);
            qr[i+6][j+k+2] = Module::Constant(true);
        }
    }
    // add quiet zones
    for col in 0..size {
        for row in 0..4 {
            // pattern of four bolcks on each side
            qr[col][row] = Module::QuietZone;
            qr[col][size-row-1] = Module::QuietZone;
            qr[row][col] = Module::QuietZone;
            qr[size-row-1][col] = Module::QuietZone;
        }
    }

    
    // display the qr code
    //      add padding for displaying properly
    qr.push(vec![Module::QuietZone; size]);
    //      display the actual modules (pixels)
    for i in 0..(size)/2 {
        for j in 0..size {
            print!( 
                "{}", 
                match (get_pixel(&qr[i*2][j]), get_pixel(&qr[i*2+1][j])) {
                    (true, true) => "█",
                    (true, false) => "▀",
                    (false, true) => "▄",
                    (false, false) => " "
                }
            );
        }
        print!("\n");
    }
}