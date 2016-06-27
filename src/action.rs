// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// An action to be executed.
pub trait Action {
    /// Execute the action.
    ///
    /// This is typically invoked by the [`Sequence`].
    ///
    /// [`Sequence`]: struct.Sequence.html
    fn execute(&self);

    /// Execute one step of an action.
    ///
    /// Some actions may consist of multiple steps and the
    /// user may want to walk through them individually.
    ///
    /// This is also used for stepping through a [`Sequence`]
    /// of actions.
    ///
    /// [`Sequence`]: struct.Sequence.html
    fn step(&mut self) -> bool {
        self.execute();
        true
    }
}