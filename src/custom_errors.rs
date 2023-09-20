use thiserror::Error;

#[derive(Debug,Error)]
pub enum CustomErrors {
	#[error("Attempting to add component to an entitity without calling create component first!")]
	CreateComponentNeverCalled,
  #[error("Attempted to use an unregisted component")]
	ComponentNotRegistered, 
	#[error("Attempted to reference an entity that does not exist")]
	EntityDoesNotExist
}