use crate::Node;

mod block;
mod helpers;
mod html;
mod segment;

pub trait IntoNode<'a> {
    fn into_node(self) -> Node<'a>;
}

pub trait IntoNodes<'a> {
    fn into_nodes(self) -> Vec<Node<'a>>;
}

impl<'a, T: IntoNode<'a>> IntoNodes<'a> for Vec<T> {
    fn into_nodes(self) -> Vec<Node<'a>> {
        self.into_iter().map(IntoNode::into_node).collect()
    }
}
