// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use action::Action;

/// A sequence of [actions].
///
/// This sequence is also an [`Action`] itself, so sequences
/// can be nested within themselves to provide some structure
/// or hierarchy.
///
/// [actions]: trait.Action.html
pub struct Sequence {
    actions: Vec<Box<Action>>,
    step: Option<usize>,
}

impl Action for Sequence {
    fn execute(&self) {
        for action in &self.actions {
            action.execute();
        }
    }

    fn step(&mut self) -> bool {
        let previous = self.step;
        let next = match previous {
            Some(step) => step + 1,
            None => 0,
        };
        self.step = Some(next);
        if next < self.actions.len() {
            self.actions[next].execute();
        }
        next >= self.actions.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}