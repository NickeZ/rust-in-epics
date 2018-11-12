#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::str::Utf8Error;

use epics_sys::epics_register;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[epics_register]
pub fn mySubInit_impl(record: &mut subRecord) -> Result<(), ()> {
    record.a = 1f64;
    record.b = 2f64;
    record.c = 3f64;
    record.d = 4f64;
    println!("Initialized record {}", try_from(&record.name).unwrap());
    Ok(())
}

#[epics_register]
pub fn mySubProcess_impl(record: &mut subRecord) -> Result<(), ()> {
    match try_from(record.name.as_ref()) {
        Ok(name) => println!("Hello from rust! name={}", name),
        _ => println!("Invalid UTF8 in name"),
    }
    println!("A={:.2}", record.a);
    record.val = quad(record.a);
    Ok(())
}

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

#[derive(Debug)]
enum Error {
    Utf8Error(Utf8Error),
    NoNullCharacter,
}

fn try_from(input: &[i8]) -> Result<&str, Error> {
    if ! input.contains(&0i8) {
        return Err(Error::NoNullCharacter);
    }
    unsafe {CStr::from_ptr(input.as_ptr())}.to_str().map_err(|e| Error::Utf8Error(e))
}
