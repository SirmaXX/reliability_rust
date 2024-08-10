use statrs::function::gamma::gamma;
use std::f64;

pub fn average_av(mttf: i32, mttr: i32) -> f64 {
    let average = mttf as f64 / (mttf + mttr) as f64;
    println!("{}", average);
    average
}

// Three-parameter Weibull distribution CDF
pub fn threeweibullcdf(tvalue: i32, alpha: i32, beta: i32, eta: i32) -> f64 {
    assert!(
        alpha >= 0 && beta >= 0 && eta >= 0,
        "Parameters alpha, beta, and eta must be non-negative"
    );

    if tvalue > alpha {
        let cdfvalue = (-((tvalue - alpha) as f64 / eta as f64).powf(beta as f64)).exp();
        cdfvalue
    } else {
        0.0
    }
}

// Three-parameter Weibull distribution PDF
pub fn threeweibullpdf(tvalue: i32, alpha: i32, beta: i32, eta: i32) -> f64 {
    assert!(
        alpha >= 0 && beta >= 0 && eta >= 0,
        "Parameters alpha, beta, and eta must be non-negative"
    );

    if tvalue > alpha {
        let pdfvalue = (beta as f64 / eta as f64)
            * ((tvalue - alpha) as f64 / eta as f64).powf(beta as f64 - 1.0)
            * (-((tvalue - alpha) as f64 / eta as f64).powf(beta as f64)).exp();
        pdfvalue
    } else {
        0.0
    }
}

// Mean of Three-Parameter Weibull distribution
pub fn mean_of_threeweibull(alpha: f64, beta: f64, eta: f64) -> f64 {
    assert!(
        alpha >= 0.0 && beta >= 0.0 && eta >= 0.0,
        "Parameters alpha, beta, and eta must be non-negative"
    );

    eta * gamma(1.0 + 1.0 / beta) + alpha
}

// Variance of Three-Parameter Weibull distribution
pub fn variance_of_threeweibull(alpha: f64, beta: f64, eta: f64) -> f64 {
    assert!(
        alpha >= 0.0 && beta >= 0.0 && eta >= 0.0,
        "Parameters alpha, beta, and eta must be non-negative"
    );

    let gamma1 = gamma(1.0 + 1.0 / beta);
    let gamma2 = gamma(1.0 + 2.0 / beta);

    eta.powi(2) * (gamma2 - gamma1.powi(2))
}

// Inverse (Quantile) of Three-Parameter Weibull distribution
pub fn inverse_of_threeweibull(p: f64, alpha: f64, beta: f64, eta: f64) -> f64 {
    assert!(
        p >= 0.0 && p < 1.0,
        "Probability p must be in the range [0, 1)"
    );
    assert!(
        alpha >= 0.0 && beta >= 0.0 && eta >= 0.0,
        "Parameters alpha, beta, and eta must be non-negative"
    );

    alpha + eta * (-((1.0 - p).ln())).powf(1.0 / beta)
}

// Mean of Inverse Chi-Square Distribution
pub fn mean_of_inverse(v: i32) -> f64 {
    if v > 2 {
        1.0 / (v as f64 - 2.0)
    } else {
        0.0
    }
}

// Variance of Inverse Chi-Square Distribution
pub fn variance_of_inversechi(v: i32) -> f64 {
    if v > 4 {
        2.0 / ((v as f64 - 2.0).powi(2) * (v as f64 - 4.0))
    } else {
        0.0
    }
}