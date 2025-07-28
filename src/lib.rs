mod lcdm;

type Likelihood = lcdm::Likelihood;

// export likelihood to C++ interface, leave out the distances
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Likelihood;
        fn create_likelihood(data_file: &str, cov_file: &str) -> Box<Likelihood>;
        fn logl(likelihood: &Likelihood, h0rd: f64, omegam: f64) -> f64;
    }
}

pub fn create_likelihood(data_file: &str, cov_file: &str) -> Box<lcdm::Likelihood> {
    Box::new(lcdm::Likelihood::from_files(data_file, cov_file))
}

pub fn logl(likelihood: &lcdm::Likelihood, h0rd: f64, omegam: f64) -> f64 {
    likelihood.logl(h0rd, omegam)
}
