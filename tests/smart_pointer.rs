#![cfg(feature = "std")]

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use sorted_insert::*;

#[test]
fn rc() {
    let mut v: Vec<Rc<isize>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc(Rc::new(1)));
    assert_eq!([Rc::new(1)], v.as_slice());

    let mut v: Vec<Rc<isize>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc_binary(Rc::new(1)));
    assert_eq!([Rc::new(1)], v.as_slice());

    let mut v: Vec<Rc<isize>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc(Rc::new(1)));
    assert_eq!([Rc::new(1)], v.as_slice());

    let mut v: Vec<Rc<isize>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc_binary(Rc::new(1)));
    assert_eq!([Rc::new(1)], v.as_slice());
}

#[test]
fn rc_refcell() {
    let mut v: Vec<Rc<RefCell<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc(Rc::new(RefCell::new(1))));
    assert_eq!([Rc::new(RefCell::new(1))], v.as_slice());

    let mut v: Vec<Rc<RefCell<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc_binary(Rc::new(RefCell::new(1))));
    assert_eq!([Rc::new(RefCell::new(1))], v.as_slice());

    let mut v: Vec<Rc<RefCell<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc(Rc::new(RefCell::new(1))));
    assert_eq!([Rc::new(RefCell::new(1))], v.as_slice());

    let mut v: Vec<Rc<RefCell<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc_binary(Rc::new(RefCell::new(1))));
    assert_eq!([Rc::new(RefCell::new(1))], v.as_slice());
}

#[test]
fn arc() {
    let mut v: Vec<Arc<isize>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc(Arc::new(1)));
    assert_eq!([Arc::new(1)], v.as_slice());

    let mut v: Vec<Arc<isize>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc_binary(Arc::new(1)));
    assert_eq!([Arc::new(1)], v.as_slice());

    let mut v: Vec<Arc<isize>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc(Arc::new(1)));
    assert_eq!([Arc::new(1)], v.as_slice());

    let mut v: Vec<Arc<isize>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc_binary(Arc::new(1)));
    assert_eq!([Arc::new(1)], v.as_slice());
}

#[test]
fn arc_mutex() {
    let mut v: Vec<Arc<Mutex<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc(Arc::new(Mutex::new(1))));
    assert_eq!(vec![1], v.as_slice().iter().map(|e| *e.lock().unwrap()).collect::<Vec<isize>>());

    let mut v: Vec<Arc<Mutex<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc_binary(Arc::new(Mutex::new(1))));
    assert_eq!(vec![1], v.as_slice().iter().map(|e| *e.lock().unwrap()).collect::<Vec<isize>>());

    let mut v: Vec<Arc<Mutex<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc(Arc::new(Mutex::new(1))));
    assert_eq!(vec![1], v.as_slice().iter().map(|e| *e.lock().unwrap()).collect::<Vec<isize>>());

    let mut v: Vec<Arc<Mutex<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc_binary(Arc::new(Mutex::new(1))));
    assert_eq!(vec![1], v.as_slice().iter().map(|e| *e.lock().unwrap()).collect::<Vec<isize>>());
}

#[test]
fn arc_rw_lock() {
    let mut v: Vec<Arc<RwLock<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc(Arc::new(RwLock::new(1))));
    assert_eq!(vec![1], v.as_slice().iter().map(|e| *e.read().unwrap()).collect::<Vec<isize>>());

    let mut v: Vec<Arc<RwLock<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_asc_binary(Arc::new(RwLock::new(1))));
    assert_eq!(vec![1], v.as_slice().iter().map(|e| *e.read().unwrap()).collect::<Vec<isize>>());

    let mut v: Vec<Arc<RwLock<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc(Arc::new(RwLock::new(1))));
    assert_eq!(vec![1], v.as_slice().iter().map(|e| *e.read().unwrap()).collect::<Vec<isize>>());

    let mut v: Vec<Arc<RwLock<isize>>> = Vec::new();

    assert_eq!(0, v.sorted_insert_desc_binary(Arc::new(RwLock::new(1))));
    assert_eq!(vec![1], v.as_slice().iter().map(|e| *e.read().unwrap()).collect::<Vec<isize>>());
}
