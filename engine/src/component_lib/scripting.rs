// Refactor
// -Consider adding functionality for one entity to have multiple scripts.
// -Might just be as simple as the multiple scripts being in one file they load in but it's possible there might be some that for whatever reason need to run separately

use std::{collections::HashMap, rc::Rc};

#[derive(Debug,Clone)]
pub struct BaseScript(Rc<String>);
impl BaseScript{
  pub fn new(script_slice:&str) -> Self {
    BaseScript(Rc::new(String::from(script_slice)))
  }
}

pub struct AutoAttackScripts(HashMap<String,BaseScript>);

impl AutoAttackScripts{
  pub fn new(start:&str,on_hit:&str) -> Self {
    let mut scripts = HashMap::new();
    scripts.insert(String::from("AutoAttackStart"), BaseScript::new(start));
    scripts.insert(String::from("OnAutoAttackHit"), BaseScript::new(on_hit));
    AutoAttackScripts(scripts)
  }

  pub fn add_script(&mut self, name:&str,script:&str){
    self.0.insert(String::from(name), BaseScript::new(script));
  }

  pub fn get_script(&self, name:&str) -> BaseScript {
    self.0.get(name).unwrap().clone()
  }
}

#[derive(Debug, Clone, Default)]
pub struct AutoAttackScript(String);

impl AutoAttackScript {
  pub fn new(script:&str) -> Self{
    // let scripts:Vec<String> = scripts.iter().map(|&str| str.into()).collect();
    AutoAttackScript(script.to_owned())
  }
  pub fn script(&self) -> &str {
    &self.0
  }
}


#[derive(Debug, Clone, Default)]
pub struct MovementScript(String);

impl MovementScript {
  pub fn new(script:&str) -> Self{
    // let scripts:Vec<String> = scripts.iter().map(|&str| str.into()).collect();
    MovementScript(script.to_owned())
  }
  pub fn script(&self) -> &str {
    &self.0
  }
}