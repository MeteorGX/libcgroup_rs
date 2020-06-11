
#[allow(unused_imports)]
use crate::prelude::*;

#[allow(unused_imports)]
use log::{debug,info};


#[derive(Debug)]
pub struct CGroupControllerBuilder{
    name: String,
    c_groups_ctrl: *mut cgroup_controller,
}


#[derive(Debug)]
pub struct CGroupBuilder<'a>{
    name: &'a str,
    c_groups: *mut cgroup,
}


impl CGroupControllerBuilder{
    pub fn new(name:String,c_groups_ctrl:*mut cgroup_controller)->Self{
        Self{name,c_groups_ctrl}
    }
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
           return Err(CGroup::get_error(C_EC_GROUP_NOT_CREATED))
       }
       Ok(cg)
   }

    pub fn add_controller(&self, ctrl_name: &str) ->Result<CGroupControllerBuilder, std::io::Error> {
        unsafe {
            let c_ctrl_name = std::ffi::CString::new(ctrl_name)?;
            let c_ctrl_ptr = cgroup_add_controller(self.c_groups,c_ctrl_name.as_ptr());
            info!("CGroupBuilder::add_controller[return pointer] = {:?}",c_ctrl_ptr);
            if c_ctrl_ptr.is_null() {
                return Err(CGroup::get_error(C_EC_CONTROLLER_CREATE_FAILED))
            }
            return Ok(CGroupControllerBuilder::new(String::from(ctrl_name),c_ctrl_ptr));
        }
    }

    pub fn get_controller(&self, ctrl_name: &str) ->Result<CGroupControllerBuilder, std::io::Error>{
        unsafe {
            let c_ctrl_name = std::ffi::CString::new(ctrl_name)?;
            let c_ctrl_ptr = cgroup_get_controller(self.c_groups,c_ctrl_name.as_ptr());
            info!("CGroupBuilder::get_controller[return pointer] = {:?}",c_ctrl_ptr);
            if c_ctrl_ptr.is_null() {
                return Err(CGroup::get_error(C_EC_CONTROLLER_CREATE_FAILED))
            }
            return Ok(CGroupControllerBuilder::new(String::from(ctrl_name),c_ctrl_ptr));
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
                return Err(CGroup::get_error(ret));
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
                return Err(CGroup::get_error(ret));
            }
        }
        Ok(())
    }

    pub fn modify(&self)->Result<(),std::io::Error>{
        unsafe {
            let ret = cgroup_modify_cgroup(self.c_groups);
            info!("CGroupBuilder::modify[return code] = {}",ret);
            if ret != C_GROUP_SUCCESS {
                return Err(CGroup::get_error(ret));
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
                return Err(CGroup::get_error(ret));
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
                return Err(CGroup::get_error(ret));
            }
        }
        Ok(())
    }

}