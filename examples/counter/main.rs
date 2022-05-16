use pinwheel::prelude::*;
use web_sys as dom;

fn main() {
	let window = dom::window().unwrap();
	let document = window.document().unwrap();
	let body = document.body().unwrap();
	let counter = Counter::new();
	App::new(body.into(), counter).forget();
}

struct Counter {
	count: Mutable<usize>,
}

impl Counter {
	pub fn new() -> Counter {
		Counter {
			count: Mutable::new(0),
		}
	}
}

impl Component for Counter {
	fn into_node(self) -> Node {
		let on_click = {
			let count = self.count.clone();
			move |_| {
				count.replace_with(|count| *count + 1);
			}
		};
		let count_text = self
			.count
			.signal()
			.map(|count| p().child(count.to_string()));
		let increment_button = button().onclick(on_click).child("Increment");
		div()
			.child_signal(count_text)
			.child(increment_button)
			.into_node()
	}
}
