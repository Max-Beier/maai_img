use maai_core::Maai;

fn main() {
    print!("{esc}c", esc = 27 as char);
    println!(
        "STARTING MAAI IMAGE | VERSION {}",
        env!("CARGO_PKG_VERSION")
    );

    let payload: Vec<f64> = vec![1.0, 1.0];

    let _my_ai = Maai::new(payload, 512, 16, 4);
    _my_ai.inspect();
}
