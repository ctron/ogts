use yew_nested_router::Target;

mod child;
mod index;

pub use child::*;
pub use index::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    #[target(index)]
    Index,
    Child,
}
