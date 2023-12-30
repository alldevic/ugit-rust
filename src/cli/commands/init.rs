pub fn init() -> () {
  let _ = data::init();
  let current_dir = utils::get_current_working_dir();
  println!("Initialized empty ugit repository in {}/{}", current_dir, data::GIT_DIR);
}
