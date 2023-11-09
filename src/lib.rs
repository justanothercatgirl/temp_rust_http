use std::ffi::CString; // разноязычный код
use std::ffi::CStr;
use std::os::raw::c_char;
use reqwest::blocking;
use serde::{Deserialize, Deserializer};
//use serde_json::;

#[repr(C)]
#[derive(Debug)]
#[derive(Deserialize)]
pub struct data {
    number_char: u8,
    number_int: u32,
    number_long: i64,
    #[serde(deserialize_with = "str_to_raw")]
    string: *const c_char,
}

fn str_to_raw<'de, D>(deserializer: D) -> Result<*mut c_char, D::Error>
where 
    D: Deserializer<'de>
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let c_string = CString::new(s).map_err(serde::de::Error::custom)?;
    let ptr = c_string.into_raw();
    Ok(ptr)
}

extern "C" {
    fn write(x: u64, y: *const c_char, z: u64);
}

#[no_mangle]
pub extern "C" fn get_request(http: *mut c_char) -> data {
    println!("starting reqwest");
   
    let http = unsafe{ CStr::from_ptr(http) };

    dbg!(http.to_bytes().len()); 

    let result: CString = match blocking::get(http.to_str().unwrap_or("https://example.com")) {
        Ok(resp) => {
            CString::new(resp.text().expect("line 24, expect 1")).expect("could not unwrap Ok(resp)")
        }
        Err(resp) => {
            CString::new(resp.to_string()).expect("could not unwrap Err(resp)")
        }
    };

    println!("get reqwest");
    let ptr = result.as_ptr();
    unsafe {
        write(1, ptr, result.into_bytes().len() as u64 );
    }
    data {
        number_char: 0xAA,
        number_int: 0xBBBBBBBB,
        number_long: 0xCCCCCCCCCCCCCCCCu64 as i64,
        string: ptr,
    }
}

#[no_mangle]
pub extern "C" fn simulate_data_request() -> data {
    let json = r#"
{
    "number_char" : 69, 
    "number_long" : -9999999999,
    "string" : "hello!",
    "number_int" : 1444
}"#;
    let test: data = serde_json::from_str(json).expect("you missed some names lmao");
    dbg!(&test);
    test
}

