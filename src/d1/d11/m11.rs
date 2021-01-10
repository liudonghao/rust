use crate::d1::m1;
use crate::util;

pub fn prn(from:&str){
    println!("Caller = {}, Callee = d1/d11/m11::prn()", from);
    m1::prn2("d1/d11/m11::prn()");
    util::prn("d1/d11/m11::prn()");
}