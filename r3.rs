use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
   /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
   data: Vec<T>,
   /// Number of rows
   row: usize,
   /// Number of columns
   col: usize,
}

impl<T: Copy> Matrix<T> {
   /// Creates a new matrix of `row` rows and `col` columns, and initializes
   /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
         let mut data_vec: Vec<T> = vec![];
         for value in values {
            data_vec.push(*value);
        }
        Matrix{data: data_vec, row: row, col: col}
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        Matrix{data: Vec::new(), row: row, col: col}
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        & self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
         if self.row != rhs.row || self.col != rhs.col {panic!(); }
         let mut data_vec: Vec<T> = vec![];
         for index in 0..self.data.len() {
            data_vec.push(self.data[index] + rhs.data[index])
         }
         Matrix{data: data_vec, row: self.row, col: self.col}
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {panic!(); }
         let mut data_vec: Vec<T> = vec![];
         for index in 0..self.data.len() {
            data_vec.push(self.data[index] + rhs.data[index])
         }
         Matrix{data: data_vec, row: self.row, col: self.col}
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
         if self.row != rhs.row || self.col != rhs.col {panic!(); }
         let mut data_vec: Vec<T> = vec![];
         for index in 0..self.data.len() {
            data_vec.push(self.data[index] + rhs.data[index])
         }

         Matrix{data: data_vec, row: self.row, col: self.col}
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: &Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {panic!(); }
         let mut data_vec: Vec<T> = vec![];
         for index in 0..self.data.len() {
            data_vec.push(self.data[index] + rhs.data[index])
         }
         Matrix{data: data_vec, row: self.row, col: self.col}
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {panic!(); }
         let mut data_vec: Vec<T> = vec![];
         for index in 0..self.data.len() {
            data_vec.push(self.data[index] - rhs.data[index])
         }
         Matrix{data: data_vec, row: self.row, col: self.col}
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {panic!(); }
         let mut data_vec: Vec<T> = vec![];
         for index in 0..self.data.len() {
            data_vec.push(self.data[index] - rhs.data[index])
         }
         Matrix{data: data_vec, row: self.row, col: self.col}
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {panic!(); }
         let mut data_vec: Vec<T> = vec![];
         for index in 0..self.data.len() {
            data_vec.push(self.data[index] - rhs.data[index])
         }
         Matrix{data: data_vec, row: self.row, col: self.col}
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: &Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {panic!(); }
         let mut data_vec: Vec<T> = vec![];
         for index in 0..self.data.len() {
            data_vec.push(self.data[index] - rhs.data[index])
         }
         Matrix{data: data_vec, row: self.row, col: self.col}
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
         if self.col != rhs.row {panic!();}
         let mut data_vec: Vec<T> = vec![];
         let mut entry: Vec<T> = vec![];
         for row in 0..self.row { //iterate through left hand rows
            for column in 0..rhs.col{ //iterate through right hand cols
               for col in 0..self.col { // now multiply the corresponding element of each column of left with each row of right
                  entry.push(self.data[self.col*row + col] * rhs.data[rhs.col*col + column]);
               }
               let mut curr_entry = entry.pop().unwrap();
               while entry.len() > 0 {
                  curr_entry = curr_entry + entry.pop().unwrap();
               }
               data_vec.push(curr_entry);
            }
         }
         Matrix{data: data_vec, row: self.row, col: rhs.col}
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.col != rhs.row {panic!();}
         let mut data_vec: Vec<T> = vec![];
         let mut entry: Vec<T> = vec![];
         for row in 0..self.row { //iterate through left hand rows
            for column in 0..rhs.col{ //iterate through right hand cols
               for col in 0..self.col { // now multiply the corresponding element of each column of left with each row of right
                  entry.push(self.data[self.col*row + col] * rhs.data[rhs.col*col + column]);
               }
               let mut curr_entry = entry.pop().unwrap();
               while entry.len() > 0 {
                  curr_entry = curr_entry + entry.pop().unwrap();
               }
               data_vec.push(curr_entry);
            }
         }
         Matrix{data: data_vec, row: self.row, col: rhs.col}
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.col != rhs.row {panic!();}
         let mut data_vec: Vec<T> = vec![];
         let mut entry: Vec<T> = vec![];
         for row in 0..self.row { //iterate through left hand rows
            for column in 0..rhs.col{ //iterate through right hand cols
               for col in 0..self.col { // now multiply the corresponding element of each column of left with each row of right
                  entry.push(self.data[self.col*row + col] * rhs.data[rhs.col*col + column]);
               }
               let mut curr_entry = entry.pop().unwrap();
               while entry.len() > 0 {
                  curr_entry = curr_entry + entry.pop().unwrap();
               }
               data_vec.push(curr_entry);
            }
         }
         Matrix{data: data_vec, row: self.row, col: rhs.col}
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: &Self) -> Self::Output {
        if self.col != rhs.row {panic!();}
         let mut data_vec: Vec<T> = vec![];
         let mut entry: Vec<T> = vec![];
         for row in 0..self.row { //iterate through left hand rows
            for column in 0..rhs.col{ //iterate through right hand cols
               for col in 0..self.col { // now multiply the corresponding element of each column of left with each row of right
                  entry.push(self.data[self.col*row + col] * rhs.data[rhs.col*col + column]);
               }
               let mut curr_entry = entry.pop().unwrap();
               while entry.len() > 0 {
                  curr_entry = curr_entry + entry.pop().unwrap();
               }
               data_vec.push(curr_entry);
            }
         }
         Matrix{data: data_vec, row: self.row, col: rhs.col}
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut form_str = "".to_string();
        for rows in 0..self.row {
            for cols in 0..(self.col - 1) {
                form_str = form_str + &self.data[self.col*rows + cols].to_string();
                form_str.push(' ');
            }
            form_str = form_str + &self.data[self.col*rows + self.col-1].to_string();
            form_str.push('\n');
        }
        write!(f, "{}", form_str)
    }
}

#[test]
fn it_works() {
   let x: Matrix<i32> = Matrix::new(2, 2, &[2, 3, 4, 5]);
   println!("Print Matrix!: {:?}", x);
   let y: Matrix<i32> = Matrix::new(2, 3, &[3, 2, 1, 0, 2, 4]);
   println!("Testing operators: {:?}", x * y);
}

