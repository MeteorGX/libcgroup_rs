//! # Globals Handler
//!
//! ### Manual
//!
//! [libcg site](http://libcg.sourceforge.net/html)
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

#[allow(unused_imports)]
use log::{debug,info};




/// Structure describing one or more control groups.
#[allow(non_camel_case_types)]
pub enum cgroup {}


/// Structure describing a controller attached to one struct cgroup, including parameters of the group and their values.
#[allow(non_camel_case_types)]
pub enum cgroup_controller {}


#[allow(non_camel_case_types)]
#[repr(C)]
pub enum cgroup_file_type{
    FILE,
    DIR,
    OTHER
}


#[repr(C)]
pub struct CGroupFileInfo {
    pub c_type: cgroup_file_type,
    pub path: *const libc::c_char,
    pub parent: *const libc::c_char,
    pub full_path: *const libc::c_char,
    pub depth: libc::c_short,
}

impl Default for CGroupFileInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}



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
pub struct CGroupMountPoint{
    pub name: [libc::c_char; libc::FILENAME_MAX as usize],
    pub path: [libc::c_char; libc::FILENAME_MAX as usize],
}

impl Clone for CGroupMountPoint {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for CGroupMountPoint {
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
        tasks_uid: *const libc::uid_t,
        tasks_gid: *const libc::gid_t,
        ctrl_uid: *const libc::uid_t,
        ctrl_gid: *const libc::gid_t
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
        value:*const *const libc::c_char
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



    // 3.Iterators
    pub fn cgroup_walk_tree_begin(
        ctrl: *const libc::c_char,
        base_path: *const libc::c_char,
        depth: libc::c_int,
        handle: *mut *mut libc::c_void,
        info: *mut cgroup_file_type,
        base_level: *mut libc::c_int
    )->libc::c_int;





    // 6.Error handling
    pub fn cgroup_get_last_errno()->libc::c_int;
    pub fn cgroup_strerror(code:libc::c_int)->*const libc::c_char;



}
