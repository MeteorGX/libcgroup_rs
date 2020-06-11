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
    println!("Add Controller = {:?}",cg.add_controller("cpu")?);
    println!("Get Controller = {:?}",cg.get_controller("cpu")?);
    //cg.free_controllers();

    //only root
    cg.create(0)?;

    Ok(())
}