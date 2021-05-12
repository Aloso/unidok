use unidoc_repr::ir::IrState;

mod block;
mod helpers;
mod html;
mod macros;
mod segment;

pub trait IntoNode<'a> {
    fn into_node(self, state: &IrState<'a>) -> crate::Node<'a>;
}

pub trait IntoNodes<'a> {
    fn into_nodes(self, state: &IrState<'a>) -> Vec<crate::Node<'a>>;
}

impl<'a, T: IntoNode<'a>> IntoNodes<'a> for Vec<T> {
    fn into_nodes(self, state: &IrState<'a>) -> Vec<crate::Node<'a>> {
        self.into_iter().map(|n| n.into_node(state)).collect()
    }
}
