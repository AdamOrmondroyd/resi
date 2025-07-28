mod lib;

fn main() {
    println!("Hello, world!");
    let dh_over_rs = lib::dh_over_rs(0.5, 10000.0, 0.3);
    println!("1/H(z) at z=0.5 with Omega_m=0.3: {}", dh_over_rs);
    let dm_over_rs = lib::dm_over_rs(0.5, 10000.0, 0.3);
    println!("1/H(z) at z=0.5 with Omega_m=0.3: {}", dm_over_rs);
}
