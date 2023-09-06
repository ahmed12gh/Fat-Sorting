pub use drives::find_drives;

/// module used to detect the usb flash drives connected to the device
/// this module should be used with rusb to detect hot plug (any device) that have been connected.
/// it exports the `Drive` struct contains `name` `path` `format` for now format is not being used hopefully in the future .
/// `name` is like /dev/sda1 .
/// `path` is like /run/media/user/usb-name.
///
///
pub mod drives {

    use std::fs;
    use std::path::PathBuf;

    const PROC_MOUNTS: &str = "/proc/mounts";
    const DEV : &str = "/dev/";
    #[derive(Debug, Clone)]
    pub struct Drive {
        pub name: String,
        pub path: PathBuf,
        pub format: String,
    }

    impl Drive {
        pub fn new(n: String, p: &str, format: &str) -> Drive {
            Drive {
                name: n,
                path: PathBuf::from(p),
                format: String::from(format),
            }
        }
    }
    pub fn check_dev() -> Result<Vec<PathBuf> , std::io::Error>{
        //TODO:read the /dev dir to check for sd** 
        let dir_dev = match fs::read_dir(DEV){
            Ok(d) => d ,
            Err(e) => {
                dbg!("/dev/ is not found or not permitted "); 
                return Err(e);
            }
        } ;

        let mut sds :Vec<PathBuf> = vec![] ;
        // look for sd** in /dev
        for entry in dir_dev { 
            let e = entry.unwrap() ;

            let name = e.file_name() ;
            if name.into_string().expect("").contains("sd"){
                sds.push(e.path());
            }
        }
        Ok(sds)
    }

    /// return a list of the connect flash drives ,in the form of `Drive` Struct
    pub fn find_drives() -> Option<Vec<Drive>> {
        let media = fs::read(PROC_MOUNTS).expect("you are not on regular linux distro");
        let media = String::from_utf8(media).expect("this is not the expected proc/media file");

        // the actual list that contain the usb devices
        // read each line to find /dev/sd**
        let mut usb_drives: Vec<Drive> = Vec::new();
        let media: Vec<&str> = media.split("\n").collect();

        for device in media {
            if device.contains("/dev/sd") {
                let data: Vec<&str> = device.split_ascii_whitespace().collect();

                let name = String::from(data[0]);
                let path = data[1].replace("\\040", " ");
                let format = data[2];

                let d = Drive::new(name, &path, format);
                println!("{d:?}");
                usb_drives.push(d);
            }
        }
        if usb_drives.len() > 0 {
            return Some(usb_drives);
        } else {
            return None;
        }
    }
}
