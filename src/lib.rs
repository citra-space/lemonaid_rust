mod entities;
mod error;

// Re-export types for public API
pub use entities::access::{
    FOVAccessRequest, FOVAccessResponse, HorizonAccess, SatelliteAccessToGroundstationRequest,
    SensorFrame,
};
pub use entities::antenna::Antenna;
pub use entities::groundstation::Groundstation;
pub use entities::rf_observation::{CreateRFCaptureRequest, RFCapture, RFCaptureSummary};
pub use entities::task::{CreateTaskRequest, Task, TaskStatus, TaskUpdateRequest};
pub use entities::telescope::Telescope;
pub use error::LemonaidError;

use crate::entities::groundstation::GroundstationCreateRequest;

pub struct CitraClient {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl CitraClient {
    pub fn new(api_key: &str, dev: bool) -> Self {
        if !dev {
            return CitraClient {
                base_url: "https://api.citra.space/".to_string(),
                api_key: api_key.to_string(),
                client: reqwest::Client::new(),
            };
        }
        CitraClient {
            base_url: "https://dev.api.citra.space/".to_string(),
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Helper method to check response status and return appropriate error
    async fn check_response(
        &self,
        response: reqwest::Response,
    ) -> Result<reqwest::Response, LemonaidError> {
        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(LemonaidError::Api { status, message })
        }
    }

    pub async fn get_telescope(&self, telescope_id: &str) -> Result<Telescope, LemonaidError> {
        let url = format!("{}telescopes/{}", self.base_url, telescope_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let telescope = response.json::<Telescope>().await?;
        Ok(telescope)
    }

    pub async fn list_telescopes(&self) -> Result<Vec<Telescope>, LemonaidError> {
        let url = format!("{}telescopes", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let telescopes = response.json::<Vec<Telescope>>().await?;
        Ok(telescopes)
    }

    pub async fn create_telescope(
        &self,
        telescope: &Telescope,
    ) -> Result<Telescope, LemonaidError> {
        // API only implements a bulk create endpoint for telescopes, so we wrap the single telescope in a vector
        let url = format!("{}telescopes", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![telescope])
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let telescopes = response.json::<Vec<Telescope>>().await?;
        Ok(telescopes.into_iter().next().unwrap())
    }

    pub async fn delete_telescope(&self, telescope_id: &str) -> Result<(), LemonaidError> {
        // API only implements a bulk delete endpoint, with a vector of IDs
        let url = format!("{}telescopes", self.base_url);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![telescope_id])
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn update_telescope(
        &self,
        telescope: &Telescope,
    ) -> Result<Telescope, LemonaidError> {
        // API only implements a bulk update endpoint for telescopes, so we wrap the single telescope in a vector
        let url = format!("{}telescopes", self.base_url);
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![telescope])
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let telescopes = response.json::<Vec<Telescope>>().await?;
        Ok(telescopes.into_iter().next().unwrap())
    }

    pub async fn get_groundstation(
        &self,
        groundstation_id: &str,
    ) -> Result<Groundstation, LemonaidError> {
        let url = format!("{}ground-stations/{}", self.base_url, groundstation_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let groundstation = response.json::<Groundstation>().await?;
        Ok(groundstation)
    }

    pub async fn list_groundstations(&self) -> Result<Vec<Groundstation>, LemonaidError> {
        let url = format!("{}ground-stations", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let list_response = response
            .json::<entities::groundstation::GroundstationListResponse>()
            .await?;
        Ok(list_response.ground_stations)
    }

    pub async fn create_groundstation(
        &self,
        groundstation: &GroundstationCreateRequest,
    ) -> Result<Groundstation, LemonaidError> {
        // API only implements a bulk create endpoint for groundstations, so we wrap the single groundstation in a vector
        let url = format!("{}ground-stations", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![groundstation])
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let groundstations = response.json::<Vec<Groundstation>>().await?;
        Ok(groundstations.into_iter().next().unwrap())
    }

    pub async fn delete_groundstation(&self, groundstation_id: &str) -> Result<(), LemonaidError> {
        // API only implements a bulk delete endpoint, with a vector of IDs
        let url = format!("{}ground-stations", self.base_url);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![groundstation_id])
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn update_groundstation(
        &self,
        groundstation_id: &str,
        groundstation: &GroundstationCreateRequest,
    ) -> Result<Groundstation, LemonaidError> {
        // API only implements a bulk update endpoint for groundstations, so we wrap the single groundstation in a vector
        let url = format!("{}ground-stations/{}", self.base_url, groundstation_id);
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![groundstation])
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let groundstations = response.json::<Vec<Groundstation>>().await?;
        Ok(groundstations.into_iter().next().unwrap())
    }

    pub async fn solve_access_for_groundstation(
        &self,
        access_request: &SatelliteAccessToGroundstationRequest,
    ) -> Result<Vec<HorizonAccess>, LemonaidError> {
        let url = format!(
            "{}access/window/satellites_to_ground_station",
            self.base_url
        );
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(access_request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let accesses = response.json::<Vec<HorizonAccess>>().await?;
        Ok(accesses)
    }

    pub async fn solve_fov_access(
        &self,
        fov_request: &FOVAccessRequest,
    ) -> Result<Vec<FOVAccessResponse>, LemonaidError> {
        let url = format!("{}access/fov", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(fov_request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let fov_responses = response.json::<Vec<FOVAccessResponse>>().await?;
        Ok(fov_responses)
    }

    pub async fn list_tasks_for_telescope(
        &self,
        telescope_id: &str,
    ) -> Result<Vec<Task>, LemonaidError> {
        let url = format!("{}telescopes/{}/tasks", self.base_url, telescope_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let tasks = response.json::<Vec<Task>>().await?;
        Ok(tasks.into_iter().collect())
    }

    pub async fn get_telescope_tasks_by_status(
        &self,
        telescope_id: &str,
        statuses: Vec<TaskStatus>,
    ) -> Result<Vec<Task>, LemonaidError> {
        let status_params: Vec<String> = statuses
            .iter()
            .map(|s| format!("statuses={:?}", s))
            .collect();
        let query_string = status_params.join("&");
        let url = format!(
            "{}telescopes/{}/tasks?{}",
            self.base_url, telescope_id, query_string
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let tasks = response.json::<Vec<Task>>().await?;
        Ok(tasks.into_iter().collect())
    }

    pub async fn update_task(&self, task: &TaskUpdateRequest) -> Result<Task, LemonaidError> {
        let url = format!("{}tasks/{}", self.base_url, task.id);
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(task)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let updated_task = response.json::<Task>().await?;
        Ok(updated_task)
    }

    pub async fn create_task(&self, task: &CreateTaskRequest) -> Result<Task, LemonaidError> {
        let url = format!("{}tasks", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(task)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let created_task = response.json::<Task>().await?;
        Ok(created_task)
    }

    pub async fn list_antennas(&self) -> Result<Vec<Antenna>, LemonaidError> {
        let url = format!("{}antennas", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let antennas = response.json::<Vec<Antenna>>().await?;
        Ok(antennas)
    }

    pub async fn get_antenna(&self, antenna_id: &str) -> Result<Antenna, LemonaidError> {
        let url = format!("{}antennas/{}", self.base_url, antenna_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let antenna = response.json::<Antenna>().await?;
        Ok(antenna)
    }

    pub async fn create_antenna(&self, antenna: &Antenna) -> Result<Antenna, LemonaidError> {
        // API only implements a bulk create endpoint for antennas, so we wrap the single antenna in a vector
        let url = format!("{}antennas", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![antenna])
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let antennas = response.json::<Vec<Antenna>>().await?;
        Ok(antennas.into_iter().next().unwrap())
    }

    pub async fn delete_antenna(&self, antenna_id: &str) -> Result<(), LemonaidError> {
        // API only implements a bulk delete endpoint, with a vector of IDs
        let url = format!("{}antennas", self.base_url);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![antenna_id])
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn update_antenna(&self, antenna: &Antenna) -> Result<Antenna, LemonaidError> {
        // API only implements a bulk update endpoint for antennas, so we wrap the single antenna in a vector
        let url = format!("{}antennas", self.base_url);
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![antenna])
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let antennas = response.json::<Vec<Antenna>>().await?;
        Ok(antennas.into_iter().next().unwrap())
    }

    pub async fn list_tasks_for_antenna(
        &self,
        antenna_id: &str,
    ) -> Result<Vec<Task>, LemonaidError> {
        let url = format!("{}antennas/{}/tasks", self.base_url, antenna_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let tasks = response.json::<Vec<Task>>().await?;
        Ok(tasks.into_iter().collect())
    }

    pub async fn get_antenna_tasks_by_status(
        &self,
        antenna_id: &str,
        statuses: Vec<TaskStatus>,
    ) -> Result<Vec<Task>, LemonaidError> {
        let status_params: Vec<String> = statuses
            .iter()
            .map(|s| format!("statuses={:?}", s))
            .collect();
        let query_string = status_params.join("&");
        let url = format!(
            "{}antennas/{}/tasks?{}",
            self.base_url, antenna_id, query_string
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let tasks = response.json::<Vec<Task>>().await?;
        Ok(tasks.into_iter().collect())
    }

    pub async fn create_rf_capture(
        &self,
        rf_capture_request: &CreateRFCaptureRequest,
    ) -> Result<RFCapture, LemonaidError> {
        let url = format!("{}rf-captures", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(rf_capture_request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let rf_capture = response.json::<RFCapture>().await?;
        Ok(rf_capture)
    }

    pub async fn get_rf_capture(&self, rf_capture_id: &str) -> Result<RFCapture, LemonaidError> {
        let url = format!("{}rf-captures/{}", self.base_url, rf_capture_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let rf_capture = response.json::<RFCapture>().await?;
        Ok(rf_capture)
    }

    pub async fn list_rf_captures_for_antenna(
        &self,
        antenna_id: &str,
    ) -> Result<Vec<RFCaptureSummary>, LemonaidError> {
        let url = format!("{}antennas/{}/rf-captures", self.base_url, antenna_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let rf_captures = response.json::<Vec<RFCaptureSummary>>().await?;
        Ok(rf_captures)
    }

    pub async fn list_rf_captures_for_task(
        &self,
        task_id: &str,
    ) -> Result<Vec<RFCaptureSummary>, LemonaidError> {
        let url = format!("{}tasks/{}/rf-captures", self.base_url, task_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let rf_captures = response.json::<Vec<RFCaptureSummary>>().await?;
        Ok(rf_captures)
    }
}
