#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Ok(Self(value as u64))
        if value < 0 {
           return Err(CreationError::Negative);
        } else if value == 0 {
            return Err(CreationError::Zero);
        } 
            Ok(Self(value as u64))
        
    }
}

fn main() {
    // You can optionally experiment here.
     match PositiveNonzeroInteger::new(0) {
        Ok(n) => println!("Success: {:?}", n),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}