//# In God We Trust 

use std::process ;
use std::{process::exit , path::PathBuf , fs} ;

// use super::debug; 
use utils::find_drives; 
use std::env::* ;

//TODO : Take index argument 
// TODO : abiltiy to choose the drive 
/// takes an argument from the command line (used) debug or Nothing .This then is given to fatsort program to actually sort the files as of now `2023/9/6`
pub fn sort_files() -> Option<Vec<PathBuf>>{ 
    
    // check if we want to just run the program or check the actual order of files 
    match args().nth(1) { // either debug or no thing
        None => {
            // get a list of mounted drives from /proc/mounts
            let flash_drives =  match find_drives(){
                Some(c) => c , 
                // TODO: just make it wait not quit ;
                None => {println!("No device Connected") ; return None ; } 
            } ;

            let flash = flash_drives[0].clone() ;
                       
            let mut files: Vec<PathBuf> = Vec::new();         // creating a Vector to store file paths
            let mount_path = flash.path.as_os_str();  // get the mounting point of first drive
            
            let d = fs::read_dir(mount_path).expect(&format!("could not open dir path {}" , mount_path.to_str().unwrap()) );  // get the contents of drive

            for f in d {
                let f = f.expect("could not open file").path();
                files.push(f);
            }
            
            println!("{:?}" , mount_path); 
            dbg!(&files);
            // just prefixes the files with a number to be sorted by 

            /*sort(files);
            // sort according to ascii alpahbatical order by calling fatsort 
            fatsort(&flash_drives[0].name); */

            return Some(files);
        }

        // uses fatfs to get the acutal order of files in the system and sfn (shot file name 8.3 nane)
        Some(arg) => match arg.as_str() {
            "debug" => {
                // debug::debug(); // deprecated after finishing the project 
            }
            _ => (),
        },
        
    }
   None
}


#[allow(unused)]
//TODO: sort according acustum order 
fn sort(files: Vec<PathBuf>) {
let mut i = 1;
let mut j = 0;
while j < files.len() {
    for p in files.clone() {
        let f = p.file_name().expect("").to_str().expect("");
        
        if f.as_bytes()[f.len() - 5] == format!("{}", i).as_bytes()[0] {
            mv_rename(&p, &str_order(i as u32));
                break;
            }
        }

        i += 1;
        j += 1;
    }


}

fn mv_rename(path: &PathBuf, order: &str) {
    let filename = path.file_name().expect("").to_str().expect("");
    let parent = path.parent().expect("").to_str().expect("");

    //TODO GET the order  part out df the file name ;
    let arg = format!("{parent}/{filename}");
    let arg2 = format!("{parent}/{order}.{filename}");

    let _mv_out = process::Command::new("/bin/mv")
        .arg(arg)
        .arg(arg2)
        .output()
        .expect("msg");

    dbg!(_mv_out);
}

fn str_order(i: u32) -> String {
    //adds the needed zeros and convert to string
    if i > 9999999 {
        format!("{i}")
    } else if i > 999999 {
        format!("{i}0")
    } else if i > 99999 {
        format!("{i}00")
    } else if i > 9999 {
        format!("{i}000")
    } else if i > 999 {
        format!("{i}0000")
    } else if i > 99 {
        format!("{i}00000")
    } else if i > 9 {
        format!("{i}000000")
    } else {
        format!("{i}0000000")
    }
}

#[allow(dead_code)]
/// path should be the /dev/sd path not the mounted place \n
/// also the filesystem has to be NOT mounted
fn fatsort(path: &str) {
    // possible to change
    // whoami -> user -\n 
    let user: String =
        String::from_utf8(process::Command::new("whoami").output().expect("").stdout)
            .expect("")
            .trim_end()
            .parse()
            .expect("");

    let p = format!("/run/media/{user}/USB_Drive");
    
    // mkdir /run/media/user/usb_drive
    let _ = process::Command::new("mkdir").arg(p).output().expect("");
    
    //umount /dev/sd{a,b}{1,2} 
    // TODO : make sure that the sd name hasn't change ex from sda1 -> sdb1 or vice versa 
    let _umount = process::Command::new("umount")
        .arg(path)
        .output()
        .expect("could not unmount");

    // sudo(pkexec graphically) fatsort -a /dev/sd
    let out_put = process::Command::new("pkexec")
        .arg("fatsort")
        .arg(path)
        .output()
        .expect("error");
    
    // mount /dev/sd /run/media/user/USB_Drive also ask for permissions  
    let _mount = process::Command::new("pkexec")
        .arg("mount")
        .arg(path)
        .arg("")
        .output()
        .expect("error");

    dbg!(out_put);
}

