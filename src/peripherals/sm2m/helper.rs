pub fn write_bit(to: u32, dst: u32, from: u32, src: u32) -> u32 {
    let bit = (from >> src) & 1;
    let clean = to & !(1 << dst);
    clean | (bit << dst)
}
