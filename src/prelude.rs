//! # Globals Handler
//!
//! [Manual](http://libcg.sourceforge.net/html)
//!
//! ### 1.Initialize
//!
//! URL: [Initialize](http://libcg.sourceforge.net/html/group__group__init.html)
//!
//! Functions:
//!    - cgroup_init
//!    - cgroup_get_subsys_mount_point
//!
//!
//! ### 2.Group Manipulation API
//!
//! URL: [Group Manipulation API](http://libcg.sourceforge.net/html/group__group__groups.html)
//!
//! Functions:
//!    - cgroup_new_cgroup
//!    - cgroup_add_controller
//!    - cgroup_get_controller
//!    - cgroup_free
//!    - cgroup_free_controllers
//!    - cgroup_create_cgroup
//!    - cgroup_create_cgroup_from_parent
//!    - cgroup_modify_cgroup
//!    - cgroup_delete_cgroup
//!    - cgroup_delete_cgroup_ext
//!    - cgroup_get_cgroup
//!    - cgroup_copy_cgroup
//!    - cgroup_compare_cgroup
//!    - cgroup_compare_controllers
//!    - cgroup_set_uid_gid
//!    - cgroup_get_uid_gid
//!    - cgroup_add_value_*
//!    - cgroup_get_value_*
//!    - cgroup_set_value_*
//!    - cgroup_get_value_name_count
//!    - cgroup_get_value_name
//!
//! ### 6.Error handling
//!
//! URL: [Error handling](http://libcg.sourceforge.net/html/group__group__errors.html)
//!
//! Functions:
//!    - cgroup_get_last_errno
//!    - cgroup_strerror
//!
//!

#[allow(unused_imports)]
use log::{debug,info};

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




pub struct CGroup;
impl CGroup{

    /// Convert to Rust Error(cgroup_strerror)
    pub fn get_error(code:libc::c_int)->std::io::Error{
        unsafe {
            std::io::Error::new(
                std::io::Error::from_raw_os_error(code).kind(),
                std::ffi::CStr::from_ptr(cgroup_strerror(code))
                    .to_string_lossy()
                    .into_owned()
            )
        }
    }

    /// Get last error number in c(cgroup_get_last_errno)
    pub fn get_last_c_errno()->libc::c_int{
        unsafe {
            cgroup_get_last_errno()
        }
    }

    /// Get last error number in Rust
    pub fn get_last_errno()->i32{
        unsafe {
            Self::convert_c_error(cgroup_get_last_errno())
        }
    }

