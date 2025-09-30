use std::fmt::Debug;

#[derive(Debug, Clone)]
enum Phase<T> {
    MonotonicallyRising,
    MonotonicallyFalling,
    AtValue { value: T },
}

pub struct TimeSeriesPattern<T> {
    phases: Vec<Phase<T>>,
}

impl<T> TimeSeriesPattern<T>
where
    T: Copy + Ord + Debug,
{
    pub fn new() -> Self {
        Self { phases: Vec::new() }
    }

    pub fn monotonically_rising(mut self) -> Self {
        self.phases.push(Phase::MonotonicallyRising);
        self
    }

    pub fn monotonically_falling(mut self) -> Self {
        self.phases.push(Phase::MonotonicallyFalling);
        self
    }

    pub fn at_value(mut self, value: T) -> Self {
        self.phases.push(Phase::AtValue { value });
        self
    }

    pub fn matches(&self, samples: &[T]) -> Result<(), String> {
        let mut view_start = 0;
        let mut view_end = 0;

        assert!(self.phases.len() > 0);
        for (phase_idx, phase) in self.phases.iter().enumerate() {
            let last_phase = match phase_idx {
                0 => None,
                x => self.phases.get(x - 1),
            };

            macro_rules! fail {
              ($($arg:tt)*) => {
                  {
                    let msg = format!($($arg)*);
                  return Err(format!("@{phase:?}{phase_idx:?} {msg}"));
                  }
              };
          }

            use Phase::*;
            match (phase, last_phase) {
                (AtValue { value }, None) => {
                    // using at_value at start
                    view_start = 0;
                    view_end = view_start
                        + samples[view_start..]
                            .iter()
                            .take_while(|x| *x == value)
                            .count()
                        - 1;

                    if view_end - view_start == 0 {
                        fail!("samples do not start with {:?}", value)
                    }
                }

                (MonotonicallyRising | MonotonicallyFalling, None)
                | (MonotonicallyFalling, Some(AtValue { value: _ } | MonotonicallyRising))
                | (MonotonicallyRising, Some(AtValue { value: _ } | MonotonicallyFalling)) => {
                    let is_rise = matches!(phase, MonotonicallyRising);
                    // rising after an at_value
                    //     or after an Falling

                    // doesn't matter where we start in case of at_value
                    // but for Falling case we should start at the minimum
                    // view that supports the pattern.
                    view_start = view_start + 1;
                    view_end = view_start
                        + samples[view_start..]
                            .iter()
                            .enumerate()
                            .take_while(|(i, x)| match (i, is_rise) {
                                (0, _) => true,
                                (_, true) => *x >= &samples[view_start..][*i - 1],
                                (_, false) => *x <= &samples[view_start..][*i - 1],
                            })
                            .count()
                        - 1;
                    if view_end - view_start == 0 {
                        fail!("couldn't find a {phase:?} pattern")
                    }
                }
                (MonotonicallyRising, Some(MonotonicallyRising)) => {
                    fail!("Rising after Rising pattern doesn't make sense")
                }
                (MonotonicallyFalling, Some(MonotonicallyFalling)) => {
                    fail!("Falling after Falling pattern doesn't make sense")
                }
                (AtValue { value }, Some(_)) => {
                    // rising and falling have open ends. just need to find the value
                    // and latch onto that.

                    // if we find the value at view_start the assertion for
                    // the last phase wouldn't be valid anymore because it needs at-least
                    // one sample in it.
                    view_start = view_start + 1;
                    view_start += match samples[view_start..=view_end]
                        .iter()
                        .position(|x| *x == *value)
                    {
                        Some(offset) => offset,
                        None => fail!("can't find value {value:?}"),
                    };
                    view_end = view_start
                        + samples[view_start..]
                            .iter()
                            .take_while(|x| *x == value)
                            .count()
                        - 1;

                    // the following assertion is not necessary because we will fail
                    // in view_start if we don't find the value
                    // if view_end - view_start == 0 {
                    //     fail!("can't find value {value:?}", value)
                    // }
                }
            }
        }

        if view_end < samples.len() - 1 {
            return Err(format!(
                "{} remaining for pattern match",
                samples.len() - view_end - 1
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_pattern() {
        let samples1 = vec![
            0, 0, 0, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 7, 0,
            0, 0, 0,
        ];
        let samples2 = vec![
            0, 0, 0, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 7, 0,
            0, 0, 0,
        ];
        let samples3 = vec![
            0, 0, 0, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 7, 7,
            7, 0, 0, 0, 0,
        ];

        let samples4 = vec![
            0, 0, 0, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 11, 7, 7,
            3, 0, 0, 0, 0,
        ];
        let samples5 = vec![
            0, 0, 0, 9, 9, 9, 9, 9, 9, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 11, 7, 7, 3, 0, 0, 0, 0,
        ];

        // failure
        let samples10 = vec![0, 0, 12, 12, 11, 12, 7, 8, 0];
        let samples11 = vec![0, 0, 12, 12, 11, 11, 12, 7, 8, 0];

        let pattern = TimeSeriesPattern::new()
            .at_value(0) // Starts at 0
            .monotonically_rising() // Rises to peak
            .at_value(12) // Sustains at 12
            .monotonically_falling() // Falls back down
            .at_value(0); // Ends at 0

        pattern.matches(&samples1).expect("Pattern should match");
        pattern.matches(&samples2).expect("Pattern should match");
        pattern.matches(&samples3).expect("Pattern should match");
        pattern.matches(&samples4).expect("Pattern should match");
        pattern.matches(&samples5).expect("Pattern should match");

        assert!(!pattern.matches(&samples10).is_ok());
        assert!(!pattern.matches(&samples11).is_ok());
    }
}
