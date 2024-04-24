use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

const CODE_TEMPLATE_PATH: &str =
    "/Users/rahul703/Desktop/Projects/web_template/src/code_template.rs";
const EXEC_MAIN_PATH: &str = "/Users/rahul703/Desktop/Projects/web_template/src/main.rs";
const API_SCHEMA_PATH: &str = "/Users/rahul703/Desktop/Projects/auto_gpt/schemas/api_schemas.json";

//Encourage ai function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // dbg!(ai_function_str);

    //Extent the string to encourage only printing the output
    let msg = format!(
        "FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function: {}.
    Print out what the function will return.",
        ai_function_str, func_input
    );

    dbg!("INSIDE extend_ai_function{}", &msg);

    //Return Message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

//Perform call to LLM gpt
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    //Extend AI function
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    //Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    //Get LLM response
    let llm_response = call_gpt(vec![extended_msg.clone()]).await;

    dbg!("INSIDE ai_task_request : {}", &llm_response);

    //Handle success and return Or try again
    match llm_response {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed to call openAi twice"),
    }
}

//Perform call to decoded response
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    dbg!("INSIDE ai_task_request_decoded : {}", &llm_response);

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde json");

    decoded_response
}

//check weather request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

//Get code template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code_template.rs")
}

//Get main code template
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read main.rs")
}

//Save new backend code
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file");
}

//Sace json api end point schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: String =
            "Build me a webserver for making stock price api requests.".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        dbg!(res);
    }
}
