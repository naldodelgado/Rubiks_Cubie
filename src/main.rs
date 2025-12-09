use rand::Rng;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Cubie{
    pub corners_pos : [u8;8],
    pub corners_ori: [u8;8],
    pub edges_pos : [u8;12],
    pub edges_ori : [u8;12]
}

impl Default for Cubie {
    fn default() -> Self {
        Self {
            corners_pos: [0,1,2,3,4,5,6,7],
            corners_ori: [0; 8],
            edges_pos:   [0,1,2,3,4,5,6,7,8,9,10,11],
            edges_ori:   [0; 12],
        }
    }
}

impl Cubie {
    pub fn is_solved(&self) -> bool {
        self == &Cubie::default()
    }

    pub fn randomize(&mut self, depth: usize) {
        let mut rng = rand::rng();

        for _ in 0..depth {
            let mv = rng.random_range(0..10);

            match mv {
                0 => self.R_plus(),
                1 => self.R_minus(),
                2 => self.L_plus(),
                3 => self.L_minus(),
                4 => self.U_plus(),
                5 => self.U_minus(),
                6 => self.D_plus(),
                7 => self.D_minus(),
                8 => self.B_plus(),
                9 => self.B_minus(),
                _ => unreachable!(),
            }
        }
    }

    pub fn U_plus(&mut self) {
        cycle4(&mut self.corners_pos, 0,1,2,3);
        cycle4(&mut self.corners_ori, 0,1,2,3);

        cycle4(&mut self.edges_pos, 0,1,2,3);
        cycle4(&mut self.edges_ori, 0,1,2,3);
    }
    pub fn U_minus(&mut self) { self.U_plus(); self.U_plus(); self.U_plus(); }

    pub fn D_plus(&mut self) {
        cycle4(&mut self.corners_pos, 4,5,6,7);
        cycle4(&mut self.corners_ori, 4,5,6,7);

        cycle4(&mut self.edges_pos, 4,5,6,7);
        cycle4(&mut self.edges_ori, 4,5,6,7);
    }
    pub fn D_minus(&mut self) { self.D_plus(); self.D_plus(); self.D_plus(); }

    pub fn R_plus(&mut self) {
        // corner perm
        cycle4(&mut self.corners_pos, 1,2,6,5);
        // corner orientation twists
        add_corner_ori(&mut self.corners_ori, [1,2,6,5], [1,2,1,2]);

        // edge perm
        cycle4(&mut self.edges_pos, 1,9,5,8);
        // R face does not flip edges
    }
    pub fn R_minus(&mut self) { self.R_plus(); self.R_plus(); self.R_plus(); }

    pub fn L_plus(&mut self) {
        // corner perm
        cycle4(&mut self.corners_pos, 0,3,7,4);
        add_corner_ori(&mut self.corners_ori, [0,3,7,4], [2,1,2,1]);

        // edges
        cycle4(&mut self.edges_pos, 3,11,7,10);
        // L does not flip edges
    }
    pub fn L_minus(&mut self) { self.L_plus(); self.L_plus(); self.L_plus(); }

    pub fn B_plus(&mut self) {
        // corner perm
        cycle4(&mut self.corners_pos, 2,3,7,6);
        add_corner_ori(&mut self.corners_ori, [2,3,7,6], [1,2,1,2]);

        // edges
        cycle4(&mut self.edges_pos, 2,10,6,9);

        // B flips edges
        for &i in &[2,10,6,9] {
            self.edges_ori[i] ^= 1;
        }
    }
    pub fn B_minus(&mut self) { self.B_plus(); self.B_plus(); self.B_plus(); }
}

fn cycle4<T: Copy>(arr: &mut [T], a: usize, b: usize, c: usize, d: usize) {
    let tmp = arr[a];
    arr[a] = arr[d];
    arr[d] = arr[c];
    arr[c] = arr[b];
    arr[b] = tmp;
}

fn add_corner_ori(arr: &mut [u8], idx: [usize; 4], vals: [u8; 4]) {
    for i in 0..4 {
        arr[idx[i]] = (arr[idx[i]] + vals[i]) % 3;
    }
}

fn main() {
    // randomize the cube
    let mut cube = Cubie::default();

    let mut counter = 0;

    loop{
        cube.R_plus();
        cube.D_plus();
        cube.L_plus();
        cube.U_plus();
        counter+=4;
        if (cube.is_solved() || counter >1000000) {
            break;
        }
    }

    println!("Cube Solved after {} moves, you just had to repeat the pattern {} times;", counter, counter/4)
}
