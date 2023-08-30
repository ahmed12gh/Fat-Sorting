

use std::fs;
use std::path::PathBuf;

const PROC_MOUNTS: &str = "/proc/mounts";

#[derive(Debug)]
pub struct Drive { 
   pub name: String,
    pub path: PathBuf,
   pub format : String ,
}

impl Drive {
    pub fn new(n: String, p: &str , format : &str ) -> Drive {
        Drive {
            name: n,
            path: PathBuf::from(p),
            format: String::from(format),
        }
    }
}

pub fn find_drives() -> Option<Vec<Drive>> {
    let media = fs::read(PROC_MOUNTS).expect("you are not on regular linux distro");
    let media = String::from_utf8(media).expect("this is not the expected proc/media file"); 
    
    // the actual list that contain the usb devices 
    let mut usb_drives :Vec<Drive> = Vec::new() ;

    let media :Vec<&str>= media.split("\n").collect() ;
    for device in media{
        if device.contains("/dev/sd"){
            let data :Vec<&str> = device.split_ascii_whitespace().collect();

            let name = String::from(data[0]) ;
            let path = data[1].replace("\\040", " ") ;
            let format = data[2] ;

            let d = Drive::new(name ,&path ,format );
            println!("{d:?}");
            usb_drives.push(d);
        }

    }
    Some(usb_drives) 
}