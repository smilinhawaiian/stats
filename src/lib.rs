// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;

/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```
pub fn mean(nums: &[f64]) -> Option<f64> {
    let count = nums.len() as f64;
    let mut arithmetic = 0.0;
    let mut sum = 0.0;
    if count != 0.0 {
        for num in &nums[..] {
            sum += num;
        }
        arithmetic = sum/count;
    }
    Some(arithmetic)
}

#[test]
fn test_mean_100(){
    assert_eq!(Some(100.0), mean(&[75.5,100.5,95.5,265.5,-37.0]));
}

#[test]
fn test_mean_single(){
    assert_eq!(Some(25.0), mean(&[25.0]));
}

#[test]
fn test_mean_two(){
    assert_eq!(Some(1.0), mean(&[-1.0, 3.0]));
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```
pub fn stddev(nums: &[f64]) -> Option<f64> {
    let count = nums.len() as f64;
    let mut sigma = 0.0;
    let xbar = mean(nums).unwrap() as f64;//no error since mean will be Some
    let mut sqnums = Vec::new();
    if count != 0.0 {
        for val in &nums[..] {
            let temp = (val - xbar).powf(2.0);
            sqnums.push(temp);
            sigma = mean(&sqnums[..]).unwrap().sqrt() as f64;
        }
        Some(sigma)
    } else {
        None
    }
}

#[test]
fn test_stdev_97(){
    assert_eq!(97.0, stddev(&[75.5,100.5,95.5,265.5,-37.0]).unwrap().round());
}

#[test]
fn test_stdev_single(){
    assert_eq!(Some(0.0), stddev(&[25.0]));
}

#[test]
fn test_stdev_two(){
    assert_eq!(Some(2.0), stddev(&[-1.0, 3.0]));
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut index = nums.len();
    if index != 0 {
        index = (index-1)/2;
        let med = nums[index];
        Some(med)
    } else {
        None
    }
}

#[test]
fn test_median_95(){
    assert_eq!(Some(95.5), median(&[75.5,100.5,95.5,265.5,-37.0]));
}

#[test]
fn test_median_single(){
    assert_eq!(Some(25.0), median(&[25.0]));
}

#[test]
fn test_median_two(){
    assert_eq!(Some(-1.0), median(&[-1.0, 3.0]));
}

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
pub fn l2(nums: &[f64]) -> Option<f64> {
    let mut norm = 0.0;
    let mut sum = 0.0;
    //let count = nums.len() as f64;
    //if count != 0.0 {
    if nums.len() != 0 {
        for val in &nums[..] {
            sum += val.powf(2.0);
        }
        norm = sum.sqrt();
    }
    Some(norm)
}

#[test]
fn test_l2_311(){
    assert_eq!(311.0, l2(&[75.5,100.5,95.5,265.5,-37.0]).unwrap().round());
}

#[test]
fn test_l2_single(){
    assert_eq!(Some(25.0), l2(&[25.0]));
}

#[test]
fn test_l2_two(){
    assert_eq!(3.0, l2(&[-1.0, 3.0]).unwrap().round());
}
