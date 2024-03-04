use std::rc::Rc;

// Refactor:
// -Might just be as simple as the multiple scripts being in one file they load in but it's possible there might be some that for whatever reason need to run separately
// -Add movement script
// -Add script that can return information to rust?
//  On attack hit it runs the attack's script and that script returns the amount of damage the attack should deal if applicable?
//  -A script's end can be a method on world that calls a script end event so scripts can self terminate early

// Spell Origination tracks parameters like a spell’s Cast ID, Cast Time, and Spell Slot for the entirety of its lifetime

//^ I can replicate this functionality with an entity? the actual entity the script is refering to can just hold whatever data the script needs
//If I ever need a script that does not have an explicit holder like a projectile, etc just make a script holder

pub struct Script {
  start: BaseScript,
  running: BaseScript,
  onhit: BaseScript,
  stop: BaseScript,
}

impl Script {
  pub fn new(start: &str, onhit: &str, running: &str, stop: &str) -> Script {
    let start = BaseScript::new(start);
    let running = BaseScript::new(running);
    let onhit = BaseScript::new(onhit);
    let stop = BaseScript::new(stop);
    Script { start, onhit, running, stop }
  }

  pub fn start(&self) -> Option<Rc<String>> {
    self.start.0.clone()
  }

  pub fn running(&self) -> Option<Rc<String>> {
    self.running.0.clone()
  }

  pub fn onhit(&self) -> Option<Rc<String>> {
    self.onhit.0.clone()
  }

  pub fn stop(&self) -> Option<Rc<String>> {
    self.stop.0.clone()
  }
}

#[derive(Debug, Clone)]
pub struct BaseScript(Option<Rc<String>>);
impl BaseScript {
  pub fn new(script_slice: &str) -> Self {
    BaseScript(Some(Rc::new(String::from(script_slice))))
  }
}
