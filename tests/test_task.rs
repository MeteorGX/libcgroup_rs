use libcgroup_rs::initialization::CGroupInitializer;
use libcgroup_rs::manipulation::CGroupBuilder;

#[test]
fn test_task_shell()->Result<(),Box<dyn std::error::Error>>{
    CGroupInitializer::init()?;

    let cg = CGroupBuilder::new("foo")?;
    cg.add_controller("cpu")?;

    cg.attach_task_shell()?;

    Ok(())
}