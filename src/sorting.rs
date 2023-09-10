//# In God We Trust

use std::process;
use std::{fs, path::PathBuf};

// use super::debug;
use std::env::*;
use utils::devices::drives::Drive;
use utils::find_drives;


const SPLIT_CHAR : char = '‐';

//TODO : Take index argument
// TODO : abiltiy to choose the drive
/// takes an argument from the command line (used) debug or Nothing .This then is given to fatsort program to actually sort the files as of now `2023/9/6`
pub fn list_drive_files() -> Option<Vec<PathBuf>> {
    // check if we want to just run the program or check the actual order of files
    match args().nth(1) {
        // either debug or no thing
        None => {
            // get a list of mounted drives from /proc/mounts
            let flash_drives = match find_drives() {
                Some(c) => c,
                // TODO: just make it wait not quit ;
                None => {
                    println!("No device Connected");
                    return None;
                }
            };
            let flash = flash_drives[0].clone();

            let mut files: Vec<PathBuf> = Vec::new(); // creating a Vector to store file paths
            let mount_path = flash.path.as_os_str(); // get the mounting point of first drive

            let d = fs::read_dir(mount_path).expect(&format!(
                "could not open dir path {}",
                mount_path.to_str().unwrap()
            )); // get the contents of drive

            for f in d {
                let f = f.expect("could not open file").path();
                files.push(f);
            }

            println!("{:?}", mount_path);
            dbg!(&files);
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

pub fn sort_drive(files: &Vec<PathBuf>, drive: &Drive) {
    let dev_path = drive.name.clone() ;
    sort(&files);
    //sort according to ascii alpahbatical order by calling fatsort
    fatsort(&dev_path);
}
//TODO: sort according a custom order
fn sort(files: &Vec<PathBuf>) {
    for (i , file) in files.iter().enumerate() { 
    
        mv_rename(&file, i as u32);
    }

}

fn mv_rename(path: &PathBuf, order: u32) {
    
    let filename = path.file_name().expect("").to_str().expect("");
    let parent = path.parent().expect("").to_str().expect("");
    let arg = format!("{parent}/{filename}");

    let filename: Vec<&str> = filename.split(SPLIT_CHAR).collect() ;
    let filename = if filename.len() == 2 { filename[1]} else { filename[0]};
    let order = format!("{:0>4}" , order) ;
    //TODO GET the order  part out df the file name ;
    let arg2 = format!("{parent}/{order}{SPLIT_CHAR}{filename}");

    if arg != arg2 { 
        let _mv_out = process::Command::new("/bin/mv")
            .arg(arg)
            .arg(arg2)
            .output()
            .expect("msg");
    
        dbg!(_mv_out);

    }
}

#[allow(dead_code)]
/// path should be the /dev/sd path not the mounted place \n
/// also the filesystem has to be NOT mounted
fn fatsort(dev_path: &str) {
    // possible to change
  
    //  udiskctl umount -b /dev/sd{a,b}{1,2}
    // TODO : make sure that the sd name hasn't change ex from sda1 -> sdb1 or vice versa
    dbg!(dev_path);
    let _mount = process::Command::new("udisksctl")
    .arg("unmount")
    .arg("-b")
    .arg(dev_path)
    .output()
    .expect("error unmounting the drive ");

    // sudo(pkexec graphically) fatsort -a /dev/sd
    let out_put = process::Command::new("pkexec")
        .arg("fatsort")
        .arg(dev_path)
        .output()
        .expect("error");

    //udiskctl mount -b /dev/sd /run/media/user/USB_Drive also ask for permissions
    let _mount = process::Command::new("udisksctl")
        .arg("mount")
        .arg("-b")
        .arg(dev_path)
        .output()
        .expect("error mounting the drive ");
    
    dbg!(out_put);
}
