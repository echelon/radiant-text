// Copyright 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Errors

use ilda::IldaError;

/// Errors for the program
pub enum RadiantTextError {
  IldaError { cause: IldaError },
}

impl From<IldaError> for RadiantTextError {
  fn from(error: IldaError) -> Self {
    RadiantTextError::IldaError { cause: error }
  }
}
