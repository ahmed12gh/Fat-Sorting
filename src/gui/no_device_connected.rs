// In GOD We T-Rust ;

use adw::StatusPage; 

pub fn no_device_connected() -> StatusPage{ 
    
    let status_page = adw::StatusPage::builder()
    .title("No Flash Drive Connected / Detected")
    .description("Drive should be mounted and visible in /proc/mounts to be detected")
    .icon_name("drive-removable-media-symbolic")
    .width_request(600)
    .build();
    status_page

}
// In GOD We T-Rust ;
