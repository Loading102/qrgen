use std::ops;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Gf(pub u8);

impl ops::BitXor<u8> for Gf {
    type Output = Gf;
    fn bitxor(self, rhs: u8) -> Self::Output {
        Gf(self.0^rhs)
    }
}

impl PartialEq<Gf> for Gf {
    fn eq(self: &Gf, rhs: &Gf) -> bool {
        self.0 == rhs.0
    }
}

impl ops::Add<Gf> for Gf {
    type Output = Gf;
    fn add(self, rhs: Gf) -> Self::Output {
        self ^ rhs.0
    }
}

const MUL: [[Gf; 256]; 256] = {
    let mut i: u32 = 0; let mut j: u32 = 0;
    let mut tab: [[Gf; 256]; 256] = [[Gf(0); 256]; 256];
    loop {
        tab [i as usize][j as usize] = 'cell: {
            if i < j { break 'cell tab[j as usize][i as usize] } // optimizations at compile time :)
            let mut out: u32 = 0;
            let mut k: u32 = 0;
            loop {
                if (i & (1 << k)) != 0 { out ^= j<<k; }
                k += 1;
                if k >= 8 { break; }
            }
            k = 8;
            loop {
                k -= 1;
                if (out & (0b100000000 << k)) != 0 { out ^= 0b100011101<<k; }
                if k <= 0 { break; }
            }
            Gf(out as u8)
        };
        i +=1;
        if i >= 256 { i = 0; j += 1; }
        if j >= 256 { break; }
    }
    tab
};
impl ops::Mul<Gf> for Gf {
    type Output = Gf;
    fn mul(self, rhs: Gf) -> Self::Output {
        MUL[rhs.0 as usize][self.0 as usize]
    }
}

const ALOG2: [Gf; 256] = {
    let mut tab: [Gf; 256] = [Gf(0); 256];
    let mut i: u8 = 0;
    let mut cur = Gf(1);
    loop {
        tab[i as usize] = cur;
        cur = MUL[cur.0 as usize][2];
        if i >= 255 { break; }
        i+=1;
    }
    tab
};
pub const fn antilog2(uop: Gf) -> Gf {
    ALOG2[uop.0 as usize]
}

const LOG2: [Gf; 256] = {
    let mut tab: [Gf; 256] = [Gf(0); 256];
    let mut i: u8 = 0;
    loop {
        tab[ALOG2[i as usize].0 as usize] = Gf(i);
        if i >= 255 { break; }
        i+=1;
    }
    tab
};
pub const fn log2(uop: Gf) -> Gf {
    LOG2[uop.0 as usize]
}