mod failure_models {
    pub mod models;
    pub mod nonparametric_model;
}

fn main() {
    // Example usage
    let mean = failure_models::models::mean_of_threeweibull(2.0, 1.5, 1.0);
    println!("Mean of three-parameter Weibull: {}", mean);

    let variance = failure_models::models::variance_of_threeweibull(2.0, 1.5, 1.0);
    println!("Variance of three-parameter Weibull: {}", variance);

    let quantile = failure_models::models::inverse_of_threeweibull(0.95, 2.0, 1.5, 1.0);
    println!("95th percentile of three-parameter Weibull: {}", quantile);

    let cdf = failure_models::models::threeweibullcdf(5, 2, 1, 1);
    println!("CDF of three-parameter Weibull: {}", cdf);

    let pdf = failure_models::models::threeweibullpdf(5, 2, 1, 1);
    println!("PDF of three-parameter Weibull: {}", pdf);

    let mean_inv = failure_models::models::mean_of_inverse(5);
    println!("Mean of inverse chi-square: {}", mean_inv);

    let variance_inv = failure_models::models::variance_of_inversechi(5);
    println!("Variance of inverse chi-square: {}", variance_inv);

    let x = vec![1.0, 2.0, 3.0];
    let y = vec![4.0, 5.0, 6.0];
    let alpha = 0.05;
    failure_models::nonparametric_model::logrank(x, y, alpha);
}
