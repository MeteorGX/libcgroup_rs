//!
//! ### 3.Iterators
//!
//! URL: [Iterators](http://libcg.sourceforge.net/html/group__group__iterators.html)
//!
//! Functions:
//!    - cgroup_walk_tree_begin
//!    - cgroup_walk_tree_next
//!    - cgroup_walk_tree_end
//!    - cgroup_walk_tree_set_flags
//!    - cgroup_read_stats_begin
//!    - cgroup_read_stats_next
//!    - cgroup_read_stats_end
//!    - cgroup_get_task_begin
//!    - cgroup_get_task_next
//!    - cgroup_get_task_end
//!    - cgroup_get_controller_begin
//!    - cgroup_get_controller_next
//!    - cgroup_get_controller_end
//!    - cgroup_get_all_controller_begin
//!    - cgroup_get_all_controller_next
//!    - cgroup_get_all_controller_end
//!
//! Usage(All_Controller):
//! ```
//! use libcgroup_rs::initialization::CGroupInitializer;
//! use libcgroup_rs::iterators::CGroupAllControllerIterator,
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!     CGroupInitializer::init()?;
//!
//!     let all_ctrl_iter = CGroupAllControllerIterator::from()?;
//!
//!     for info in all_ctrl_iter.into_iter() {
//!         println!("Name = {}",info.get_name());
//!         println!("Hierarchy = {}",info.get_hierarchy());
//!         println!("Num CGroups = {}",info.get_num_cgroups());
//!         println!("Enabled = {}",info.get_enabled());
//!         println!("--------------------------------------------");
//!     }
//!     Ok(())
//! }
//! ```
//!
//! Usage(Controller):
//! ```
//! use libcgroup_rs::initialization::CGroupInitializer;
//! use libcgroup_rs::iterators::CGroupControllerIterator,
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!     CGroupInitializer::init()?;
//!
//!     let ctrl_iter = CGroupControllerIterator::from()?;
//!     for info in ctrl_iter.into_iter() {
//!         println!("Name = {}",info.get_name());
//!         println!("Path = {}",info.get_path());
//!         println!("--------------------------------------------");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Usage(Task):
//! ```
//! use libcgroup_rs::initialization::CGroupInitializer;
//! use libcgroup_rs::iterators::CGroupTaskIterator,
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!     CGroupInitializer::init()?;
//!
//!     let task_iter = CGroupTaskIterator::from("foo","cpu")?;
//!     for info in task_iter.into_iter() {
//!         println!("PID = {}",info);
//!         println!("--------------------------------------------");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Usage(Stat):
//! ```
//! use libcgroup_rs::initialization::CGroupInitializer;
//! use libcgroup_rs::iterators::CGroupStatsIterator,
//!
//! fn main()->Result<(),Box<dyn std::error::Error>>{
//!     CGroupInitializer::init()?;
//!
//!     let stat_iter = CGroupStatsIterator::from("cpu","nr_periods")?;
//!     for info in stat_iter.into_iter() {
//!         println!("Name = {}",info.get_name());
//!         println!("Value = {}",info.get_value());
//!         println!("--------------------------------------------");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!


use crate::prelude::*;
use crate::error::*;

use log::{info};


pub struct CGroupWalkIterator {
    ctrl_name: *const libc::c_char,
    base_path: *const libc::c_char,
    depth: libc::c_int,
    handler: *const libc::c_void,
    info: CGroupFileInfo,
    base_level: libc::c_int,
    ret: libc::c_int,
}


impl CGroupWalkIterator {

    pub fn from(ctrl_name:&str,base_path:&str,depth:i32)->Result<Self,std::io::Error> {
        let mut handler = Self {
            ctrl_name: std::ffi::CString::new(ctrl_name)?.as_ptr(),
            base_path: std::ffi::CString::new(base_path)?.as_ptr(),
            depth: libc::c_int::from(depth),
            handler: std::ptr::null(),
            info: CGroupFileInfo::default(),
            base_level: 0,
            ret: 0
        };


        handler.cg_begin();
        if handler.ret != C_GROUP_SUCCESS {
            return Err(cg_get_error(handler.ret));
        }
        Ok(handler)
    }

