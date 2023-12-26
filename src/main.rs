use std::env;
use std::collections::HashMap;

mod data;
mod utils;

type Callback = fn(String) -> Option<()>;
struct EventHandler {
    command: HashMap<String, Callback>
}

impl EventHandler {
    fn add_user_function(&mut self, name: String, func: Callback) {
        self.command.insert(name, func);
    }

    fn on_script_call(&mut self, name: &str, argv: &[String]) -> Option<()> {
        let args = argv.iter().map(|ref x| format!("{}", &x)).collect::<Vec<String>>().join(", ");
        self.command[name](args);
        None
    }
}

fn init(_arg: String) -> Option<()> {
    let _ = data::init();
    let current_dir = utils::get_current_working_dir();
    println!("Initialized empty ugit repository in {}//{}", current_dir, data::GIT_DIR);
    None
}

fn parse_args() {
    let mut handler = EventHandler { command: HashMap::new() };
    handler.add_user_function("init".to_string(), init);
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", env::args());
    handler.on_script_call(&args[1], &args[1..]);
}

fn main() {
    parse_args()
}
