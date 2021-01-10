use super::m1;

pub fn prn(from:&str){
    println!("Caller = {}, Callee = d1/m2::prn()", from);
    m1::prn("d1/m1_1::prn()");
}