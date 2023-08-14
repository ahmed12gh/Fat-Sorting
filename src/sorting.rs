//# In God We Trust 

use std::process ;
use std::{process::exit , path::PathBuf , fs} ;
use super::debug; 
use super::drives::find_drives ;
use std::env::* ;

pub fn sort_files(){ 
    // TODO : abiltiy to choose the drive 
    // check if we want to just run the program or check the actual order of files 
    match args().nth(1) { // either debug or no thing
        None => {
            // get a list of mounted drives from /proc/mounts
            let flash_drives = match find_drives() {
                Some(drive) => drive,
                None => {
                    println!("there is no usb devices connected");
                    exit(0);
                }
            };

            // TODO: just make it wait not quit ;
            match flash_drives.len() {
                0 => {
                    println!("No DEVICES connected");
                    exit(0);
                }
                _ => (),
            }
            
            // creating a buffer to store file paths
            let mut files: Vec<PathBuf> = Vec::new();
            //get the mounting point of first drive
            let mount_path: &str = flash_drives[0].path.to_str().expect("fuck"); 
            // get the contents of drive
            let d = fs::read_dir(mount_path).expect("");
            // add them to the buffer 
            for f in d {
                let f = f.expect("").path();
                files.push(f);
            }
            
            dbg!(&files);
            // just prefixes the files with a number to be sorted by 
            sort(files);
            // sort according to ascii alpahbatical order by calling fatsort 
            fatsort(&flash_drives[0].name);
        }

        // uses fatfs to get the acutal order of files in the system and sfn (shot file name 8.3 nane)
        Some(c) => match c.as_str() {
            "debug" => {
                debug::debug(); // deprecated after finishing the project 
            }
            _ => (),
        },
    }
}



fn sort(files: Vec<PathBuf>) {

    //TODO: sort according acustum order 
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

