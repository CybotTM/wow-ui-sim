//! Ergonomic bridge layer between rilua's stack API and wow-ui-sim.
//!
//! Provides typed extraction/injection traits and macros that give
//! mlua-like method registration ergonomics without FFI overhead.
//!
//! # Design
//!
//! rilua exposes `RustFn = fn(&mut LuaState) -> LuaResult<u32>`. Arguments
//! live at `state.stack[state.base - 1 .. state.top]` (base points to the
//! first register / arg 0). This layer wraps that convention with typed
//! [`FromStack`] and [`IntoStack`] traits so call sites don't manually
//! index the stack.

mod from_stack;
mod into_stack;
mod macros;
mod table_builder;

pub(crate) use from_stack::stack_val;
pub use from_stack::FrameArena;
pub use from_stack::FrameObject;
pub use from_stack::FrameRef;
pub use from_stack::FromMethodSelf;
pub use from_stack::FromStack;
pub use into_stack::IntoStack;
pub use table_builder::create_frame_table;
pub(crate) use table_builder::table_set_rust_fn;
pub use table_builder::TableBuilder;
