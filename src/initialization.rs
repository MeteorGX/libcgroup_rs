//! ### 1.Initialize
//!
//! URL: [Initialize](http://libcg.sourceforge.net/html/group__group__init.html)
//!
//! Functions:
//!    - cgroup_init
//!    - cgroup_get_subsys_mount_point
//!
//! Usage:
//! ```
//! use libcgroup_rs::initialization::CGroupInitializer;
//! use libcgroup_rs::error::{C_EC_GROUP_NOT_MOUNTED,cg_get_error};
//! use libcgroup_rs::extend::MountBuilder;
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!     // Initialize
//!     // only root
//!     // mount cgroup space
//!     match CGroupInitializer::init() {
//!         Ok(_) => (),
//!         Err(e) if e
//!             .kind()
//!             .eq(&cg_get_error(C_EC_GROUP_NOT_MOUNTED).kind()) => {
//!
//!             // Mouth Space
//!             let mut space = MountBuilder::new();
//!             space
//!                 .set_target_path("/dev/shm/cgroups")
//!                 .set_src_path("cgroup")
//!                 .set_type_name("cgroup")
//!                 .set_opts("cpu");
//!
//!             if !space.exists() {
//!                 space.mount()?;
//!             }
//!             CGroupInitializer::init()?;
//!         }
//!         Err(e) => return Err(Box::new(e)),
//!     }
//!
//!     println!("Mount Point = {:?}",CGroupInitializer::get_subsys_mount_point("cpu"));
//!
//!     Ok(())
//! }
//! ```
//!


use crate::prelude::*;
use crate::error::*;
use log::{info};

pub struct CGroupInitializer;
impl CGroupInitializer{

    /// CGroup Global Initialize(cgroup_init)
    pub fn init()->Result<(),std::io::Error>{
        unsafe {
            let ret = cgroup_init();
            info!("CGROUP::init[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }


    pub fn get_subsys_mount_point(ctrl_name:&str)->Option<String>{
        unsafe {
            if let Ok(c_ctrl_name) = std::ffi::CString::new(ctrl_name) {
                let c_point = std::ptr::null();
                let ret = cgroup_get_subsys_mount_point(
                    c_ctrl_name.as_ptr(),
                    &c_point as *const *const libc::c_char
                );
                info!("CGROUP::get_subsys_mount_point[return code] = {}",ret);
                if ret == C_GROUP_SUCCESS {
                    return Some(std::ffi::CStr::from_ptr(c_point)
                        .to_string_lossy()
                        .to_owned()
                        .to_string())
                }

            }
        }
        return None;
    }

}