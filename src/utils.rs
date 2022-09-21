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
        if ! check_label_name(label.to_string()) {
            println!("L: {}", label);
            return Err("Error: present domain name is not syntactically correct.");
        }
    }
    return Ok(domain_name);
}

mod test {
    use crate::utils::check_label_name;

    #[test]
    fn check_empty_label_test() {
        assert_eq!(check_label_name(String::from("")), false);
    } 

    #[test]
    fn check_large_label_test() {
        assert_eq!(check_label_name(String::from("large-label-teeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeet")), false);
    }

}