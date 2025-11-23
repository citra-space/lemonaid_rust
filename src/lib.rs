mod entities;

// Re-export types for public API
pub use entities::telescope::Telescope;
pub use entities::groundstation::Groundstation;
pub use entities::task::{Task, TaskStatus, TaskUpdateRequest, CreateTaskRequest};
pub use entities::antenna::Antenna;
pub use entities::access::{SatelliteAccessToGroundstationRequest, HorizonAccess, FOVAccessRequest, FOVAccessResponse, SensorFrame};
pub use entities::rf_observation::{CreateRFCaptureRequest, RFCapture, RFCaptureSummary};

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
                client: reqwest::Client::new()
            }
        }
        CitraClient {
            base_url: "https://dev.api.citra.space/".to_string(),
            api_key: api_key.to_string(),
            client: reqwest::Client::new()
        }
    }

    pub async fn get_telescope(&self, telescope_id: &str) -> Result<Telescope, reqwest::Error> {
        let url = format!("{}telescopes/{}", self.base_url, telescope_id);
        let response = self.client
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
        let response = self.client
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
        let response = self.client
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
        self.client
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
        let response = self.client
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
        let response = self.client
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
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<entities::groundstation::GroundstationListResponse>()
            .await?;
        Ok(response.ground_stations)
    }

    pub async fn create_groundstation(&self, groundstation: &GroundstationCreateRequest) -> Result<Groundstation, reqwest::Error> {
        // API only implements a bulk create endpoint for groundstations, so we wrap the single groundstation in a vector
        let url = format!("{}ground-stations", self.base_url);
        let response = self.client
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
        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![groundstation_id])
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn update_groundstation(&self, groundstation_id: &str, groundstation: &GroundstationCreateRequest) -> Result<Groundstation, reqwest::Error> {
        // API only implements a bulk update endpoint for groundstations, so we wrap the single groundstation in a vector
        let url = format!("{}ground-stations/{}", self.base_url, groundstation_id);
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![groundstation])
            .send()
            .await?
            .json::<Vec<Groundstation>>()
            .await?;
        Ok(response.into_iter().next().unwrap())
    }

    pub async fn solve_access_for_groundstation(&self, access_request: &SatelliteAccessToGroundstationRequest) -> Result<Vec<HorizonAccess>, reqwest::Error> {
        let url = format!("{}access/window/satellites_to_ground_station", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(access_request)
            .send()
            .await?
            .json::<Vec<HorizonAccess>>()
            .await?;
        Ok(response)
    }

    pub async fn solve_fov_access(&self, fov_request: &FOVAccessRequest) -> Result<Vec<FOVAccessResponse>, reqwest::Error> {
        let url = format!("{}access/fov", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(fov_request)
            .send()
            .await?
            .json::<Vec<FOVAccessResponse>>()
            .await?;
        Ok(response)
    }

    pub async fn list_tasks_for_telescope(&self, telescope_id: &str) -> Result<Vec<Task>, reqwest::Error> {
        let url = format!("{}telescopes/{}/tasks", self.base_url, telescope_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Vec<Task>>()
            .await?;
        Ok(response.into_iter().collect())
    }

    pub async fn get_telescope_tasks_by_status(&self, telescope_id: &str, statuses: Vec<TaskStatus>) -> Result<Vec<Task>, reqwest::Error> {
        let status_params: Vec<String> = statuses.iter().map(|s| format!("statuses={:?}", s)).collect();
        let query_string = status_params.join("&");
        let url = format!("{}telescopes/{}/tasks?{}", self.base_url, telescope_id, query_string);
        let response = self.client
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
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(task)
            .send()
            .await?
            .json::<Task>()
            .await?;
        Ok(response)
    }

    pub async fn create_task(&self, task: &CreateTaskRequest) -> Result<Task, reqwest::Error> {
        let url = format!("{}tasks", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(task)
            .send()
            .await?
            .json::<Task>()
            .await?;
        Ok(response)
    }

    pub async fn list_antennas(&self) -> Result<Vec<Antenna>, reqwest::Error> {
        let url = format!("{}antennas", self.base_url);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Vec<Antenna>>()
            .await?;
        Ok(response)
    }

    pub async fn get_antenna(&self, antenna_id: &str) -> Result<Antenna, reqwest::Error> {
        let url = format!("{}antennas/{}", self.base_url, antenna_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Antenna>()
            .await?;
        Ok(response)
    }

    pub async fn create_antenna(&self, antenna: &Antenna) -> Result<Antenna, reqwest::Error> {
        // API only implements a bulk create endpoint for antennas, so we wrap the single antenna in a vector
        let url = format!("{}antennas", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![antenna])
            .send()
            .await?
            .json::<Vec<Antenna>>()
            .await?;
        Ok(response.into_iter().next().unwrap())
    }

    pub async fn delete_antenna(&self, antenna_id: &str) -> Result<(), reqwest::Error> {
        // API only implements a bulk delete endpoint, with a vector of IDs
        let url = format!("{}antennas", self.base_url);
        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![antenna_id])
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn update_antenna(&self, antenna: &Antenna) -> Result<Antenna, reqwest::Error> {
        // API only implements a bulk update endpoint for antennas, so we wrap the single antenna in a vector
        let url = format!("{}antennas", self.base_url);
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![antenna])
            .send()
            .await?
            .json::<Vec<Antenna>>()
            .await?;
        Ok(response.into_iter().next().unwrap())
    }

    pub async fn list_tasks_for_antenna(&self, antenna_id: &str) -> Result<Vec<Task>, reqwest::Error> {
        let url = format!("{}antennas/{}/tasks", self.base_url, antenna_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Vec<Task>>()
            .await?;
        Ok(response.into_iter().collect())
    }

    pub async fn get_antenna_tasks_by_status(&self, antenna_id: &str, statuses: Vec<TaskStatus>) -> Result<Vec<Task>, reqwest::Error> {
        let status_params: Vec<String> = statuses.iter().map(|s| format!("statuses={:?}", s)).collect();
        let query_string = status_params.join("&");
        let url = format!("{}antennas/{}/tasks?{}", self.base_url, antenna_id, query_string);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Vec<Task>>()
            .await?;
        Ok(response.into_iter().collect())
    }

    pub async fn create_rf_capture(&self, rf_capture_request: &CreateRFCaptureRequest) -> Result<RFCapture, reqwest::Error> {
        let url = format!("{}rf-captures", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(rf_capture_request)
            .send()
            .await?
            .json::<RFCapture>()
            .await?;
        Ok(response)
    }

    pub async fn get_rf_capture(&self, rf_capture_id: &str) -> Result<RFCapture, reqwest::Error> {
        let url = format!("{}rf-captures/{}", self.base_url, rf_capture_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<RFCapture>()
            .await?;
        Ok(response)
    }

    pub async fn list_rf_captures_for_antenna(&self, antenna_id: &str) -> Result<Vec<RFCaptureSummary>, reqwest::Error> {
        let url = format!("{}antennas/{}/rf-captures", self.base_url, antenna_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Vec<RFCaptureSummary>>()
            .await?;
        Ok(response)
    }

    pub async fn list_rf_captures_for_task(&self, task_id: &str) -> Result<Vec<RFCaptureSummary>, reqwest::Error> {
        let url = format!("{}tasks/{}/rf-captures", self.base_url, task_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<Vec<RFCaptureSummary>>()
            .await?;
        Ok(response)
    }
}

