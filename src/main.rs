fn main() {
    let nums: Vec<u16> = include_bytes!("../challenge.bin")
        .chunks(2)
        .map(|n| u16::from_le_bytes(n.try_into().expect("uneven")))
        .collect();
}
