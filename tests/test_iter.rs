use libcgroup_rs::initialization::CGroupInitializer;
use libcgroup_rs::iterators::{CGroupAllControllerIterator, CGroupControllerIterator, CGroupTaskIterator, CGroupStatsIterator, CGroupWalkIterator};

#[test]
fn test_all_ctrl()->Result<(),Box<dyn std::error::Error>>{
    CGroupInitializer::init()?;

    // only root
    //let cg = CGroupBuilder::new("foo")?;
    //cg.add_controller("cpu")?;
    //cg.create(0)?;


    //let ctrl = cg.get_controller("cpu")?;
    //println!("CTRL = {:?}",ctrl);

    let all_ctrl_iter = CGroupAllControllerIterator::from()?;


    println!("============== All Controller ===============");
    for info in all_ctrl_iter.into_iter() {
        println!("Name = {}",info.get_name());
        println!("Hierarchy = {}",info.get_hierarchy());
        println!("Num CGroups = {}",info.get_num_cgroups());
        println!("Enabled = {}",info.get_enabled());
        println!("--------------------------------------------");
    }
    println!("=============================================");

    Ok(())
}


#[test]
fn test_ctrl()->Result<(),Box<dyn std::error::Error>>{
    CGroupInitializer::init()?;

    // only root
    //let cg = CGroupBuilder::new("foo")?;
    //cg.add_controller("cpu")?;
    //cg.create(0)?;


    //let ctrl = cg.get_controller("cpu")?;
    //println!("CTRL = {:?}",ctrl);

    println!("============== Controller ===============");
    let ctrl_iter = CGroupControllerIterator::from()?;
    for info in ctrl_iter.into_iter() {
        println!("Name = {}",info.get_name());
        println!("Path = {}",info.get_path());
        println!("--------------------------------------------");
    }
    println!("=============================================");





    Ok(())
}


#[test]
fn test_task()->Result<(),Box<dyn std::error::Error>>{
    CGroupInitializer::init()?;

    // only root
    //let cg = CGroupBuilder::new("foo")?;
    //cg.add_controller("cpu")?;
    //cg.create(0)?;


    //let ctrl = cg.get_controller("cpu")?;
    //println!("CTRL = {:?}",ctrl);

    println!("============== Task ===============");
    let task_iter = CGroupTaskIterator::from("foo","cpu")?;
    for info in task_iter.into_iter() {
        println!("PID = {}",info);
        println!("--------------------------------------------");
    }
    println!("=============================================");


    Ok(())
}


#[test]
fn test_stat()->Result<(),Box<dyn std::error::Error>>{
    CGroupInitializer::init()?;

    // only root
    //let cg = CGroupBuilder::new("foo")?;
    //cg.add_controller("cpu")?;
    //cg.create(0)?;


    //let ctrl = cg.get_controller("cpu")?;
    //println!("CTRL = {:?}",ctrl);

    println!("============== Stats ===============");
    let stat_iter = CGroupStatsIterator::from("cpu","nr_periods")?;
    for info in stat_iter.into_iter() {
        println!("Name = {}",info.get_name());
        println!("Value = {}",info.get_value());
        println!("--------------------------------------------");
    }
    println!("=============================================");


    Ok(())
}


#[test]
fn test_walk()->Result<(),Box<dyn std::error::Error>>{
    CGroupInitializer::init()?;

    // only root
    //let cg = CGroupBuilder::new("foo")?;
    //cg.add_controller("cpu")?;
    //cg.create(0)?;


    //let ctrl = cg.get_controller("cpu")?;
    //println!("CTRL = {:?}",ctrl);

    println!("============== Walk ===============");
    let _walk_iter = CGroupWalkIterator::from("cpu","/dev/shm/cgroups",1)?;

    //for (level,_info) in walk_iter.into_iter() {
    //    println!("Level = {}",level);
    //    println!("--------------------------------------------");
    //}
    println!("=============================================");



    Ok(())
}



