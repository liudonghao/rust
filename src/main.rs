use tinker_module::d1::{m1,m1_1};
use tinker_module::d1::d11::m11;
use tinker_module::d2::m2;
use tinker_module::d2::d22::m22;
use tinker_module::config;

fn main() {
    m1::prn("main::main()");
    m1_1::prn("main::main()");
    m11::prn("main::main()");
    m2::prn("main::main()");
    m22::prn("main::main()");
    config::prn("main::main()");
}
