extern crate statrs;

use statrs::distribution::{ChiSquared, ContinuousCDF}; // Import necessary traits and types

// Function to calculate the mean
pub fn mean(data: &[i32]) -> Option<f64> {
    let sum: i32 = data.iter().sum();
    let count = data.len();

    if count > 0 {
        Some(sum as f64 / count as f64)
    } else {
        None
    }
}

// Function to calculate the variance
pub fn variance(data: &[i32]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 1 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f64);
                    diff * diff
                })
                .sum::<f64>()
                / (count as f64 - 1.0); // Use n-1 for sample variance

            Some(variance)
        }
        _ => None,
    }
}

// Function to calculate the chi-square critical value
fn chi_square_critical_value(degrees_of_freedom: usize, alpha: f64) -> f64 {
    let chi_square = ChiSquared::new(degrees_of_freedom as f64).unwrap();
    chi_square.inverse_cdf(1.0 - alpha)
}

// Function to calculate the chi-square statistic (Log-Rank test)
pub fn logrank(x: Vec<f64>, y: Vec<f64>, alpha: f64) {
    assert_eq!(x.len(), y.len(), "Vectors must have the same length");

    let variance_x = variance(&x.iter().map(|&v| v as i32).collect::<Vec<i32>>()).unwrap_or(0.0);
    let mut chi_square = 0.0;

    for (i, j) in x.iter().zip(y.iter()) {
        chi_square += ((i - j).powf(2.0)) / variance_x;
        println!("x: {}, y: {}", i, j);
    }

    let critical_value = chi_square_critical_value(x.len(), alpha);
    println!("Chi-Square: {}", chi_square);
    println!("Critical Value: {}", critical_value);

    if chi_square < critical_value {
        println!("Accept the null hypothesis");
    } else {
        println!("Reject the null hypothesis");
    }
}