    pub fn cg_begin(&mut self)->Option<(i32,CGroupFileInfo)>{
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let c_info = &mut self.info;
            let c_base_level = &mut self.base_level;
            self.ret = cgroup_walk_tree_begin(
                self.ctrl_name,
                self.base_path,
                self.depth,
                c_handler,
                c_info,
                c_base_level
            );
            info!("CGroupWalkIterator::cg_begin[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                let base_level = self.base_level as i32;
                return Some((base_level,self.info.clone()));
            }
        }
        None
    }

    pub fn cg_next(&mut self)->Option<(i32,CGroupFileInfo)>{
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let c_info = &mut self.info;
            let c_base_level = &mut self.base_level;
            self.ret = cgroup_walk_tree_next(c_handler,c_info,c_base_level);

            info!("CGroupWalkIterator::cg_next[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                let base_level = self.base_level as i32;
                return Some((base_level,self.info.clone()));
            }
        }
        None
    }

    pub fn cg_end(&mut self) {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            self.ret = cgroup_walk_tree_end(c_handler);
            info!("CGroupWalkIterator::cg_end[return code] = {}", self.ret);
        }
    }

    pub fn set_flags(&mut self,flags:i32)->i32{
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let c_flags = libc::c_int::from(flags);
            let ret = cgroup_walk_tree_set_flags(c_handler,c_flags);
            info!("CGroupWalkIterator::set_flags[return code] = {}", ret);
            ret
        }
    }
}

impl Iterator for CGroupWalkIterator{
    type Item = (i32,CGroupFileInfo);

    fn next(&mut self) -> Option<Self::Item> {
        return if self.ret != 0 {
            self.cg_end();
            None
        } else {
            let info = self.info.clone();
            let level = self.base_level as i32;
            self.cg_next();
            Some((level,info))
        }
    }
}



pub struct CGroupStatsIterator {
    ctrl_name: *const libc::c_char,
    path_name: *const libc::c_char,
    handler: *const libc::c_void,
    stat: CGroupStat,
    ret: libc::c_int,
}

impl CGroupStatsIterator {

    pub fn from(ctrl_name:&str,path_name:&str)->Result<Self,std::io::Error> {
        let mut handler = Self {
            ctrl_name: std::ffi::CString::new(ctrl_name)?.as_ptr(),
            path_name: std::ffi::CString::new(path_name)?.as_ptr(),
            handler: std::ptr::null(),
            stat: CGroupStat::default(),
            ret: 0
        };

        handler.cg_begin();
        if handler.ret != C_GROUP_SUCCESS {
            return Err(cg_get_error(handler.ret));
        }

        Ok(handler)
    }

    pub fn cg_begin(&mut self) -> Option<CGroupStat> {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let c_stat = &mut self.stat;
            self.ret = cgroup_read_stats_begin(self.ctrl_name,self.path_name,c_handler, c_stat);
            info!("CGroupStatsIterator::cg_begin[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                return Some(self.stat.clone());
            }
        }
        None
    }

    pub fn cg_next(&mut self) -> Option<CGroupStat> {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let c_stat = &mut self.stat;
            self.ret = cgroup_read_stats_next(c_handler,c_stat);
            info!("CGroupStatsIterator::cg_next[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                return Some(self.stat.clone());
            }
        }
        None
    }

    pub fn cg_end(&mut self) {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            self.ret = cgroup_read_stats_end(c_handler);
            info!("CGroupStatsIterator::cg_end[return code] = {}", self.ret);
        }
    }
}


impl Iterator for CGroupStatsIterator{
    type Item = CGroupStat;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.ret != 0 {
            self.cg_end();
            None
        } else {
            let stat = self.stat.clone();
            self.cg_next();
            Some(stat)
        }
    }
}




pub struct CGroupTaskIterator {
    cg_name: *const libc::c_char,
    ctrl_name: *const libc::c_char,
    handler: *const libc::c_void,
    pid: libc::pid_t,
    ret: libc::c_int,
}

impl CGroupTaskIterator {

    pub fn from(cg_name:&str,ctrl_name:&str)->Result<Self,std::io::Error>{
        let mut handler = Self{
            cg_name: std::ffi::CString::new(cg_name)?.as_ptr(),
            ctrl_name: std::ffi::CString::new(ctrl_name)?.as_ptr(),
            handler: std::ptr::null(),
            pid: 0,
            ret: 0
        };

        handler.cg_begin();
        if handler.ret != C_GROUP_SUCCESS {
            return Err(cg_get_error(handler.ret));
        }

        Ok(handler)
    }

