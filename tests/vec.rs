extern crate sorted_insert;

use sorted_insert::*;

#[test]
fn basic() {
    let mut v: Vec<isize> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc(1));
    assert_eq!([1], v.as_slice());

    let mut v: Vec<isize> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc_binary(1));
    assert_eq!([1], v.as_slice());

    let mut v: Vec<isize> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc(1));
    assert_eq!([1], v.as_slice());

    let mut v: Vec<isize> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc_binary(1));
    assert_eq!([1], v.as_slice());
}

#[test]
fn init_one_element() {
    let mut v: Vec<isize> = vec![0];

    assert_eq!(1, v.sorted_insert_asc(1));
    assert_eq!([0, 1], v.as_slice());

    let mut v: Vec<isize> = vec![0];

    assert_eq!(1, v.sorted_insert_asc_binary(1));
    assert_eq!([0, 1], v.as_slice());

    let mut v: Vec<isize> = vec![0];

    assert_eq!(0, v.sorted_insert_desc(1));
    assert_eq!([1, 0], v.as_slice());

    let mut v: Vec<isize> = vec![0];

    assert_eq!(0, v.sorted_insert_desc_binary(1));
    assert_eq!([1, 0], v.as_slice());

    // ----------

    let mut v: Vec<isize> = vec![2];

    assert_eq!(0, v.sorted_insert_asc(1));
    assert_eq!([1, 2], v.as_slice());

    let mut v: Vec<isize> = vec![2];

    assert_eq!(0, v.sorted_insert_asc_binary(1));
    assert_eq!([1, 2], v.as_slice());

    let mut v: Vec<isize> = vec![2];

    assert_eq!(1, v.sorted_insert_desc(1));
    assert_eq!([2, 1], v.as_slice());

    let mut v: Vec<isize> = vec![2];

    assert_eq!(1, v.sorted_insert_desc_binary(1));
    assert_eq!([2, 1], v.as_slice());
}

#[test]
fn init_two_elements() {
    let mut v: Vec<isize> = vec![0, 2];

    assert_eq!(1, v.sorted_insert_asc(1));
    assert_eq!([0, 1, 2], v.as_slice());

    let mut v: Vec<isize> = vec![0, 2];

    assert_eq!(1, v.sorted_insert_asc_binary(1));
    assert_eq!([0, 1, 2], v.as_slice());

    let mut v: Vec<isize> = vec![2, 0];

    assert_eq!(1, v.sorted_insert_desc(1));
    assert_eq!([2, 1, 0], v.as_slice());

    let mut v: Vec<isize> = vec![2, 0];

    assert_eq!(1, v.sorted_insert_desc_binary(1));
    assert_eq!([2, 1, 0], v.as_slice());
}

#[test]
fn init_three_elements() {
    let mut v: Vec<isize> = vec![0, 1, 2];

    assert_eq!(2, v.sorted_insert_asc(1));
    assert_eq!([0, 1, 1, 2], v.as_slice());

    let mut v: Vec<isize> = vec![0, 1, 2];

    assert_eq!(2, v.sorted_insert_asc_binary(1));
    assert_eq!([0, 1, 1, 2], v.as_slice());

    let mut v: Vec<isize> = vec![2, 1, 0];

    assert_eq!(2, v.sorted_insert_desc(1));
    assert_eq!([2, 1, 1, 0], v.as_slice());

    let mut v: Vec<isize> = vec![2, 1, 0];

    assert_eq!(2, v.sorted_insert_desc_binary(1));
    assert_eq!([2, 1, 1, 0], v.as_slice());
}
