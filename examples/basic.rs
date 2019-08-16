extern crate ecl_sys as ecl;

use std::env;
use std::ffi::CString;
use std::ptr;

fn main() {
    unsafe {
        let args = env::args()
            .map(CString::new)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut c_args = args.iter().map(|str| str.as_ptr()).collect::<Vec<_>>();
        c_args.push(ptr::null_mut());
        ecl::cl_boot((c_args.len() - 1) as _, c_args.as_ptr() as _);

        let test_code = CString::new("(+ 1 2)").unwrap();
        let code_len = test_code.as_bytes().len();
        let code = ecl::si_string_to_object(
            1,
            ecl::ecl_make_simple_base_string(test_code.as_ptr() as _, code_len as _),
        );
        let ret = ecl::cl_eval(code);
        println!("ret: {}", ecl::ecl_to_fixnum(ret));
        ecl::cl_shutdown();
    }
}
