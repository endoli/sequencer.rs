// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use action::{Action, StepStatus};

/// A sequence of [actions].
///
/// This sequence is also an [`Action`] itself, so sequences
/// can be nested within themselves to provide some structure
/// or hierarchy.
///
/// [actions]: trait.Action.html
pub struct Sequence {
    actions: Vec<Box<dyn Action>>,
    step: Option<usize>,
}

impl Action for Sequence {
    fn execute(&self) {
        for action in &self.actions {
            action.execute();
        }
    }

    fn step(&mut self) -> StepStatus {
        let previous = self.step;
        let next = match previous {
            Some(step) => step + 1,
            None => 0,
        };
        if next < self.actions.len() {
            let r = self.actions[next].step();
            if r == StepStatus::Complete {
                self.step = Some(next);
            }
            r
        } else {
            StepStatus::Complete
        }
    }

    fn summary(&self) -> String {
        String::from("Sequence")
    }

    fn description(&self) -> String {
        String::from("A sequence of actions.")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
