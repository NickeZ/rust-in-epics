#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate epics_sys;

use epics_sys::str_from_epics;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn mySubInit_priv(record: &mut subRecord) -> Result<(), ()> {
    record.a = 1f64;
    record.b = 2f64;
    record.c = 3f64;
    record.d = 4f64;
    Ok(())
}

fn mySubProcess_priv(record: &mut subRecord) -> Result<(), ()> {
    match str_from_epics(record.name.as_ref()) {
        Ok(name) => println!("Hello from rust! name={:?}", name),
        _ => println!("Invalid UTF8 in name"),
    }
    println!("A={:.2}", record.a);
    record.val = quad(record.a);
    Ok(())

}

epics_register_function!(
    mySubInit,
    mySubInit_priv,
    subRecord,
    register_func_mySubInit,
    pvar_func_register_func_mySubInit
);

epics_register_function!(
    mySubProcess,
    mySubProcess_priv,
    subRecord,
    register_func_mySubProcess,
    pvar_func_register_func_mySubProcess
);

fn quad(n: f64) -> f64 {
    (n as f64)*(n as f64)
}

#[cfg(test)]
mod tests {
    use ::quad;
    #[test]
    fn it_works() {
        assert_eq!(quad(2.0), 4.0);
    }
}
