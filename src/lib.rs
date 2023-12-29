pub fn is_balanced_1(expression: &str) -> bool {
    let mut balance = 0;

    for p in expression.chars() {
        if p == '(' {
            balance += 1
        } else {
            balance -= 1;
        }

        if balance < 0 {
            return false;
        }
    }

    balance == 0
}

pub fn is_balanced_2(expression: &str) -> bool {
    let mut balance = 0;

    for p in expression.chars() {
        if p == '(' {
            balance += 1
        } else if p == ')' {
            balance -= 1;
        }

        if balance < 0 {
            return false;
        }
    }

    balance == 0
}

pub fn bracket_partner(expression: &str, pos: usize) -> Option<usize> {
    let mut balance = 0;
    let mut chars = expression
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>();
    let mut pos = pos;

    if chars[pos].1 == ')' {
        chars.reverse();
        pos = chars.len() - pos - 1;
    }

    for (i, p) in &chars[pos..] {
        if p == &'(' {
            balance += 1
        } else if p == &')' {
            balance -= 1;
        }

        if balance == 0 {
            return Some(*i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_balanced_1_test() {
        assert_eq!(is_balanced_1("(()()()())"), true);
        assert_eq!(is_balanced_1("(((())))"), true);
        assert_eq!(is_balanced_1("(()((())()))"), true);
        assert_eq!(is_balanced_1("((((((())"), false);
        assert_eq!(is_balanced_1("()))"), false);
        assert_eq!(is_balanced_1("(()()(()"), false);
    }

    #[test]
    fn is_balanced_2_test() {
        assert_eq!(is_balanced_2("(()()()())"), true);
        assert_eq!(is_balanced_2("((((((())"), false);
        assert_eq!(is_balanced_2("(a(aw)gted()(fes)(se))th"), true);
        assert_eq!(is_balanced_2("(grd((yhtf()aw)es))"), true);
        assert_eq!(is_balanced_2("((g)(grd(es())th()aw))"), true);
        assert_eq!(is_balanced_2("((ttd((ef(ea((ese))"), false);
        assert_eq!(is_balanced_2("esfes(fes))fes)"), false);
        assert_eq!(is_balanced_2("(fes()fes(eses)((sef)"), false);
    }

    #[test]
    fn bracket_partner_test() {
        assert_eq!(bracket_partner("(()()()())", 0), Some(9));
        assert_eq!(bracket_partner("(()()()())", 2), Some(1));
        assert_eq!(bracket_partner("(((())))", 3), Some(4));
        assert_eq!(bracket_partner("(((())))", 4), Some(3));
        assert_eq!(bracket_partner("(()((())()))", 4), Some(7));
        assert_eq!(bracket_partner("(()((())()))", 10), Some(3));
        assert_eq!(bracket_partner("((((((())", 5), Some(8));
        assert_eq!(bracket_partner("()))", 2), None);
        assert_eq!(bracket_partner("(()()(()", 0), None);
    }
}
