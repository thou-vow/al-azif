use rand::Rng;
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod roll {
    use super::*;

    pub fn execute_roll_expression(dices: i64, sides: i64, advantage: i64) -> (i64, String) {
        let rolls_amount = (dices + advantage.abs()) as usize;
        let mut rng = rand::thread_rng();
        let mut indexed_rolled_values = Vec::new();

        if sides < 1 {
            for i in 0..rolls_amount {
                indexed_rolled_values.push((i, 0));
            }
        } else {
            for i in 0..rolls_amount {
                indexed_rolled_values.push((i, rng.gen_range(1..=sides)));
            }
        }

        match advantage.signum() {
            1 => indexed_rolled_values.sort_by(|(_, a), (_, b)| b.cmp(a)),
            -1 => indexed_rolled_values.sort_by(|(_, a), (_, b)| a.cmp(b)),
            _ => (),
        }

        let mut outcome = 0;
        let mut summary_values = vec![RollSummaryValue::NotSelected(0); rolls_amount];

        let mut iter = indexed_rolled_values.iter();
        for _ in 0..dices {
            let (i, value) = iter.next().unwrap();
            outcome += *value;
            summary_values[*i as usize] = RollSummaryValue::Selected(*value);
        }
        for (i, value) in iter {
            summary_values[*i as usize] = RollSummaryValue::NotSelected(*value);
        }

        (outcome, RollSummary::new(summary_values).to_string())
    }

    struct RollSummary {
        pub values: Vec<RollSummaryValue>,
    }
    impl RollSummary {
        pub fn new(values: Vec<RollSummaryValue>) -> Self {
            Self { values }
        }
    }
    impl Display for RollSummary {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;

            let Some(first_value) = self.values.first() else {
                write!(f, "]")?;

                return Ok(());
            };
            write!(f, "{first_value}")?;

            for value in self.values.iter().skip(1) {
                write!(f, ", {value}")?;
            }

            write!(f, "]")?;

            Ok(())
        }
    }

    #[derive(Clone, Debug)]
    enum RollSummaryValue {
        Selected(i64),
        NotSelected(i64),
    }
    impl Display for RollSummaryValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                RollSummaryValue::Selected(value) => write!(f, "**{value}**"),
                RollSummaryValue::NotSelected(value) => write!(f, "{value}"),
            }
        }
    }
}
