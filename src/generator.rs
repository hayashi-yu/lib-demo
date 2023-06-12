use rand::Rng;
pub fn gen_ran() -> u8 {
    let mut rug = rand::thread_rng();
    let n: u8 = rng.gen();
    n
}
