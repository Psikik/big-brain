/*!
Pickers are used by Thinkers to determine which of its Scorers will "win".
*/

use bevy::prelude::*;

use crate::{choices::Choice, scorers::Score};

/**
Required trait for Pickers. A Picker is given a slice of choices and a query that can be passed into `Choice::calculate`.

Implementations of `pick` must return `Some(Choice)` for the `Choice` that was picked, or `None`.
 */
pub trait Picker: std::fmt::Debug + Sync + Send {
    fn pick(&self, _choices: &[Choice], _utilities: &Query<&Score>) -> Option<Choice>;
}

/**
Picker that chooses the first `Choice` with a [`Score`] higher than its configured `threshold`.

### Example

```no_run
Thinker::build()
    .picker(FirstToScore::new(.8))
    // .when(...)
```
 */
#[derive(Debug, Clone, Default)]
pub struct FirstToScore {
    pub threshold: f32,
}

impl FirstToScore {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }
}

impl Picker for FirstToScore {
    fn pick(&self, choices: &[Choice], scores: &Query<&Score>) -> Option<Choice> {
        for choice in choices {
            let value = choice.calculate(scores);
            if value >= self.threshold {
                return Some(choice.clone());
            }
        }
        None
    }
}

/**
Picker that chooses the highest `Choice` with a [`Score`] equal to or higher
than its configured `threshold`. Will select the first `Choice` if multiple
choices tie for the highest score.

### Example

```no_run
Thinker::build()
    .picker(HighestScore::new(.0))
    // .when(...)
```
*/
#[derive(Debug, Clone, Default)]
pub struct HighestScore {
    pub threshold: f32,
}

impl HighestScore {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }
}

impl Picker for HighestScore {
    fn pick(&self, choices: &[Choice], scores: &Query<&Score>) -> Option<Choice> {
        let mut highest_choice_score: f32 = 0.;
        let mut highest_choice: Option<Choice> = None;

        for choice in choices {
            let value = choice.calculate(scores);
            if value >= self.threshold && value > highest_choice_score {
                highest_choice_score = value;
                highest_choice = Some(choice.clone());
            }
        }

        highest_choice
    }
}
