
extern crate ndarray;
extern crate ndarray_odeint;

use ndarray::*;
use ndarray_odeint::*;

#[test]
fn arr() {
    let dt = 0.01;
    let eom = model::Lorenz63::default();
    let teo = explicit::euler(eom, dt);
    let mut x: Array1<f64> = arr1(&[1.0, 0.0, 0.0]);
    teo.iterate(&mut x);
}

#[test]
fn rcarr() {
    let dt = 0.01;
    let eom = model::Lorenz63::default();
    let teo = explicit::euler(eom, dt);
    let mut x: RcArray1<f64> = rcarr1(&[1.0, 0.0, 0.0]);
    teo.iterate(&mut x);
}

#[test]
fn view_mut() {
    let dt = 0.01;
    let eom = model::Lorenz63::default();
    let teo = explicit::euler(eom, dt);
    let mut x: Array1<f64> = arr1(&[1.0, 0.0, 0.0]);
    let mut v = &mut x.view_mut();
    teo.iterate(v);
}