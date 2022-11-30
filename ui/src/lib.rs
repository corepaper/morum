use serde::{Serialize, Deserialize};
use east::HydrateTo;

#[derive(Serialize, Deserialize, HydrateTo, Debug, Clone)]
pub enum AnyComponent {

}
