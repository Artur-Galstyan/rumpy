use std::fmt;

/// An owned, contiguous, row-major array of f64 values.
#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    data: Vec<f64>,
    shape: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShapeError {
    SizeOverflow,
    LengthMismatch { expected: usize, actual: usize },
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SizeOverflow => write!(f, "shape element count exceeds usize"),
            Self::LengthMismatch { expected, actual } => {
                write!(
                    f,
                    "shape requires {expected} elements, but received {actual}"
                )
            }
        }
    }
}
impl std::error::Error for ShapeError {}

impl Array {
    pub fn from_vec(data: Vec<f64>, shape: Vec<usize>) -> Result<Self, ShapeError> {
        let expected = if shape.contains(&0) {
            0
        } else {
            shape
                .iter()
                .try_fold(1usize, |n, &d| n.checked_mul(d))
                .ok_or(ShapeError::SizeOverflow)?
        };
        if data.len() != expected {
            return Err(ShapeError::LengthMismatch {
                expected,
                actual: data.len(),
            });
        }

        Ok(Self { data, shape })
    }
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn as_slice(&self) -> &[f64] {
        &self.data
    }
    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        &mut self.data
    }
}
