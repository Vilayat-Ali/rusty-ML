use linear_regression::{self, LinearRegressionAlgorithm};
use std::io;
use util::{self, df::RowDataType};

fn main() {
    let a = util::df::DataFrame::init_from_csv("../bank.csv");
    let ages = &a.frame[0];
    let loan = &a.frame[5];
    let mut input: String = String::new();

    if let RowDataType::Num(age_data_points) = ages {
        if let RowDataType::Num(loan_data_points) = loan {
            let mut regression =
                linear_regression::LinearRegression::new(age_data_points, loan_data_points);

            regression.train(100000);
            io::stdin().read_line(&mut input).unwrap();
            let input: f64 = input.trim().parse::<f64>().unwrap();
            let pred_value: f64 = regression.predict(input);
            println!("Predicted Value: {}", pred_value);
        }
    }
}
