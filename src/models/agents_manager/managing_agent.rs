use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helpers::general::{ai_task_request, extend_ai_function};
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::general::llm::Message;

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>, //vector of agents that have a particular trait
}

impl ManagingAgent {
    // Create new instance of managing agent
    pub async fn new(user_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position = "Project manager".to_string();

        // Define attributes
        let attributes: BasicAgent = BasicAgent {
            objective: "Manage agents who are building a website for an end user".to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        // Extract Project Description
        let project_description: String = ai_task_request(
            user_req,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        // Initialize agents
        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        // Initialze Factsheet
        let factsheet: FactSheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        // Return Self
        Ok(Self {
            attributes,
            factsheet,
            agents,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        //TODO add backend agent
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_res = agent.execute(&mut self.factsheet).await;

            let agent_info = agent.get_attributes_from_agent();
            dbg!(agent_info);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_managing_agent() {
        let user_request = "Need a full stack app that fetched and tracks my fitness progress.
     Needs to include timezone";

        let mut managing_agent = ManagingAgent::new(user_request.to_string())
            .await
            .expect("Error creating Managing agent");

        managing_agent.execute_project().await;

        dbg!(managing_agent.factsheet);
    }
}
