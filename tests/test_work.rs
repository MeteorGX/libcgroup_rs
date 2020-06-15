use libcgroup_rs::initialization::CGroupInitializer;
#[allow(unused_imports)]
use libcgroup_rs::manipulation::CGroupBuilder;
use libcgroup_rs::extend::MountBuilder;
#[allow(unused_imports)]
use libcgroup_rs::error::{cg_get_error, C_EC_GROUP_NOT_MOUNTED, C_EC_GROUP_NOT_ALLOWED, cg_get_last_errno, cg_get_last_error_str};

#[test]
fn test_initialize()->Result<(),Box<dyn std::error::Error>>{

    // Initialize
    // only root
    // mount cgroup space
    match CGroupInitializer::init() {
        Ok(_) => (),
        Err(e) if e
            .kind()
            .eq(&cg_get_error(C_EC_GROUP_NOT_MOUNTED).kind()) => {

            // Mouth Space
            let mut space = MountBuilder::new();
            space
                .set_target_path("/dev/shm/cgroups")
                .set_src_path("cgroup")
                .set_type_name("cgroup")
                .set_opts("cpu");

            if !space.exists() {
                space.mount()?;
            }
            CGroupInitializer::init()?;
        }
        Err(e) => return Err(Box::new(e)),
    }

    println!("Mount Point = {:?}",CGroupInitializer::get_subsys_mount_point("cpu"));

    Ok(())
}


#[test]
fn test_create()->Result<(),Box<dyn std::error::Error>>{


    CGroupInitializer::init()?;

    // create cgroup container
    let container_name = "foo";
    let cg = CGroupBuilder::new(container_name)?;

    // append controller
    println!("Add Controller = {:?}",cg.add_controller("cpu")?);
    println!("Get Controller = {:?}",cg.get_controller("cpu")?);


    //only root
    match cg.create(0) {
        Ok(_) => (),
        Err(e) if e.kind().eq(&cg_get_error(C_EC_GROUP_NOT_ALLOWED).kind()) =>{
            println!("Only root! = use sudo ?");
            return Ok(());
        }
        Err(e) => return Err(Box::new(e))
    }


    Ok(())
}



#[test]
fn test_error()->Result<(),Box<dyn std::error::Error>>{
    println!("EC = {}",cg_get_last_errno());
    println!("EC Str = {}",cg_get_last_error_str());

    Ok(())
}


#[test]
fn test_create_container()->Result<(),Box<dyn std::error::Error>>{
    CGroupInitializer::init()?;

    let container_name = "foo";
    let container = CGroupBuilder::new(container_name)?;
    println!("Container = {:?}",container);

    let ctrl = container.add_controller("cpu")?;
    println!("Controller = {:?}",ctrl);

    // config = /cgroups/foo/cpu/cfs_quota_us
    ctrl.add_u64("cpu.cfs_quota_us",50000)?;

    // config = /cgroups/foo/cpu/cfs_period_us
    ctrl.add_u64("cpu.cfs_period_us",100000)?;

    // create
    container.create(0)?;

    Ok(())
}