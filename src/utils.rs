pub fn check_label_name(name: String) -> bool {
    if name.len() > 63 || name.len() == 0 {
        return false;
    }

    for (i, c) in name.chars().enumerate() {
        if i == 0 && !c.is_ascii_alphabetic() {
            return false;
        } else if i == name.len() - 1 && !c.is_ascii_alphanumeric() {
            return false;
        } else if !(c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }

    return true;
}

// validity checks should be performed insuring that the file is syntactically correct
pub fn domain_validity_syntax(domain_name: String)-> Result<String, &'static str> {
    if domain_name.eq("@") {
        return Ok(domain_name);
    }
    let mut empty_label = false;
    for label in domain_name.split("."){
        if empty_label {
            return Err("Error: Empty label is only allowed at the end os a hostname.")
        }
        if label.is_empty() {
            empty_label = true;
            continue;
        }
        if !check_label_name(label.to_string()) {
            println!("L: {}", label);
            return Err("Error: present domain name is not syntactically correct.");
        }
    }
    return Ok(domain_name);
}

#[cfg(test)]
mod utils_test {
    use super::check_label_name;
    use super::domain_validity_syntax;

    // check_label_name Tests
    #[test]
    fn check_empty_label_test() {
        assert_eq!(check_label_name(String::from("")), false);
    } 

    #[test]
    fn check_large_label_test() {
        assert_eq!(check_label_name(String::from("this-is-a-extremely-large-label-that-have-exactly--64-characters")), false);
    }

    #[test]
    fn check_first_label_character_test() {
        assert_eq!(check_label_name(String::from("-label")), false);
        assert_eq!(check_label_name(String::from("0label")), false);
    }

    #[test]
    fn check_last_label_character_test() {
        assert_eq!(check_label_name(String::from("label-")), false);
        assert_eq!(check_label_name(String::from("label2")), true);
    }

    #[test]
    fn check_interior_label_characters_test() {
        assert_eq!(check_label_name(String::from("label.test")), false);
        assert_eq!(check_label_name(String::from("label test")), false);
    }

    #[test]
    fn check_valid_label_test() {
        assert_eq!(check_label_name(String::from("label0test")), true);
    }

    // domain_validity_syntax Tests
    #[test]
    fn check_empty_domain_test() {
        assert_eq!(domain_validity_syntax(String::from("")), Ok(String::from("")));
        assert_eq!(domain_validity_syntax(String::from("label1..label2")), Err("Error: Empty label is only allowed at the end os a hostname."));
        assert_eq!(domain_validity_syntax(String::from(".label")), Err("Error: Empty label is only allowed at the end os a hostname."));
        assert_eq!(domain_validity_syntax(String::from("label1.label2.")), Ok(String::from("label1.label2.")));
    }

    #[test]
    fn at_domain_name_validity_test() {
        assert_eq!(domain_validity_syntax(String::from("@")), Ok(String::from("@")));
    }
    
    #[test]
    fn syntactically_incorrect_domain_name_test() {
        assert_eq!(domain_validity_syntax(String::from("label1.2badlabel.test")), Err("Error: present domain name is not syntactically correct."));
    }

    #[test]
    fn syntactically_correct_domain_name_test() {
        assert_eq!(domain_validity_syntax(String::from("label1.label2.test")), Ok(String::from("label1.label2.test")));
        assert_eq!(domain_validity_syntax(String::from("label1.label2.test.")), Ok(String::from("label1.label2.test.")));
    }
}