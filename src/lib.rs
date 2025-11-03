mod entities;

// Re-export types for public API
pub use entities::telescope::Telescope;
pub use entities::groundstation::Groundstation;
pub use entities::task::Task;
pub use entities::task::TaskStatus;
pub use entities::task::TaskUpdateRequest;

pub struct CitraClient {
    base_url: String,
    api_key: String
}

impl CitraClient {
    pub fn new(api_key: &str) -> Self {
        CitraClient {
            base_url: "https://dev.api.citra.space/".to_string(),
            api_key: api_key.to_string()
        }
    }

    pub async fn get_telescope(&self, telescope_id: &str) -> Result<Telescope, reqwest::Error> {
        let url = format!("{}telescopes/{}", self.base_url, telescope_id);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Telescope>()
            .await?;
        Ok(response)
    }

    pub async fn list_telescopes(&self) -> Result<Vec<Telescope>, reqwest::Error> {
        let url = format!("{}telescopes", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Vec<Telescope>>()
            .await?;
        Ok(response)
    }

    pub async fn create_telescope(&self, telescope: &Telescope) -> Result<Telescope, reqwest::Error> {
        // API only implements a bulk create endpoint for telescopes, so we wrap the single telescope in a vector
        let url = format!("{}telescopes", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![telescope])
            .send()
            .await?
            .json::<Vec<Telescope>>()
            .await?;
        Ok(response.into_iter().next().unwrap())
    }

    pub async fn delete_telescope(&self, telescope_id: &str) -> Result<(), reqwest::Error> {
        // API only implements a bulk delete endpoint, with a vector of IDs
        let url = format!("{}telescopes", self.base_url);
        let client = reqwest::Client::new();
        client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![telescope_id])
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn update_telescope(&self, telescope: &Telescope) -> Result<Telescope, reqwest::Error> {
        // API only implements a bulk update endpoint for telescopes, so we wrap the single telescope in a vector
        let url = format!("{}telescopes", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![telescope])
            .send()
            .await?
            .json::<Vec<Telescope>>()
            .await?;
        Ok(response.into_iter().next().unwrap())
    }

    pub async fn get_groundstation(&self, groundstation_id: &str) -> Result<Groundstation, reqwest::Error> {
        let url = format!("{}ground-stations/{}", self.base_url, groundstation_id);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Groundstation>()
            .await?;
        Ok(response)
    }

    pub async fn list_groundstations(&self) -> Result<Vec<Groundstation>, reqwest::Error> {
        let url = format!("{}ground-stations", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<entities::groundstation::GroundstationListResponse>()
            .await?;
        Ok(response.ground_stations)
    }

    pub async fn create_groundstation(&self, groundstation: &Groundstation) -> Result<Groundstation, reqwest::Error> {
        // API only implements a bulk create endpoint for groundstations, so we wrap the single groundstation in a vector
        let url = format!("{}ground-stations", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![groundstation])
            .send()
            .await?
            .json::<Vec<Groundstation>>()
            .await?;
        Ok(response.into_iter().next().unwrap())
    }

    pub async fn delete_groundstation(&self, groundstation_id: &str) -> Result<(), reqwest::Error> {
        // API only implements a bulk delete endpoint, with a vector of IDs
        let url = format!("{}ground-stations", self.base_url);
        let client = reqwest::Client::new();
        client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![groundstation_id])
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn update_groundstation(&self, groundstation: &Groundstation) -> Result<Groundstation, reqwest::Error> {
        // API only implements a bulk update endpoint for groundstations, so we wrap the single groundstation in a vector
        let url = format!("{}ground-stations", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![groundstation])
            .send()
            .await?
            .json::<Vec<Groundstation>>()
            .await?;
        Ok(response.into_iter().next().unwrap())
    }

    pub async fn list_tasks_for_telescope(&self, telescope_id: &str) -> Result<Vec<Task>, reqwest::Error> {
        let url = format!("{}telescopes/{}/tasks", self.base_url, telescope_id);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Vec<Task>>()
            .await?;
        Ok(response.into_iter().collect())
    }

    pub async fn update_task(&self, task: &TaskUpdateRequest) -> Result<Task, reqwest::Error> {
        let url = format!("{}tasks/{}", self.base_url, task.id);
        let client = reqwest::Client::new();
        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(task)
            .send()
            .await?
            .json::<Task>()
            .await?;
        Ok(response)
    }
}

