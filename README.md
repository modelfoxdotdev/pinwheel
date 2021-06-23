<p align="center">
  <img src="pinwheel.svg" title="Pinwheel">
</p>

# Pinwheel

Pinwheel is a library for writing web user interfaces with Rust.

## Example

The example below increments the value in a `<p>` each time a `<button>` is pressed.

```rust
let body = dom::window().unwrap().document().unwrap().body().unwrap().into();
let count = Mutable::new(0);
let on_click = {
  let count = count.clone();
  move |_| {
    count.replace_with(|count| *count + 1);
  }
};
let count_text = count.signal().map(|count| p().child(count.to_string()));
let increment_button = button().onclick(on_click).child("Increment");
let counter = div()
  .style(style::DISPLAY, "grid")
  .style(style::JUSTIFY_CONTENT, "start")
  .child_signal(count_text)
  .child(increment_button);
App::new(body, counter).forget();
```

## Features

### Fine-Grained Reactivity

Pinwheel uses the [futures-signals](https://lib.rs/futures-signals) crate to update exactly the right DOM nodes as your application's state changes. No virtual DOM required!

### Isomorphic Rendering

When compiled for the browser, Pinwheel renders by creating DOM nodes. On the server, it renders by writing HTML to a string.

```rust
let root = p().child("Hello, World!");

// On the server...
assert_eq!(root.to_string(), "<p>Hello, World!</p>");

// On the client...
App::new(dom_node, root).forget();
```

### Partial Hydration

After server rendering, make a subset of your app interactive on the client. In the example below, `dynamic_component` will render on the server and the client, but `static_component` will render only on the server.

```rust
let root = p().child("Hello, World!");

// On the server...
let html = div()
  .child(static_component)
  .child(Dehydrate::new("hydration_id", dynamic_component))
  .to_string();

// On the client...
hydrate("hydration_id");
```

### Macro-free Builders

Pinwheel provides statically typed builders for DOM elements with no macros, so you get all the benefits of `rustfmt` formatting and `rust-analyzer` autocomplete.

```rust
let count_p = count.signal().map(|count| p().child(count.to_string()));
let increment_button = button().onclick(on_click).child("Increment");
let root = div()
  .style(style::DISPLAY, "grid")
  .style(style::JUSTIFY_CONTENT, "start")
  .child_signal(count_p)
  .child(increment_button);
```

### Components

Organize your application into self-contained components.

```rust
use pinwheel::prelude::*;

struct Alert {
  title: String,
  color: Option<String>,
  children: Vec<Node>,
}

impl Component for Alert {
  fn into_node(self) -> Node {
    div()
      .style(style::BACKGROUND_COLOR, self.color)
      .child(h1().child(self.title))
      .children(self.children)
      .into_node()
  }
}
```

### Component Builders

Components frequently have a few required fields and many optional fields. Pinwheel provides a derive macro to make using these components easy.

```rust
#[derive(ComponentBuilder)]
struct Alert {
  // required field
  title: String,
  // optional field
  #[optional]
  color: Option<String>,
  #[children]
  children: Vec<Node>,
}
```

Now, you can make an alert with the builder pattern.

```rust
Alert::new("Alert!")
  .color("green".to_owned())
  .child("An alert occurred!")
```
