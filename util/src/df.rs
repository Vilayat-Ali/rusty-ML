use std::{fs, io::Read};

pub struct DataFrame {
    pub frame: Vec<RowDataType>,
    pub shape: (usize, usize),
    pub header: Vec<String>,
}

#[derive(Debug)]
pub enum RowDataType {
    Num(Box<[f64]>),
    Str(Box<[String]>),
}

impl std::default::Default for RowDataType {
    fn default() -> Self {
        RowDataType::Num(Box::new([0_f64; 1]))
    }
}

impl DataFrame {
    pub fn init_from_csv(path: &'static str) -> Self {
        let mut file: fs::File = fs::File::open(path).expect("Failed to locate and read file!");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file!");

        let rows: Vec<Vec<String>> = contents
            .clone()
            .split("\n")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .into_iter()
            .map(|row| {
                row.split('"')
                    .collect::<String>()
                    .split(";")
                    .map(|inner_row| inner_row.to_owned())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        let shape: (usize, usize) = (rows.len() - 1, rows[0].len());
        let mut parsed_col_vec: Vec<bool> = Vec::with_capacity(shape.1);

        for col_cell in rows[1].as_slice().to_owned().into_iter() {
            match col_cell.parse::<f64>() {
                Ok(_num_val) => {
                    parsed_col_vec.push(true);
                }
                Err(e) => {
                    parsed_col_vec.push(false);
                }
            }
        }

        let mut row_df: Vec<RowDataType> = Vec::with_capacity(shape.0);

        for col_no in 0..parsed_col_vec.len() {
            if parsed_col_vec[col_no] {
                let mut col_vec: Vec<f64> = Vec::new();
                // parsable into float
                for row_no in 1..shape.0 {
                    col_vec.push(rows[row_no][col_no].parse::<f64>().unwrap());
                }
                row_df.push(RowDataType::Num(col_vec.into_boxed_slice()));
            } else {
                let mut col_vec: Vec<String> = Vec::new();
                // not parsable into float
                for row_no in 1..shape.0 {
                    col_vec.push(rows[row_no][col_no].clone());
                }
                row_df.push(RowDataType::Str(col_vec.into_boxed_slice()));
            }
        }

        Self {
            frame: row_df,
            shape,
            header: (&rows[0]).clone(),
        }
    }
}
