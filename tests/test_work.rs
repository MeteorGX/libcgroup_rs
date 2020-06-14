use libcgroup_rs::prelude::CGroup;
use libcgroup_rs::cg::CGroupBuilder;

#[test]
fn test_work()->Result<(),std::io::Error>{



    // Initialize
    CGroup::init()?;
    println!("Mount Point = {:?}",CGroup::get_subsys_mount_point("cpu"));


    // create cgroup container
    let container_name = "container_test_work";
    let mut cg = CGroupBuilder::new(container_name)?;
    println!("Source CG = {:?}",cg);


    println!("Add Controller = {:?}",cg.add_controller("cpu")?);
    println!("Get Controller = {:?}",cg.get_controller("cpu")?);
    //cg.free_controllers();


    //only root
    cg.create(0)?;
    println!("Get UID/GID = {:?}",cg.get_uid_pid()?);

    let mut clone_cg = cg.clone();
    println!("Clone CG = {:?}",clone_cg);
    assert!(!clone_cg.is_null());
    assert!(clone_cg.eq(&cg));



    Ok(())
}