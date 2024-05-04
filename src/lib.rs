pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let bracket_map: [(char, char); 3] = [(')', '('), ('}', '{'), (']', '[')];

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' | '}' | ']' => {
                if let Some(&top) = stack.last() {
                    if let Some(&expected) = bracket_map.iter().find(|&&m| m.0 == c) {
                        if top != expected.1 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}
