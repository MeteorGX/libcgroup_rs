use log::info;

pub const MNT_DEFAULT:i32 = 0x0;
pub const MNT_FORCE:i32 = 0x1;
pub const MNT_DETACH:i32 = 0x2;
pub const MNT_EXPIRE:i32 = 0x4;
pub const UMOUNT_NOFOLLOW:i32 = 0x6; //Unrealized(since Linux 2.6.34)



pub struct MountBuilder<'a,'b,'c,'d>{
    src_path:&'a str,
    target_path:&'b str,
    type_name:&'c str,
    flags:u64,
    opts:&'d str
}

impl<'a,'b,'c,'d> MountBuilder<'a,'b,'c,'d>{
    pub fn new()->Self{
        Self{
            src_path:"",
            target_path:"",
            type_name:"",
            flags:0,
            opts:""
        }
    }


    pub fn set_src_path(&mut self,src_path:&'a str)->&mut Self{
        self.src_path = src_path;
        self
    }

    pub fn set_target_path(&mut self,target_path:&'b str)->&mut Self{
        self.target_path = target_path;
        self
    }

    pub fn set_type_name(&mut self,type_name:&'c str)->&mut Self{
        self.type_name = type_name;
        self
    }

    pub fn set_flags(&mut self,flags:u64)->&mut Self{
        self.flags = flags;
        self
    }

    pub fn set_opts(&mut self,opts:&'d str)->&mut Self{
        self.opts = opts;
        self
    }

    fn is_param_failed(s:&str)->bool{
        if s.trim().len() <= 0 { true } else { false }
    }

    pub fn mount(&self)->Result<(),std::io::Error> {
        if Self::is_param_failed(self.target_path) ||
            Self::is_param_failed(self.src_path) ||
            Self::is_param_failed(self.type_name){
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }

        unsafe {
            std::fs::create_dir_all(self.target_path)?;

            let c_target_path = std::ffi::CString::new(self.target_path)?;
            let c_src_path = std::ffi::CString::new(self.src_path)?;
            let c_type_name = std::ffi::CString::new(self.type_name)?;
            let c_flags = libc::c_ulong::from(self.flags);
            let c_opts = std::ffi::CString::new(self.opts)?;
            let c_opts_void = c_opts.as_ptr() as *const libc::c_void;

            let ret = libc::mount(
                c_src_path.as_ptr(),
                c_target_path.as_ptr(),
                c_type_name.as_ptr(),
                c_flags,
                c_opts_void
            );
            info!("MountBuilder::mount[return code] = {}",ret);
            if ret != 0 {
                std::fs::remove_dir(self.target_path)?;
                return Err(std::io::Error::from_raw_os_error(ret));
            }
        }

        Ok(())
    }

    pub fn exists(&self)->bool{
        match std::fs::metadata(self.target_path) {
            Ok(d) if !d.is_dir() => false,
            Err(e) if e.kind().eq(&std::io::ErrorKind::NotFound) => false,
            _ => true
        }
    }



    pub fn umount(&self,flag:i32)->Result<(),std::io::Error>{
        if Self::is_param_failed(self.target_path) {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }

        unsafe {
            let c_target_path = std::ffi::CString::new(self.target_path)?;
            let ret = if flag == MNT_DEFAULT {
                libc::umount(c_target_path.as_ptr())
            }else{
                let c_flags = libc::c_int::from(flag);
                libc::umount2(c_target_path.as_ptr(),c_flags)
            };

            info!("MountBuilder::umount[return code] = {}",ret);
            if ret != 0 {
                return Err(std::io::Error::from_raw_os_error(ret))
            }
            std::fs::remove_dir(self.target_path)?;
        }

        Ok(())
    }
}