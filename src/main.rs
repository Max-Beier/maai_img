use maai_core::Maai;

fn main() {
    print!("{esc}c", esc = 27 as char);
    println!(
        "STARTING MAAI IMAGE | VERSION {}",
        env!("CARGO_PKG_VERSION")
    );

    let payload: Vec<f64> = Vec::new();

    let _my_ai = Maai::new(payload);
    _my_ai.run();
}
