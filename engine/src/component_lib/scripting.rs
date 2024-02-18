// Refactor
// -Consider adding functionality for one entity to have multiple scripts.
// -Might just be as simple as the multiple scripts being in one file they load in but it's possible there might be some that for whatever reason need to run separately

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