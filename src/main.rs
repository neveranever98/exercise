use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
fn main() {
    println!("Hello, world!");
    let mut r = StdRng::seed_from_u64(42);
    let mut y = [1, 2, 3, 4, 5];
    println!("Unshuffled: {:?}", y);
    y[0..2].shuffle(&mut r);
    println!("Shuffled:   {:?}", y);
    
}
