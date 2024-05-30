use std::rc::Rc;

// Refactor:
// -Might just be as simple as the multiple scripts being in one file they load
// in but it's possible there might be some that for whatever reason need to run
// separately -Add movement script
// -Add script that can return information to rust?
//  On attack hit it runs the attack's script and that script returns the amount
// of damage the attack should deal if applicable? -A script's end can be a
// method on world that calls a script end event so scripts can self terminate
// early -Make the option in script wrap around the base script instead of
// having the base script hold the option

// Spell Origination tracks parameters like a spell’s Cast ID, Cast Time, and
// Spell Slot for the entirety of its lifetime

//^ I can replicate this functionality with an entity? the actual entity the
//^ script is refering to can just hold whatever data the script needs
//If I ever need a script that does not have an explicit holder like a
// projectile, etc just make a script holder

#[derive(Debug, Default, Clone)]
pub struct Script {
  start:Option<BaseScript>,
  running:Option<BaseScript>,
  onhit:Option<BaseScript>,
  stop:Option<BaseScript>
}

impl Script {
  pub fn new(start:Option<&str>, onhit:Option<&str>, running:Option<&str>, stop:Option<&str>) -> Script {
    let start = BaseScript::new(start);
    let running = BaseScript::new(running);
    let onhit = BaseScript::new(onhit);
    let stop = BaseScript::new(stop);
    Script { start, onhit, running, stop }
  }

  pub fn start(&self) -> Option<BaseScript> {
    self.start.clone()
  }

  pub fn running(&self) -> Option<BaseScript> {
    self.running.clone()
  }

  pub fn onhit(&self) -> Option<BaseScript> {
    self.onhit.clone()
  }

  pub fn stop(&self) -> Option<BaseScript> {
    self.stop.clone()
  }
}

#[derive(Debug, Clone)]
pub struct BaseScript(pub Rc<String>);
impl BaseScript {
  ///Matches the string slice to return an `Option` wrapping a [`BaseScript`].
  pub fn new(script_slice:Option<&str>) -> Option<Self> {
    match script_slice {
      Some(string) => Some(BaseScript(Rc::new(String::from(string)))),
      None => None
    }
  }
}

//script entity can hold a "running script" and track information needed such
// as if a target has reached a knockback desination or a cc timer has run out
// or if a toggle has been toggled
pub struct PersistentScript;
pub struct RunningScript {
  pub running:Option<BaseScript>,
  pub stop:Option<BaseScript>
}

impl RunningScript {
  pub fn new(running:Option<BaseScript>, stop:Option<BaseScript>) -> Self {
    // let running = Some(running.0);
    // let stop = Some(stop.0);
    RunningScript { running, stop }
  }

  pub fn running(&self) -> Option<BaseScript> {
    self.running.clone()
  }

  pub fn stop(&self) -> Option<BaseScript> {
    self.stop.clone()
  }
}
