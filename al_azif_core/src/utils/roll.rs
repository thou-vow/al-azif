use crate::_prelude::*;

pub struct RollExpression {
    pub dices:     i64,
    pub sides:     i64,
    pub advantage: i64,
}
impl RollExpression {
    pub fn new(dices: i64, sides: i64, advantage: i64) -> Self { Self { dices, sides, advantage } }

    pub fn evaluate(&self) -> (i64, RollSummary) {
        let rolls_amount = (self.dices + self.advantage.abs()) as usize;
        let mut rng = rand::thread_rng();
        let mut indexed_rolled_values = Vec::new();

        if self.sides < 1 {
            for i in 0 .. rolls_amount {
                indexed_rolled_values.push((i, 0));
            }
        } else {
            for i in 0 .. rolls_amount {
                indexed_rolled_values.push((i, rng.gen_range(1 .. self.sides + 1)));
            }
        }

        match self.advantage.signum() {
            1 => indexed_rolled_values.sort_by(|(_, a), (_, b)| b.cmp(a)),
            -1 => indexed_rolled_values.sort_by(|(_, a), (_, b)| a.cmp(b)),
            _ => (),
        }

        let mut outcome = 0;
        let mut summary_values = vec![RollSummaryValue::NotSelected(0); rolls_amount];

        let mut iter = indexed_rolled_values.iter();
        for _ in 0 .. self.dices {
            let (i, value) = iter.next().unwrap();
            outcome += *value;
            summary_values[*i] = RollSummaryValue::Selected(*value);
        }
        for (i, value) in iter {
            summary_values[*i] = RollSummaryValue::NotSelected(*value);
        }

        (outcome, RollSummary::new(summary_values))
    }
}

#[derive(Deserialize, Serialize)]
pub struct RollSummary {
    pub values: Vec<RollSummaryValue>,
}
impl RollSummary {
    pub fn new(values: Vec<RollSummaryValue>) -> Self { Self { values } }

    pub fn ansi_code_block(&self) -> String {
        let mut block = String::from("```ansi\n[");

        let Some(first_value) = self.values.first() else {
            block.push_str("]```");
            return block;
        };
        match first_value {
            RollSummaryValue::Selected(num) => block.push_str(&f!("\u{001b}[1;31m{}\u{001b}[0m", mark_thousands(*num))),
            RollSummaryValue::NotSelected(num) => block.push_str(&mark_thousands(*num)),
        }

        for value in self.values.iter().skip(1) {
            match value {
                RollSummaryValue::Selected(num) => block.push_str(&f!("\u{001b}[1;31m{}\u{001b}[0m", mark_thousands(*num))),
                RollSummaryValue::NotSelected(num) => block.push_str(&mark_thousands(*num)),
            }
        }

        block.push_str("]```");

        block
    }

    pub fn ansi_code_block_in_block_quote(&self) -> String {
        let mut block = String::from("> ```ansi\n> [");

        let Some(first_value) = self.values.first() else {
            block.push_str("]```");
            return block;
        };
        match first_value {
            RollSummaryValue::Selected(num) => block.push_str(&f!("\u{001b}[1;32m{}\u{001b}[0m", mark_thousands(*num))),
            RollSummaryValue::NotSelected(num) => block.push_str(&mark_thousands(*num)),
        }

        for value in self.values.iter().skip(1) {
            match value {
                RollSummaryValue::Selected(num) => block.push_str(&f!(", \u{001b}[1;32m{}\u{001b}[0m", mark_thousands(*num))),
                RollSummaryValue::NotSelected(num) => block.push_str(&f!(", {}", mark_thousands(*num))),
            }
        }

        block.push_str("]```");

        block
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RollSummaryValue {
    Selected(i64),
    NotSelected(i64),
}
