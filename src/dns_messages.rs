fn create_header_section() -> str {
    let header = "00000000";
    header
}

fn create_question_section() -> str {
    let question = "00000000";
    question
}

fn create_answer_section() -> str {
    let answer = "00000000";
    answer
}

fn create_authority_section() -> str {
    let authority = "00000000";
    authority
}

fn create_additional_section() -> str {
    let additional = "00000000";
    additional
}

fn create_dns_message() -> str {
    let header = create_header_section();
    let question = create_question_section();
    let answer = create_answer_section();
    let authority = create_authority_section();
    let additional = create_additional_section();
}
