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
//! Usage(Create):
//! ```
//! use libcgroup_rs::initialization::CGroupInitializer;
//! use libcgroup_rs::manipulation::CGroupBuilder;
//! use libcgroup_rs::error::{cg_get_error, C_EC_GROUP_NOT_ALLOWED};
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!     CGroupInitializer::init()?;
//!     let container_name = "foo";
//!     let cg = CGroupBuilder::new(container_name)?;
//!     println!("Source CG = {:?}",cg);
//!
//!     // append controller
//!     println!("Add Controller = {:?}",cg.add_controller("cpu")?);
//!     println!("Get Controller = {:?}",cg.get_controller("cpu")?);
//!
//!     //only root
//!     match cg.create(0) {
//!         Ok(_) => (),
//!         Err(e) if e.kind().eq(&cg_get_error(C_EC_GROUP_NOT_ALLOWED).kind()) =>{
//!             println!("Only root! = use sudo ?");
//!             return Ok(());
//!         }
//!         Err(e) => return Err(Box::new(e))
//!     }
//!     Ok(())
//! }
//! ```
//!
//! Usage(Create with params):
//! ```
//! use libcgroup_rs::initialization::CGroupInitializer;
//! use libcgroup_rs::manipulation::CGroupBuilder;
//! use libcgroup_rs::error::{cg_get_error, C_EC_GROUP_NOT_ALLOWED};
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!     CGroupInitializer::init()?;
//!
//!     let container_name = "foo";
//!     let container = CGroupBuilder::new(container_name)?;
//!     println!("Container = {:?}",container);
//!
//!     let ctrl = container.add_controller("cpu")?;
//!     println!("Controller = {:?}",ctrl);
//!
//!     // cfg = /cgroups/foo/cpu/cfs_quota_us
//!     ctrl.add_u64("cpu.cfs_quota_us",50000)?;
//!
//!     // cfg = /cgroups/foo/cpu/cfs_period_us
//!     ctrl.add_u64("cpu.cfs_period_us",100000)?;
//!
//!     // create
//!     container.create(0)?;
//!
//!     Ok(())
//! }
//! ```
//!

use crate::prelude::*;
use crate::error::*;
use log::{info,error};


#[derive(Debug)]
pub struct CGroupControllerBuilder{
    name: String,
    c_groups:*mut cgroup,
    c_groups_ctrl: *mut cgroup_controller,
}


#[derive(Debug)]
pub struct CGroupBuilder<'a>{
    name: &'a str,
    c_groups: *mut cgroup,
}



impl<'a> CGroupBuilder<'a>{

    pub fn new(name:&'a str)->Result<Self,std::io::Error>{
        let mut cg = Self{
            name,
            c_groups:std::ptr::null_mut(),
        };
        cg.c_groups = unsafe {
            let c_name = std::ffi::CString::new(cg.name)?;
            cgroup_new_cgroup(c_name.as_ptr())
        };

        if cg.c_groups.is_null() {
            return Err(cg_get_error(C_EC_GROUP_NOT_CREATED))
        }
        Ok(cg)
    }

    pub fn is_null(&self)->bool{
        self.c_groups.is_null()
    }

    pub fn add_controller(&self, ctrl_name: &str) ->Result<CGroupControllerBuilder, std::io::Error> {
        unsafe {
            let c_ctrl_name = std::ffi::CString::new(ctrl_name)?;
            let c_ctrl_ptr = cgroup_add_controller(self.c_groups,c_ctrl_name.as_ptr());
            info!("CGroupBuilder::add_controller[return pointer] = {:?}",c_ctrl_ptr);
            if c_ctrl_ptr.is_null() {
                return Err(cg_get_error(C_EC_CONTROLLER_CREATE_FAILED))
            }
            return Ok(CGroupControllerBuilder::new(String::from(ctrl_name),self.c_groups,c_ctrl_ptr));
        }
    }

    pub fn get_controller(&self, ctrl_name: &str) ->Result<CGroupControllerBuilder, std::io::Error>{
        unsafe {
            let c_ctrl_name = std::ffi::CString::new(ctrl_name)?;
            let c_ctrl_ptr = cgroup_get_controller(self.c_groups,c_ctrl_name.as_ptr());
            info!("CGroupBuilder::get_controller[return pointer] = {:?}",c_ctrl_ptr);
            if c_ctrl_ptr.is_null() {
                return Err(cg_get_error(C_EC_CONTROLLER_CREATE_FAILED))
            }
            return Ok(CGroupControllerBuilder::new(String::from(ctrl_name),self.c_groups,c_ctrl_ptr));
        }
    }

