use fatfs::{FileSystem, FsOptions};
use fscommon::BufStream;
use std::process::exit;
use sudo ;
use crate::drives::* ;



pub fn debug(){ 
    _ = sudo::escalate_if_needed();
    let flash_drives  = match find_drives()  { Some(drive) => drive , 
        None  => {println!("there is no usb devices connected") ;exit(0) }  
        };
    
        match flash_drives.len() {
            0 => { println!("No DEVICES connected") ; exit(0)}, 
            _=> () ,
        }

    let path = flash_drives[0].name.clone() ;
    read_fat(&path)
}
/// this fucntion is to make sure that files are named by a suitable short file name ;  
/* extern crate sudo  
///  escalte_if_needed() ; 
/// use` read_fat("/dev/sda1"); 
///  */
/// 
//
fn read_fat(path: &str) {
   
    let f = std::fs::File::open(path).expect("fuck ");
    let buf_rdr = BufStream::new(f);
    let fs = FileSystem::new(buf_rdr, FsOptions::new()).expect("fuck");
    let root_dir = fs.root_dir();

    for e in root_dir.iter() {
        let e = e.expect("");
        if e.is_file() {
            println!("{:?}:{:?}" ,e.short_file_name() , e.file_name());
        }
    }
}

