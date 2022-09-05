// if2.rs

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.

// I AM DONE

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "shit" {
        "vince"
    } else if fizzish == "potatoes" {
        "sophia"
    } else {
        "cunt"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vince_for_shit() {
        assert_eq!(foo_if_fizz("shit"), "vince")
    }

    #[test]
    fn sophia_for_potatoes() {
        assert_eq!(foo_if_fizz("potatoes"), "sophia")
    }

    #[test]
    fn default_to_cunt() {
        assert_eq!(foo_if_fizz("literally anything"), "cunt")
    }
}
