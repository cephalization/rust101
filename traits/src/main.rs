pub trait Addition {
    fn add(&self) -> i32;
}

impl Addition for (i32, i32) {
    fn add(&self) -> i32 {
        &self.0 + &self.1
    }
}

fn main() {
    println!("{}", (1, 2).add());
}
