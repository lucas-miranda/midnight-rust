mod any_component_query;
pub use any_component_query::AnyComponentQuery;

mod explicit_component_query;
pub use explicit_component_query::ExplicitComponentQuery;

pub trait ComponentQuery {
    fn count(&self) -> usize;
}
