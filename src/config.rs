//!
//! ### 5.Configuration
//!
//! URL: [Configuration](http://libcg.sourceforge.net/html/group__group__config.html)
//!
//! Functions:
//!    - cgroup_config_load_config
//!    - cgroup_unload_cgroups
//!
//! Usage:
//! ```
//! use libcgroup_rs::config::Loader;
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!
//!     assert!(Loader::load("/etc/cgroup.conf").is_ok());
//!     assert!(Loader::unload().is_ok());
//!     Ok(())
//! }
//! ```
//!


use crate::prelude::*;
use crate::error::{C_GROUP_SUCCESS, cg_get_error};

pub struct Loader{}


impl Loader{
    pub fn load(pathname:&str)->Result<(),std::io::Error>{
        unsafe {
            let c_pathname = std::ffi::CString::new(pathname)?;
            let ret = cgroup_config_load_config(c_pathname.as_ptr());
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret))
            }
        }
        Ok(())
    }

    pub fn unload()->Result<(),std::io::Error>{
        unsafe {
            let ret = cgroup_unload_cgroups();
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret))
            }
        }
        Ok(())
    }
}