mod lcdm;

fn main() {
    println!("Hello, world!");
    let dh_over_rs = lcdm::dh_over_rs(0.5, 10000.0, 0.3);
    println!("Dh/rs at z=0.5 with Omega_m=0.3: {}", dh_over_rs);
    let dm_over_rs = lcdm::dm_over_rs(0.5, 10000.0, 0.3);
    println!("Dm/rs at z=0.5 with Omega_m=0.3: {}", dm_over_rs);
    let desi_likelihood = lcdm::Likelihood::from_files(
        "data/desidr2/desidr2_mean.txt",
        "data/desidr2/desidr2_cov.txt",
    );
    let logl = desi_likelihood.logl(10000.0, 0.3);
    println!("Log-likelihood for DESI data: {}", logl);
}
