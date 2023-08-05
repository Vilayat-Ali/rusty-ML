use util;

fn main() {
    let a = util::df::DataFrame::init_from_csv("bank.csv");
    println!("{:#?}", a.frame[0]);
}
