#[cfg(test)]
#[path = "../src/failure_models/models.rs"]
mod model;

#[test]
fn test_mean_of_threeweibull() {
    let mean = model::mean_of_threeweibull(2.0, 1.5, 1.0);
    assert!(
        (mean - 3.5156).abs() < 1e-4,
        "Mean calculation is incorrect"
    );
}

#[test]
fn test_variance_of_threeweibull() {
    let variance = model::variance_of_threeweibull(2.0, 1.5, 1.0);
    assert!(
        (variance - 1.3754).abs() < 1e-4,
        "Variance calculation is incorrect"
    );
}

#[test]
fn test_inverse_of_threeweibull() {
    let quantile = model::inverse_of_threeweibull(0.95, 2.0, 1.5, 1.0);
    assert!(
        (quantile - 4.4295).abs() < 1e-4,
        "Quantile calculation is incorrect"
    );
}

#[test]
fn test_threeweibullcdf() {
    let cdf = model::threeweibullcdf(5, 2, 1, 1);
    assert!((cdf - 0.0183).abs() < 1e-4, "CDF calculation is incorrect");
}

#[test]
fn test_threeweibullpdf() {
    let pdf = model::threeweibullpdf(5, 2, 1, 1);
    assert!((pdf - 0.0183).abs() < 1e-4, "PDF calculation is incorrect");
}

#[test]
fn test_mean_of_inverse() {
    let mean_inv = model::mean_of_inverse(5);
    assert!(
        (mean_inv - 0.5).abs() < 1e-4,
        "Mean of inverse chi-square is incorrect"
    );
}

#[test]
fn test_variance_of_inversechi() {
    let variance_inv = model::variance_of_inversechi(5);
    assert!(
        (variance_inv - 0.5).abs() < 1e-4,
        "Variance of inverse chi-square is incorrect"
    );
}