    pub fn free(&self){
        unsafe {
            if !self.c_groups.is_null() {
                let c_point = self.c_groups as *const cgroup;
                info!("CGroupBuilder::free[return point] = {:?}",c_point);
                cgroup_free(&c_point  as *const *const cgroup);
            }
        }
    }

    pub fn free_controllers(&self){
        unsafe {
            if !self.c_groups.is_null() {
                let c_point = self.c_groups;
                info!("CGroupBuilder::free_controllers[return point] = {:?}",c_point);
                cgroup_free_controllers(c_point);
            }
        }
    }

    pub fn create(&self,ignore_ownership:i32)->Result<(),std::io::Error>{
        unsafe {
            let c_ignore_ownership = libc::c_int::from(ignore_ownership);
            let ret = cgroup_create_cgroup(self.c_groups,c_ignore_ownership);
            info!("CGroupBuilder::create[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn create_from_parent(&self,ignore_ownership:i32)->Result<(),std::io::Error>{
        unsafe {
            let c_ignore_ownership = libc::c_int::from(ignore_ownership);
            let ret = cgroup_create_cgroup_from_parent(self.c_groups,c_ignore_ownership);
            info!("CGroupBuilder::create_from_parent[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn modify(&self)->Result<(),std::io::Error>{
        unsafe {
            let ret = cgroup_modify_cgroup(self.c_groups);
            info!("CGroupBuilder::modify[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }

        Ok(())
    }

    pub fn delete(&self,ignore_migration:i32)->Result<(),std::io::Error>{
        unsafe {
            let c_ignore_migration = libc::c_int::from(ignore_migration);
            let ret = cgroup_delete_cgroup(self.c_groups,c_ignore_migration);
            info!("CGroupBuilder::delete[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn delete_ext(&self,flags:i32)->Result<(),std::io::Error>{
        unsafe {
            let c_flags = libc::c_int::from(flags);
            let ret = cgroup_delete_cgroup_ext(self.c_groups,c_flags);
            info!("CGroupBuilder::delete_ext[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn set_uid_pid(&self,
                       tasks_uid:u32,
                       tasks_gid:u32,
                       ctrl_uid:u32,
                       ctrl_gid:u32
    )->Result<(),std::io::Error>{
        unsafe {
            let c_tasks_uid = libc::uid_t::from(tasks_uid);
            let c_tasks_gid = libc::uid_t::from(tasks_gid);
            let c_ctrl_uid = libc::uid_t::from(ctrl_uid);
            let c_ctrl_gid = libc::uid_t::from(ctrl_gid);
            let ret = cgroup_set_uid_gid(
                self.c_groups,
                c_tasks_uid,
                c_tasks_gid,
                c_ctrl_uid,
                c_ctrl_gid
            );

            info!("CGroupBuilder::set_uid_pid[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }


    pub fn get_uid_pid(&self)->Result<(u32,u32,u32,u32),std::io::Error>{
        unsafe {
            let c_tasks_uid = libc::uid_t::from(0u32);
            let c_tasks_gid = libc::uid_t::from(0u32);
            let c_ctrl_uid = libc::uid_t::from(0u32);
            let c_ctrl_gid = libc::uid_t::from(0u32);
            let ret = cgroup_get_uid_gid(
                self.c_groups,
                &c_tasks_uid,
                &c_tasks_gid,
                &c_ctrl_uid,
                &c_ctrl_gid
            );

            info!("CGroupBuilder::get_uid_pid[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }

            Ok((
                c_tasks_uid as u32,
                c_tasks_gid as u32,
                c_ctrl_uid as u32,
                c_ctrl_gid as u32,
            ))
        }
    }



}

impl CGroupControllerBuilder {
    pub fn new(name: String, c_groups: *mut cgroup, c_groups_ctrl: *mut cgroup_controller) -> Self {
        Self { name, c_groups, c_groups_ctrl }
    }

    pub fn is_null(&self) -> bool {
        self.c_groups_ctrl.is_null()
    }

    pub fn add_str(&self, name: &str, value: &str) -> Result<(), std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ffi::CString::new(value)?;
            let ret = cgroup_add_value_string(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                c_value.as_ptr());

            info!("CGroupControllerBuilder::add_str[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn add_i64(&self, name: &str, value: i64) -> Result<(), std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_longlong::from(value);
            let ret = cgroup_add_value_int64(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                c_value
            );
            info!("CGroupControllerBuilder::add_i64[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn add_u64(&self, name: &str, value: u64) -> Result<(), std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_ulonglong::from(value);
            let ret = cgroup_add_value_uint64(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                c_value
            );
            info!("CGroupControllerBuilder::add_u64[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn add_bool(&self, name: &str, value: bool) -> Result<(), std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let ret = cgroup_add_value_bool(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                value
            );
            info!("CGroupControllerBuilder::add_bool[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }


    pub fn set_str(&self, name: &str, value: &str) -> Result<(), std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ffi::CString::new(value)?;
            let ret = cgroup_set_value_string(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                c_value.as_ptr());

            info!("CGroupControllerBuilder::set_str[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn set_i64(&self, name: &str, value: i64) -> Result<(), std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_longlong::from(value);
            let ret = cgroup_set_value_int64(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                c_value
            );
            info!("CGroupControllerBuilder::set_i64[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn set_u64(&self, name: &str, value: u64) -> Result<(), std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_ulonglong::from(value);
            let ret = cgroup_set_value_uint64(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                c_value
            );
            info!("CGroupControllerBuilder::set_u64[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn set_bool(&self, name: &str, value: bool) -> Result<(), std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let ret = cgroup_set_value_bool(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                value
            );
            info!("CGroupControllerBuilder::set_bool[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
        }
        Ok(())
    }

    pub fn get_str(&self, name: &str) -> Result<String, std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ptr::null();
            let ret = cgroup_get_value_string(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                &c_value as *const *const libc::c_char
            );

            info!("CGroupControllerBuilder::get_str[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
            Ok(std::ffi::CStr::from_ptr(c_value)
                .to_string_lossy()
                .to_owned()
                .to_string())
        }
    }

    pub fn get_i64(&self, name: &str) -> Result<i64, std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let mut c_value = libc::c_longlong::from(0 as i64);
            let ret = cgroup_get_value_int64(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                &mut c_value
            );
            info!("CGroupControllerBuilder::get_i64[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
            Ok(c_value as i64)
        }
    }

    pub fn get_u64(&self, name: &str) -> Result<u64, std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let mut c_value = libc::c_ulonglong::from(0 as u64);
            let ret = cgroup_get_value_uint64(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                &mut c_value
            );
            info!("CGroupControllerBuilder::get_u64[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
            Ok(c_value as u64)
        }
    }

    pub fn get_bool(&self, name: &str) -> Result<bool, std::io::Error> {
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let mut c_value = false;
            let ret = cgroup_get_value_bool(
                self.c_groups_ctrl,
                c_name.as_ptr(),
                &mut c_value
            );
            info!("CGroupControllerBuilder::get_bool[return code] = {}", ret);
            if ret != C_GROUP_SUCCESS {
                return Err(cg_get_error(ret));
            }
            Ok(c_value as bool)
        }
    }

}


impl<'a> Clone for CGroupBuilder<'a>{

    fn clone(&self) -> Self {
        unsafe {
            match Self::new(self.name.clone()) {
                Ok(mut clone) => {
                    let c_sdt :*mut cgroup = clone.c_groups;
                    let ret = cgroup_copy_cgroup(self.c_groups,c_sdt);
                    info!("CGroupBuilder::clone[return code] = {}",ret);
                    if ret == C_GROUP_SUCCESS {
                        clone.c_groups = c_sdt;
                    }
                    clone
                }
                Err(e) => {
                    error!("CGroupBuilder::clone[error msg] = {:?}",e);
                    Self{name:self.name,c_groups:std::ptr::null_mut()}
                }
            }
        }
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            let c_sdt :*mut cgroup = source.c_groups;
            let ret = cgroup_copy_cgroup(c_sdt,self.c_groups);
            info!("CGroupBuilder::clone_from[return code] = {}",ret);
            if ret == C_GROUP_SUCCESS {
                self.c_groups = c_sdt;
            }
        }
    }
}


impl<'a> PartialEq for CGroupBuilder<'a>{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let ret = cgroup_compare_cgroup(self.c_groups,other.c_groups);
            info!("CGroupBuilder::eq[return code] = {}",ret);
            if ret == C_GROUP_SUCCESS {
                return true;
            }
        }
        false
    }
}


impl PartialEq for CGroupControllerBuilder{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let ret = cgroup_compare_controllers(self.c_groups_ctrl,other.c_groups_ctrl);
            info!("CGroupControllerBuilder::eq[return code] = {}",ret);
            if ret == C_GROUP_SUCCESS {
                return true;
            }
        }
        false
    }
}
