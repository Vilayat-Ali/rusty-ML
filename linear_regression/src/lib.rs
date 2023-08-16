pub struct LinearRegression<'a> {
    pub x_data_points: &'a Box<[f64]>,
    pub y_data_points: &'a Box<[f64]>,
    pub mse: f64,
}

pub trait LinearRegressionAlgorithm {
    fn gradient_descent(&self, current_m: f64, current_b: f64, learning_rate: f64) -> (f64, f64);
    fn train(&self);
    fn predict(&self);
}

impl<'a> LinearRegressionAlgorithm for LinearRegression<'a> {
    fn gradient_descent(&self, current_m: f64, current_c: f64, learning_rate: f64) -> (f64, f64) {
        /*
            Returns:
             (new slope (m), new y-intercept (c))
        */

        let mut gradient_m: f64 = 0_f64;
        let mut gradient_c: f64 = 0_f64;

        for i in 0..self.x_data_points.len() {
            let x_val: f64 = self.x_data_points[i];
            let y_val: f64 = self.y_data_points[i];
            let size: f64 = self.x_data_points.len() as f64;

            // updating gradient values
            gradient_m +=
                ((-2.0 / size) * x_val * (y_val - ((current_m * x_val) + current_c))) as f64;
            gradient_c += ((-2.0 / size) * (y_val - ((current_m * x_val) + current_c))) as f64;
        }
        (
            (current_m - (gradient_m * learning_rate)),
            (current_c - (gradient_c * learning_rate)),
        )
    }

    fn train(&self) {}

    fn predict(&self) {}
}

impl<'a> LinearRegression<'a> {
    pub fn new(x: &'a Box<[f64]>, y: &'a Box<[f64]>) -> Self {
        if x.len() != y.len() {
            panic!("X and Y data points should be of same size!");
        }
        Self {
            x_data_points: x,
            y_data_points: y,
            mse: 0.0,
        }
    }

    fn cost_function(&self, m: f64, c: f64) -> f64 {
        /*
           Cost Function: MEAN SQUARE ERROR
           --------------------------------

           MSE = 1/N * (y - yi)^2
           N => Total number of data points
           y => actual value on Y axis
           yi => predicted value of variable on Y axis
        */
        let mut total_error: f64 = 0.0;
        for i in 0..self.x_data_points.len() {
            total_error += (&self.y_data_points[i] - ((m * &self.x_data_points[i]) + c)).powi(2);
        }
        total_error / (self.x_data_points.len() as f64)
    }
}