    pub fn cg_begin(&mut self) -> Option<i32> {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let c_pid = &mut self.pid;
            self.ret = cgroup_get_task_begin(self.cg_name,self.ctrl_name,c_handler, c_pid);
            info!("CGroupTaskIterator::cg_begin[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                return Some(self.pid);
            }
        }
        None
    }

    pub fn cg_next(&mut self) -> Option<i32> {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let c_pid = &mut self.pid;
            self.ret = cgroup_get_task_next(c_handler, c_pid);
            info!("CGroupTaskIterator::cg_next[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                return Some(self.pid);
            }
        }
        None
    }

    pub fn cg_end(&mut self) {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            self.ret = cgroup_get_task_end(c_handler);
            info!("CGroupTaskIterator::cg_end[return code] = {}", self.ret);
        }
    }

}

impl Iterator for CGroupTaskIterator{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.ret != 0 {
            self.cg_end();
            None
        } else {
            let pid : i32 = self.pid;
            self.cg_next();
            Some(pid)
        }
    }
}




pub struct CGroupControllerIterator {
    handler: *const libc::c_void,
    ctrl: CGroupMountPoint,
    ret: libc::c_int,
}

impl CGroupControllerIterator {
    pub fn from()->Result<Self,std::io::Error>{
        let mut handler = Self { handler: std::ptr::null(), ctrl: CGroupMountPoint::default(),ret:0 };
        handler.cg_begin();
        if handler.ret != C_GROUP_SUCCESS {
            return Err(cg_get_error(handler.ret));
        }

        Ok(handler)
    }

    pub fn cg_begin(&mut self) -> Option<CGroupMountPoint> {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let ctrl = &mut self.ctrl;
            self.ret = cgroup_get_controller_begin(c_handler, ctrl);
            info!("CGroupControllerIterator::cg_begin[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                return Some(ctrl.clone());
            }
        }
        None
    }

    pub fn cg_next(&mut self) -> Option<CGroupMountPoint> {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let ctrl = &mut self.ctrl;
            self.ret = cgroup_get_controller_next(c_handler,ctrl);
            info!("CGroupControllerIterator::cg_next[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                return Some(ctrl.clone());
            }
        }
        None
    }

    pub fn cg_end(&mut self) {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            self.ret = cgroup_get_controller_end(c_handler);
            info!("CGroupControllerIterator::cg_end[return code] = {}", self.ret);
        }
    }
}

impl Iterator for CGroupControllerIterator{
    type Item = CGroupMountPoint;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.ret != 0 {
            self.cg_end();
            None
        } else {
            let ctrl = self.ctrl.clone();
            self.cg_next();
            Some(ctrl)
        }
    }
}




pub struct CGroupAllControllerIterator {
    handler: *const libc::c_void,
    ctrl: CGroupControllerData,
    ret: libc::c_int,
}

impl CGroupAllControllerIterator {

    pub fn from()->Result<Self,std::io::Error>{
        let mut handler = Self { handler: std::ptr::null(), ctrl: CGroupControllerData::default(),ret:0 };
        handler.cg_begin();
        if handler.ret != C_GROUP_SUCCESS {
            return Err(cg_get_error(handler.ret));
        }

        Ok(handler)
    }


    pub fn cg_begin(&mut self) -> Option<CGroupControllerData> {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let ctrl = &mut self.ctrl;
            self.ret = cgroup_get_all_controller_begin(c_handler, ctrl);
            info!("CGroupAllControllerIterator::cg_begin[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                return Some(ctrl.clone());
            }
        }
        None
    }

    pub fn cg_next(&mut self) -> Option<CGroupControllerData> {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            let ctrl = &mut self.ctrl;
            self.ret = cgroup_get_all_controller_next(c_handler,ctrl);
            info!("CGroupAllControllerIterator::cg_next[return code] = {}", self.ret);
            if self.ret == C_GROUP_SUCCESS {
                return Some(ctrl.clone());
            }
        }
        None
    }

    pub fn cg_end(&mut self) {
        unsafe {
            let c_handler = &(self.handler) as *const *const libc::c_void;
            self.ret = cgroup_get_all_controller_end(c_handler);
            info!("CGroupAllControllerIterator::cg_end[return code] = {}", self.ret);
        }
    }
}

impl Iterator for CGroupAllControllerIterator{
    type Item = CGroupControllerData;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.ret != 0 {
            self.cg_end();
            None
        } else {
            let ctrl = self.ctrl.clone();
            self.cg_next();
            Some(ctrl)
        }
    }
}


