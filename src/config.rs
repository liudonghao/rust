
use crate::d2::m2;
use crate::d2::d22::m22;
use crate::util;

pub fn prn(from:&str){
    println!("Caller = {}, Callee = config::prn()", from);
    m2::prn("config::prn()");
    m22::prn("config::prn()");
    util::prn("config::prn()");
}