//! Access registers for Armv8-R only

mod cbar;
#[doc(inline)]
pub use cbar::Cbar;

mod hactlr;
#[doc(inline)]
pub use hactlr::Hactlr;

mod hvbar;
#[doc(inline)]
pub use hvbar::Hvbar;

mod vbar;
#[doc(inline)]
pub use vbar::Vbar;
