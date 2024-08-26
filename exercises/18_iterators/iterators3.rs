
#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if a == i64::MIN && b == -1{
        Err(DivisionError::IntegerOverflow)
    } else if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a % b == 0 {
        Ok(a / b)
    } else {
        Err(DivisionError::NotDivisible)
    } 
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`

// Vec is much easier
fn result_with_list() -> Result<[i64; 4], DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let mut division_results = numbers.into_iter().map(|n| divide(n, 27));
    let mut res: [i64; 4] = [0; 4];
    res[0] = division_results.next().unwrap()?;
    res[1] = division_results.next().unwrap()?;
    res[2] = division_results.next().unwrap()?;
    res[3] = division_results.next().unwrap()?;

    Ok(res)
}

fn result_with_list_vec() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    division_results.collect()
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> [Result<i64, DivisionError>; 4]{
    let numbers = [27, 297, 38502, 81];
    let mut division_results = numbers.into_iter().map(|n| divide(n, 27));
    const ARRAY_REPEAT_VALUE: Result<i64, DivisionError> = Ok(0);
    let mut res: [Result<i64, DivisionError>; 4] = [ARRAY_REPEAT_VALUE; 4];
    res[0] = division_results.next().unwrap();
    res[1] = division_results.next().unwrap();
    res[2] = division_results.next().unwrap();
    res[3] = division_results.next().unwrap();

    res
}

fn list_of_results_vec() -> Vec<Result<i64, DivisionError>>{
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    division_results.collect()
}
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_result_with_list_vec() {
        assert_eq!(result_with_list_vec().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }

    #[test]
    fn test_list_of_results_vec() {
        assert_eq!(list_of_results_vec(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
