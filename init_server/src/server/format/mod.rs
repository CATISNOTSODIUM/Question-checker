pub mod response_type;
pub mod question_type;

pub fn check_question_id_format(input: &String) -> Result<(), Box<dyn std::error::Error>> {
    let question_id_parts: Vec<_> = input.split('_').collect();
    if question_id_parts.len() != 4 {
        return Err("Invalid question id")?;
    }

    // check competency id (ABC-123)
    let competency_parts: Vec<_> = question_id_parts[0].split('-').collect();
    if competency_parts.len() != 2 {
        return Err("Invalid competency id: the competency must contain three alphabetical characters, \
        followed by a hyphen and three numerical characters. (eg. ABC-123)")?;
    }

    //check if all characters in the first part of the competency id are alphabetical characters.
    match competency_parts[0].chars().all(|c| c.is_alphabetic()) {
        true => (),
        false => return Err("Invalid competency id: the first part of the competency id must contain only alphabetical characters.")?,
    }

    //check if all characters in the second part of the competency id are numerical characters.
    match competency_parts[1].parse::<u32>() {
        Ok(_) => (),
        Err(_) => return Err("Invalid competency id: the second part of the competency id must contain only numerical characters.")?,
    }


    // check skillset id 
    match question_id_parts[1].parse::<u32>() {
        Ok(_) => (),
        Err(_) => return Err("Invalid skillset id. Skillset id must be in u32.")?,
    }

    // check skill id
    match question_id_parts[2].parse::<u32>() {
        Ok(_) => (),
        Err(_) => return Err("Invalid skill id. Skill id must be in u32.")?,
    }

    Ok(())
}