    /// Get last error string in Rust
    pub fn get_last_error_str()->String{
        unsafe {
            let ret = Self::get_last_c_errno();
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

    /// CGroup Global Initialize(cgroup_init)
    pub fn init()->Result<(),std::io::Error>{
        unsafe {
            let ret = cgroup_init();
            info!("CGROUP::init[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(Self::get_error(ret));
            }
        }
        Ok(())
    }

    /// Convert to Rust errno(libc::c_int to std::i32)
    pub fn convert_c_error(code:libc::c_int)->i32{
        code as i32
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




/// Structure describing one or more control groups.
#[allow(non_camel_case_types)]
pub enum cgroup {}


/// Structure describing a controller attached to one struct cgroup, including parameters of the group and their values.
#[allow(non_camel_case_types)]
pub enum cgroup_controller {}



#[repr(C)]
#[derive(Copy)]
pub struct CGroupStat {
    pub name: [libc::c_char; libc::FILENAME_MAX as usize],
    pub value: [libc::c_char; libc::FILENAME_MAX as usize],
}

impl Clone for CGroupStat {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for CGroupStat {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}




#[repr(C)]
#[derive(Copy)]
pub struct CGroupControllerData {
    pub name: [libc::c_char; libc::FILENAME_MAX as usize],
    pub hierarchy: libc::c_int,
    pub num_cgroups: libc::c_int,
    pub enabled: libc::c_int,
}


impl Clone for CGroupControllerData {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for CGroupControllerData {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}







extern "C" {



    // 1.Initialize
    pub fn cgroup_init()->libc::c_int;
    pub fn cgroup_get_subsys_mount_point(ctrl:*const libc::c_char,mount_point:*const *const libc::c_char)->libc::c_int;



    // 2.Group Manipulation API
    pub fn cgroup_new_cgroup(name:*const libc::c_char)->*mut cgroup;
    pub fn cgroup_add_controller(cg:*mut cgroup,name:*const libc::c_char)->*mut cgroup_controller;
    pub fn cgroup_get_controller(cg:*mut cgroup,name:*const libc::c_char)->*mut cgroup_controller;
    pub fn cgroup_free(cg:*const *const cgroup);
    pub fn cgroup_free_controllers(cg:*mut cgroup);

    pub fn cgroup_create_cgroup(cg:*mut cgroup,ignore_ownership:libc::c_int)->libc::c_int;
    pub fn cgroup_create_cgroup_from_parent(cg:*mut cgroup,ignore_ownership:libc::c_int)->libc::c_int;
    pub fn cgroup_modify_cgroup(cg:*mut cgroup)->libc::c_int;
    pub fn cgroup_delete_cgroup(cg:*mut cgroup,ignore_migration:libc::c_int)->libc::c_int;
    pub fn cgroup_delete_cgroup_ext(cg:*mut cgroup,flags:libc::c_int)->libc::c_int;

    pub fn cgroup_get_cgroup(cg:*mut cgroup)->libc::c_int;
    pub fn cgroup_copy_cgroup(dst:*mut cgroup,src:*mut cgroup)->libc::c_int;
    pub fn cgroup_compare_cgroup(cg_a:*mut cgroup,cg_b:*mut cgroup)->libc::c_int;
    pub fn cgroup_compare_controllers(cg_ctrl_a:*mut cgroup_controller,cg_ctrl_b:*mut cgroup_controller)->libc::c_int;
    pub fn cgroup_set_uid_gid(
        cg:*mut cgroup,
        tasks_uid: libc::uid_t,
        tasks_gid: libc::gid_t,
        ctrl_uid: libc::uid_t,
        ctrl_gid: libc::gid_t
    )->libc::c_int;
    pub fn cgroup_get_uid_gid(
        cg:*mut cgroup,
        tasks_uid: *mut libc::uid_t,
        tasks_gid: *mut libc::gid_t,
        ctrl_uid: *mut libc::uid_t,
        ctrl_gid: *mut libc::gid_t
    )->libc::c_int;



    pub fn cgroup_add_value_string(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:*const libc::c_char
    )->libc::c_int;

    pub fn cgroup_add_value_int64(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:libc::c_longlong
    )->libc::c_int;

    pub fn cgroup_add_value_uint64(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:libc::c_ulonglong
    )->libc::c_int;

    pub fn cgroup_add_value_bool(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:bool
    )->libc::c_int;

    pub fn cgroup_get_value_string(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:*mut *mut libc::c_char
    )->libc::c_int;

    pub fn cgroup_get_value_int64(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:*mut libc::c_longlong
    )->libc::c_int;

    pub fn cgroup_get_value_uint64(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:*mut libc::c_ulonglong
    )->libc::c_int;

    pub fn cgroup_get_value_bool(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:*mut bool
    )->libc::c_int;

    pub fn cgroup_set_value_string(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:*const libc::c_char
    )->libc::c_int;

    pub fn cgroup_set_value_int64(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:libc::c_longlong
    )->libc::c_int;

    pub fn cgroup_set_value_uint64(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:libc::c_ulonglong
    )->libc::c_int;

    pub fn cgroup_set_value_bool(
        cg_ctrl:*mut cgroup_controller,
        name:*const libc::c_char,
        value:bool
    )->libc::c_int;

    pub fn cgroup_get_value_name_count(cg_ctrl:*mut cgroup_controller)->libc::c_int;
    pub fn cgroup_get_value_name(cg_ctrl:*mut cgroup_controller,idx:libc::c_int)->*mut libc::c_char;







    // 6.Error handling
    pub fn cgroup_get_last_errno()->libc::c_int;
    pub fn cgroup_strerror(code:libc::c_int)->*const libc::c_char;



}
