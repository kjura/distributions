use std::f64::consts::PI;
use plotly::common::Mode;
use plotly::{Plot, Scatter};
use ndarray::Array;

fn main() {

    let mu = 0.; 
    let sigma = (0.2_f64).sqrt();

    let generate_xs: Vec<f64> = Array::linspace(-5., 5., 300).to_vec();
    let generate_ys = generate_xs.clone().into_iter().map(|x| normal_pdf(x, sigma, mu)).collect();

    let trace1 = Scatter::new(
        generate_xs,
        generate_ys
    )
    .name("Linear function")
    .mode(Mode::Lines);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.show();

}


fn normal_pdf(x: f64, sigma: f64, mu: f64) -> f64 {
    
    let scaler = 1.0 / ((2.0 * PI * sigma.powi(2)).sqrt());
    let exponent_power = - (x - mu).powi(2) / (2.0 * sigma.powi(2));
    scaler * exponent_power.exp()
}
