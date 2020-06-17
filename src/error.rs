//!
//! ### 6.Error handling
//!
//! URL: [Error handling](http://libcg.sourceforge.net/html/group__group__errors.html)
//!
//! Functions:
//!    - cgroup_get_last_errno
//!    - cgroup_strerror
//!
//! Usage:
//! ```
//! use libcgroup_rs::error::{cg_get_last_errno, cg_get_last_error_str};
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!     println!("EC = {}",cg_get_last_errno());
//!     println!("EC Str = {}",cg_get_last_error_str());
//!     Ok(())
//! }
//! ```
//!

use crate::prelude::*;

/// code = 0, success
pub static C_GROUP_SUCCESS: libc::c_int = 0;

/// code = 50000
pub static C_EC_GROUP_NOT_COMPILED: libc::c_int = 50000;

/// code = 50001
pub static C_EC_GROUP_NOT_MOUNTED: libc::c_int = 50001;

/// code = 50002
pub static C_EC_GROUP_NOT_EXIST: libc::c_int = 50002;

/// code = 50003
pub static C_EC_GROUP_NOT_CREATED: libc::c_int = 50003;

/// code = 50004
pub static C_EC_GROUP_SUBSYS_NOT_MOUNTED: libc::c_int = 50004;

/// code = 50005
pub static C_EC_GROUP_NOT_OWNER: libc::c_int = 50005;

/// code = 50006, controller bound to different mount points
pub static C_EC_GROUP_MULTI_MOUTHED: libc::c_int = 50006;

/// code = 50007
pub static C_EC_GROUP_NOT_ALLOWED: libc::c_int = 50007;

/// code = 50008
pub static C_EC_MAX_VALUES_EXCEEDED: libc::c_int = 50008;

/// code = 50009
pub static C_EC_CONTROLLER_EXISTS: libc::c_int = 50009;

/// code = 50010
pub static C_EC_VALUE_EXISTS: libc::c_int = 50010;

/// code = 50011
pub static C_EC_INVAL: libc::c_int = 50011;

/// code = 50012
pub static C_EC_CONTROLLER_CREATE_FAILED: libc::c_int = 50012;

/// code = 50013
pub static C_EC_FAIL: libc::c_int = 50013;

/// code = 50014
pub static C_EC_GROUP_NOT_INITIALIZED: libc::c_int = 50014;

/// code = 50015
pub static C_EC_GROUP_VALUE_NOT_EXIST: libc::c_int = 50015;

/// code = 50016, represents error coming from other libraries like glibc
pub static C_EC_OTHER: libc::c_int = 50016;

/// code = 50017
pub static C_EC_GROUP_NOT_EQUAL: libc::c_int = 50017;

/// code = 50018
pub static C_EC_GROUP_ROLLER_NOT_EQUAL: libc::c_int = 50018;

/// code = 50019, failed to parse rules configuration file
pub static C_EC_GROUP_PARSE_FAIL: libc::c_int = 50019;

/// code = 50020, rules List does not exist
pub static C_EC_GROUP_NO_RULES: libc::c_int = 50020;

/// code = 50021
pub static C_EC_GROUP_MOUNT_FAIL: libc::c_int = 50021;

/// code = 50022, not an real error, just a auxiliary mark in the enum
pub static C_EC_GROUP_SENTINEL: libc::c_int = 50022;

/// code = 50023, not an real error, it just indicates that that iterator has come to end of sequence and no more items are left.
pub static C_EC_GROUP_EOF: libc::c_int = 50023;

/// code = 50024, failed to parse config(cgconfig.conf)
pub static C_EC_GROUP_CONFIG_PARSE_FAIL: libc::c_int = 50024;

/// code = 50025
pub static C_EC_GROUP_NAMESPACE_PATHS: libc::c_int = 50025;

/// code = 50026
pub static C_EC_GROUP_NAMESPACE_CONTROLLER: libc::c_int = 50026;

/// code = 50027
pub static C_EC_GROUP_MOUNT_NAMESPACE: libc::c_int = 50027;


/// Convert to Rust Error(cgroup_strerror)
pub fn cg_get_error(code:libc::c_int)->std::io::Error{
    unsafe {
        std::io::Error::new(
            std::io::Error::from_raw_os_error(code).kind(),
            if code == C_EC_OTHER {
                    format!("Unknown Error = {}",code)
                }else{
                    std::ffi::CStr::from_ptr(cgroup_strerror(code))
                        .to_string_lossy()
                        .into_owned()
                }

        )
    }
}


/// Get last error number in c(cgroup_get_last_errno)
pub fn cg_get_last_c_errno()->libc::c_int{
    unsafe {
        cgroup_get_last_errno()
    }
}


/// Convert to Rust errno(libc::c_int to std::i32)
pub fn cg_convert_c_error(code:libc::c_int)->i32{
    code as i32
}

/// Get last error number in Rust
pub fn cg_get_last_errno()->i32{
    unsafe {
        cg_convert_c_error(cgroup_get_last_errno())
    }
}


/// Get last error string in Rust
pub fn cg_get_last_error_str()->String{
    unsafe {
        let ret = cg_get_last_c_errno();
        if ret == C_GROUP_SUCCESS {
            String::from("Success")
        }else{
            std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                .to_string_lossy()
                .into_owned()
                .to_string()
        }
    }
}

