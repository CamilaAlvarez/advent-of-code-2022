use std::num::ParseIntError;

#[derive(Debug)]
pub enum ElfError {
    UnableToLoad(String),
    UnableToOtainCalories(String),
}

impl std::fmt::Display for ElfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnableToLoad(error) => write!(f, "Unable to load elves: {}", error),
            Self::UnableToOtainCalories(error) => write!(f, "Invalid calories: {}", error),
        }
    }
}

impl std::convert::From<std::io::Error> for ElfError {
    fn from(error: std::io::Error) -> Self {
        Self::UnableToLoad(format!("Unable to load elves file: {}", error))
    }
}

impl std::convert::From<ParseIntError> for ElfError {
    fn from(e: ParseIntError) -> Self {
        Self::UnableToOtainCalories(format!("Couldn't get calories: {}", e))
    }
}
