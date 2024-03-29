use crate::message::type_rtype::Rtype;
use crate::domain_name::DomainName;

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
pub fn domain_validity_syntax(domain_name: DomainName) -> Result<DomainName, &'static str> {
    let domain_name_string = domain_name.get_name();
    if domain_name_string.eq("@") {
        return Ok(domain_name);
    }
    let mut empty_label = false;
    for label in domain_name_string.split(".") {
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

/// Given the value of the STYPE, obtains its corresponding string.
pub fn get_string_stype(stype: Rtype) -> String {
    let s_type = Rtype::from_rtype_to_str(stype);
    s_type
}

#[cfg(test)]
mod utils_test {
    use crate::domain_name::DomainName;

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
        let mut expected_domain_name = DomainName::new();
        expected_domain_name.set_name(String::from(""));
        let ok = Ok(expected_domain_name.clone());
        let mut domain_name = DomainName::new();
        let empty_dom = String::from("");
        domain_name.set_name(empty_dom);

        let empty_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(empty_dom_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_valid_dom() {
        let mut expected_domain_name = DomainName::new();
        expected_domain_name.set_name(String::from("label1.label2."));
        let ok = Ok(expected_domain_name);
        let mut domain_name = DomainName::new();
        let valid_dom = String::from("label1.label2.");
        domain_name.set_name(valid_dom);

        let valid_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(valid_dom_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_wrong_middle_dom() {
        let mut domain_name = DomainName::new();
        let wrong_middle_dom = String::from("label1..label2");
        domain_name.set_name(wrong_middle_dom.clone());
        let wrong_middle_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(
            wrong_middle_dom_validity,
            Err("Error: Empty label is only allowed at the end of a hostname.")
        );
    }

    #[test]
    fn domain_validity_syntax_wrong_init_dom() {
        let mut domain_name = DomainName::new();
        let wrong_init_dom = String::from(".label");
        domain_name.set_name(wrong_init_dom);
        let wrong_init_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(
            wrong_init_dom_validity,
            Err("Error: Empty label is only allowed at the end of a hostname.")
        );
    }

    #[test]
    fn domain_validity_syntax_at_domain_name() {
        let mut domain_name = DomainName::new();
        let at_str = String::from("@");
        domain_name.set_name(at_str.clone());
        let ok = Ok(domain_name.clone());
        let at_str_validity = domain_validity_syntax(domain_name);

        assert_eq!(at_str_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_syntactically_incorrect_dom() {
        let mut domain_name = DomainName::new();
        let incorrect_dom = String::from("label1.2badlabel.test");
        domain_name.set_name(incorrect_dom.clone());
        let incorrect_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(
            incorrect_dom_validity,
            Err("Error: present domain name is not syntactically correct.")
        );
    }

    #[test]
    fn domain_validity_syntax_syntactically_correct_dom() {
        let mut domain_name_1 = DomainName::new();
        let correct_dom_1 = String::from("label1.label2.test");
        domain_name_1.set_name(correct_dom_1.clone());

        let mut domain_name_2 = DomainName::new();
        let correct_dom_2 = String::from("label1.label2.test.");
        domain_name_2.set_name(correct_dom_2.clone());

        let ok_dom_1 = Ok(domain_name_1.clone());
        let ok_dom_2 = Ok(domain_name_2.clone());
        let correct_dom_1_validity = domain_validity_syntax(domain_name_1);
        let correct_dom_2_validity = domain_validity_syntax(domain_name_2);

        assert_eq!(correct_dom_1_validity, ok_dom_1);
        assert_eq!(correct_dom_2_validity, ok_dom_2);
    }
}
