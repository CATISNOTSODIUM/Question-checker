pub mod response_type;
pub mod question_type;

//question id format Q_[skill-id]_[question-index]
pub fn check_question_id_format(input: &String) -> Result<(), Box<dyn std::error::Error>> {
    let question_id_parts: Vec<_> = input.split('_').collect();
    if question_id_parts.len() != 3 {
        return Err("Invalid question id (format: Q_[skill-id]_[question-index])")?;
    }

    if question_id_parts[0] != "Q" {
        return Err("Question id prefix must start with Q")?;
    }

    // check skill id
    match question_id_parts[1].parse::<u32>() {
        Ok(_) => (),
        Err(_) => return Err("Invalid skill id. Skill id must be in u32.")?,
    }

    Ok(())
}