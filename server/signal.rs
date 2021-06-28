use super::node::Node;
use futures::FutureExt;
use futures_signals::signal::{Signal, SignalExt};

pub struct SignalNode {
	pub(crate) child: Box<Node>,
}

impl SignalNode {
	pub fn new<T, S>(signal: S) -> SignalNode
	where
		T: Into<Node>,
		S: 'static + Unpin + Signal<Item = T>,
	{
		let node = signal.first().to_future().now_or_never().unwrap().into();
		SignalNode {
			child: Box::new(node),
		}
	}
}

impl std::fmt::Display for SignalNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.child)
	}
}
