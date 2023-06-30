use crate::message::rtype::Rtype;

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
pub fn domain_validity_syntax(domain_name: String) -> Result<String, &'static str> {
    if domain_name.eq("@") {
        return Ok(domain_name);
    }
    let mut empty_label = false;
    for label in domain_name.split(".") {
        if empty_label {
            return Err("Error: Empty label is only allowed at the end of a hostname.");
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

// checks if host_name is writtena as an reverse query
pub fn is_reverse_query(host_name: String) -> bool {
    let mut is_reverse_query: bool = false;
    let labels = host_name.split(".");
    let length_ip = host_name.split(".").count();

    if length_ip != 4 {
        return is_reverse_query;
    }

    for label in labels {
        let label_char = label.chars();

        //if it's reverse query should be a number
        for char in label_char {
            //verified if it's a number
            is_reverse_query = char.is_ascii_digit();

            //if not a number is not a reverse query
            if is_reverse_query == false {
                return is_reverse_query;
            }
        }
    }

    return is_reverse_query;
}

/// Given the value of the STYPE, obtains its corresponding string.
pub fn get_string_stype(stype: Rtype) -> String {
    let s_type = Rtype::from_rtype_to_str(stype);
    s_type
}

#[cfg(test)]
mod utils_test {
    use crate::utils::is_reverse_query;

    use super::check_label_name;
    use super::domain_validity_syntax;

    #[test]
    fn check_label_name_empty_label() {
        let cln_empty_str = check_label_name(String::from(""));
        assert_eq!(cln_empty_str, false);
    }

    #[test]
    fn check_label_name_large_label() {
        let cln_large_str = check_label_name(String::from(
            "this-is-a-extremely-large-label-that-have-exactly--64-characters",
        ));
        assert_eq!(cln_large_str, false);
    }

    #[test]
    fn check_label_name_first_label_character() {
        let cln_symbol_str = check_label_name(String::from("-label"));
        assert_eq!(cln_symbol_str, false);

        let cln_num_str = check_label_name(String::from("0label"));
        assert_eq!(cln_num_str, false);
    }

    #[test]
    fn check_label_name_last_label_character() {
        let cln_symbol_str = check_label_name(String::from("label-"));
        assert_eq!(cln_symbol_str, false);

        let cln_num_str = check_label_name(String::from("label2"));
        assert_eq!(cln_num_str, true);
    }

    #[test]
    fn check_label_name_interior_label_characters() {
        let cln_dot_str = check_label_name(String::from("label.test"));
        assert_eq!(cln_dot_str, false);

        let cln_space_str = check_label_name(String::from("label test"));
        assert_eq!(cln_space_str, false);
    }

    #[test]
    fn check_label_name_valid_label() {
        let cln_valid_str = check_label_name(String::from("label0test"));
        assert_eq!(cln_valid_str, true);
    }

    #[test]
    fn domain_validity_syntax_empty_dom() {
        let ok = Ok(String::from(""));
        let empty_dom = String::from("");

        let empty_dom_validity = domain_validity_syntax(empty_dom);

        assert_eq!(empty_dom_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_valid_dom() {
        let ok = Ok(String::from("label1.label2."));
        let valid_dom = String::from("label1.label2.");

        let valid_dom_validity = domain_validity_syntax(valid_dom);

        assert_eq!(valid_dom_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_wrong_middle_dom() {
        let wrong_middle_dom = String::from("label1..label2");
        let wrong_middle_dom_validity = domain_validity_syntax(wrong_middle_dom);

        assert_eq!(
            wrong_middle_dom_validity,
            Err("Error: Empty label is only allowed at the end of a hostname.")
        );
    }

    #[test]
    fn domain_validity_syntax_wrong_init_dom() {
        let wrong_init_dom = String::from(".label");
        let wrong_init_dom_validity = domain_validity_syntax(wrong_init_dom);

        assert_eq!(
            wrong_init_dom_validity,
            Err("Error: Empty label is only allowed at the end of a hostname.")
        );
    }

    #[test]
    fn domain_validity_syntax_at_domain_name() {
        let at_str = String::from("@");
        let ok = Ok(at_str.clone());
        let at_str_validity = domain_validity_syntax(at_str);

        assert_eq!(at_str_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_syntactically_incorrect_dom() {
        let incorrect_dom = String::from("label1.2badlabel.test");
        let incorrect_dom_validity = domain_validity_syntax(incorrect_dom);

        assert_eq!(
            incorrect_dom_validity,
            Err("Error: present domain name is not syntactically correct.")
        );
    }

    #[test]
    fn domain_validity_syntax_syntactically_correct_dom() {
        let correct_dom_1 = String::from("label1.label2.test");
        let correct_dom_2 = String::from("label1.label2.test.");

        let ok_dom_1 = Ok(correct_dom_1.clone());
        let ok_dom_2 = Ok(correct_dom_2.clone());
        let correct_dom_1_validity = domain_validity_syntax(correct_dom_1);
        let correct_dom_2_validity = domain_validity_syntax(correct_dom_2);

        assert_eq!(correct_dom_1_validity, ok_dom_1);
        assert_eq!(correct_dom_2_validity, ok_dom_2);
    }

    #[test]
    fn is_reverse_query_dom() {
        let dom_str = String::from("not_inverse.com");
        assert_eq!(is_reverse_query(dom_str), false);
    }

    #[test]
    fn is_reverse_query_ip() {
        let ip_str = String::from("10.1.0.52");
        assert_eq!(is_reverse_query(ip_str), true);
    }

    #[test]
    fn is_reverse_query_num() {
        let num_str = String::from("100");
        assert_eq!(is_reverse_query(num_str), false);
    }
}
