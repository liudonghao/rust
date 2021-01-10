use super::d11::m11;

pub fn prn(from:&str){
    println!("Caller = {}, Callee = d1/m1::prn()", from);
    m11::prn("d1/m1::prn()");
}

pub fn prn2(from:&str){
    println!("Caller = {}, Callee = d1/m1::prn2()", from);
}