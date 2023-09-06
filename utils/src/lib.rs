#[allow(unused)]
use rusb; 

pub mod devices;
pub use  devices::find_drives ;

pub fn hot_plug(){ 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
     
    }
}
