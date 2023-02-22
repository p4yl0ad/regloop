use std::io;
use winreg::enums::*;
use winreg::{RegKey, RegValue, types::FromRegValue};
use std::vec::Vec;

// Computer\HKEY_CLASSES_ROOT\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\PackageRepository\Extensions\windows.protocol\
fn display_reg_value(rv: &RegValue) -> String {
    // https://stackoverflow.com/questions/59645451/how-to-react-on-regvaule-types-from-winreg-crate-correctly
    match rv.vtype {
        REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
            String::from_reg_value(rv).unwrap_or_default()
        }
        REG_DWORD => u32::from_reg_value(rv).unwrap_or_default().to_string(),
        REG_QWORD => u64::from_reg_value(rv).unwrap_or_default().to_string(),
        _ => panic!("can only process reg value of type string, u32 or u64"),
    }
}

fn open_sub(current :String, vector : &mut Vec<String>) -> io::Result<()>{
    let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
    let service_key_subkey = hklm.open_subkey(current)?;
    for value in service_key_subkey.enum_values(){
        match value {
            Ok(ref fuck) => {
                if &fuck.0 == "" {
                    let fuckstr: String = display_reg_value(&fuck.1);
                    let split: Vec<&str> = fuckstr.split(":").collect();
                    let split1: String = split[0].to_string();
                    if split1 == "URL"
                    {
                        let splitstrip:Vec<&str> = split[1].split(" ").collect();
                        if splitstrip[0] != ""
                        {
                            vector.push(splitstrip[0].to_string());
                        }
                    }
                }
            },
            Err(_error) => {
                // Silently ignore
                //eprintln!("[ERROR] Problem looping over subkeys: {:?}", error)
            },
        };
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("[i] Hunting for protocol handlers");
    let mut vec:Vec<String> = Vec::new();
    for i in RegKey::predef(HKEY_CLASSES_ROOT).enum_keys() {
        match i {
            Ok(ref good_val) => {
                let _returned = open_sub(good_val.to_string(), &mut vec);
            },
            Err(_error) => {
                // Silently ignore
                //eprintln!("[ERROR] Problem opening subkey: {:?}", error)
            },
        };
    }
    vec.sort();
    vec.dedup();
    for i in &vec
    {
        println!("{}", i);
    }
    Ok(())
}