use pest::{iterators::Pair, RuleType};
/// walk a
pub fn walk<Callback, State, Rule>(
    pair: Pair<Rule>,
    initial: State,
    mut callback: Callback,
) -> State
where
    State: Copy,
    Rule: RuleType,
    Callback: Copy + FnMut(&Pair<Rule>, State) -> State,
{
    let mut inner = initial;
    for pair in pair.clone().into_inner() {
        inner = walk(pair, inner, callback);
    }
    return callback(&pair, initial);
}

pub fn contains<Rule: RuleType>(pair: Pair<Rule>, rules: Vec<Rule>) -> bool {
    let callback = |pair: &Pair<Rule>, n_found: usize| {
        for rule in rules.as_slice() {
            if &pair.as_rule() == rule {
                return n_found + 1;
            }
        }
        return n_found;
    };
    let n_found = walk(pair, 0, callback);
    return n_found > 0;
}
