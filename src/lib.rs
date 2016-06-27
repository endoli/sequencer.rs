// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Sequencer
//!
//! Sequencer provides a mechanism for executing sequences of
//! actions in a flexible way.
//!
//! Sequencer is useful for automating some types of workflow
//! or as the basis for a macro system for an application.
//!
//! Actions can provide textual descriptions of what they are and
//! what they are doing, so the application can provide a richer
//! UI, informing the user of what is going to happen.
//!
//! Possible uses:
//!
//! * Sequencing animations or effects.
//! * Recording actions as they're performed within an application
//!   for playback, editing or repeating. An example of this can
//!   be seen in [Bret Victor]'s [Talk Addendum] about his Drawing Dynamic
//!   Visualizations work as well as applications like [Reform] which have
//!   been inspired by it. (This approach was also used in [Data Wrangler].)
//!
//! Currently, only actions or sequences of actions can be
//! executed. Sequences can be nested.
//!
//! Near term future plans:
//!
//! * Add a `Loop` action which has a child [`Action`], which
//!   can be a [`Sequence`]. This would provide for looping
//!   until a terminating condition is true.
//! * Add a `Range` action which is similar to `Loop`, but
//!   is explicitly iterating over the elements of a
//!   collection or a range of numbers, such as 1 through 10.
//! * Add a `Timeline` action which allows for scheduling
//!   actions to be executed at particular points in time.
//!   It should be executed via [`step`] rather than [`execute`].
//!
//! Some open questions:
//!
//! * How might this integrate with threading? Should we support
//!   any sort of cross-thread interaction?
//! * Integration with the markup library so that descriptions
//!   are rich text.
//! * Whether or not we should deal with return values from
//!   an action and allow them to be fed into a subsequent action.
//! * Consider removing [`execute`] and only having [`step`].
//!
//! [`Action`]: trait.Action.html
//! [Bret Victor]: http://worrydream.com/
//! [Data Wrangler]: http://vis.stanford.edu/wrangler/
//! [`execute`]: trait.Action.html#tymethod.execute
//! [Reform]: https://github.com/laszlokorte/reform-swift
//! [`Sequence`]: struct.Sequence.html
//! [`step`]: trait.Action.html#method.step
//! [Talk Addendum]: http://worrydream.com/DrawingDynamicVisualizationsTalkAddendum/

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

mod action;
mod sequence;

pub use self::action::Action;
pub use self::sequence::Sequence;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
