use crate::d1::d11::m11;

pub fn prn(from:&str){
    println!("Caller = {}, Callee = d2/m2::prn()", from);
    m11::prn("d2/m2::prn()");
}