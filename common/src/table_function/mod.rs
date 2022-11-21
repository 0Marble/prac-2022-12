pub mod error;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use crate::function::Function;

use self::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct TableFunction {
    sorted_table: Vec<(f64, f64)>,
}

impl TableFunction {
    pub fn from_table(mut table: Vec<(f64, f64)>) -> Self {
        table.sort_by(|(x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());

        Self {
            sorted_table: table,
        }
    }

    pub fn from_read<R>(src: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let f = BufReader::new(src);

        let mut table = vec![];

        for (line, l) in f.lines().enumerate() {
            let l = l?;
            let mut split = l.split(',').take(2);
            let x = split
                .next()
                .ok_or(Error::InvalidCsv { line })?
                .parse::<f64>()
                .map_err(|_| Error::InvalidCsv { line })?;
            let y = split
                .next()
                .ok_or(Error::InvalidCsv { line })?
                .parse::<f64>()
                .map_err(|_| Error::InvalidCsv { line })?;

            table.push((x, y))
        }

        Ok(Self::from_table(table))
    }

    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let f = File::open(path)?;
        Self::from_read(f)
    }
}

fn larp(min_x: f64, max_x: f64, x: f64, from_y: f64, to_y: f64) -> f64 {
    let t = (x - min_x) / (max_x - min_x);
    from_y * (1.0 - t) + to_y * t
}

impl Function for TableFunction {
    type Error = Error;
    fn apply(&self, arg: f64) -> Result<f64, Self::Error> {
        if self.sorted_table.is_empty() {
            return Err(Error::TableEmpty);
        }

        for i in 1..self.sorted_table.len() {
            let (x, y) = self.sorted_table[i];
            let (prev_x, prev_y) = self.sorted_table[i - 1];

            if prev_x <= arg && x >= arg {
                return Ok(larp(prev_x, x, arg, prev_y, y));
            }
        }

        Err(Error::PointOutOfBounds {
            x: arg,
            min: self.sorted_table.first().unwrap().0,
            max: self.sorted_table.last().unwrap().0,
        })
    }
}

#[test]
fn table_function() -> Result<(), Error> {
    let src = "0.1,1\n0.2,2\n0.3,3\n0.4,4";
    let func = TableFunction::from_read(src.as_bytes())?;

    assert_eq!(
        &func.sorted_table,
        &vec![(0.1, 1.0), (0.2, 2.0), (0.3, 3.0), (0.4, 4.0)]
    );

    assert_eq!(func.apply(0.2), Ok(2.0));
    assert_eq!(func.apply(0.15), Ok(1.5));
    assert_eq!(
        func.apply(1.0),
        Err(Error::PointOutOfBounds {
            x: 1.0,
            min: 0.1,
            max: 0.4
        })
    );
    assert_eq!(
        func.pts_to_str(&[0.1, 0.2, 0.3]),
        Ok("0.1,1\n0.2,2\n0.3,3\n".to_string())
    );

    assert!(TableFunction::from_read("0.1,1\n0.2,2\n0.3".as_bytes()).is_err());

    Ok(())
}
