mod entities;
mod error;

// Re-export types for public API
pub use entities::access::{
    EphemerisPoint, EphemerisRequest, EphemerisResponse, FOVAccessRequest, FOVAccessResponse,
    GeoAccessQuery, GeoAccessResult, GroundStationAccessToSatelliteRequest, HorizonAccess,
    LocationAccess, LocationWaypoint, SatelliteAccessToGroundstationRequest,
    SatellitesToLocationRequest, SensorFrame, TrackingParameters,
};
pub use entities::account::{
    AddGroupMemberRequest, GroupMember, UpdatePreferencesRequest, UserAccount, UserPreferences,
};
pub use entities::alert_subscription::{
    AlertSubscription, AlertType, CreateAlertSubscriptionRequest, TargetType,
    UpdateAlertSubscriptionRequest,
};
pub use entities::antenna::Antenna;
pub use entities::auth::{
    CreatePersonalAccessTokenRequest, CreatePersonalAccessTokenResponse, PersonalAccessToken,
    PersonalAccessTokenListResponse,
};
pub use entities::collection_request::{
    CollectionRequest, CollectionRequestType, CreateCollectionRequestRequest,
};
pub use entities::elset::{
    CreateElsetRequest, Elset, ElsetCount, ElsetHistoryQuery, GeoScatterPoint, LeoScatterPoint,
};
pub use entities::filter::{Filter, FilterExpandRequest, FilterExpandResponse};
pub use entities::groundstation::{Groundstation, GroundstationCreateRequest};
pub use entities::image::{
    ImageData, ImageDataRequest, ImageListQuery, ImageStatus, ImageUploadRequest,
    ImageUploadResponse,
};
pub use entities::maneuver::{
    CreateManeuverRequest, Maneuver, ManeuverListQuery, UpdateManeuverRequest,
};
pub use entities::observation::{
    CreateOpticalObservationRequest, ObservationCount, ObservationQuery, OpticalObservation,
};
pub use entities::orbit_determination::{ODObservation, ODRequest, ODResult};
pub use entities::rf_observation::{CreateRFCaptureRequest, RFCapture, RFCaptureSummary};
pub use entities::satellite::{
    CloseApproach, CountryCount, GroundTrackPoint, ObservationBounds, ObservationResidual,
    OrbitalElements, RelativeState, ResidualResult, ResidualsRequest, Satellite, SatelliteListQuery,
    SatelliteOverview, SatellitePageResponse, SatellitePaginatedResponse,
};
pub use entities::satellite_group::{
    CreateSatelliteGroupRequest, SatelliteGroup, SatelliteGroupMembersRequest,
    UpdateSatelliteGroupRequest,
};
pub use entities::task::{CreateTaskRequest, Task, TaskStatus, TaskUpdateRequest};
pub use entities::telescope::Telescope;
pub use entities::weather::{
    CurrentWeather, MinutelyForecast, WeatherCondition, WeatherQuery, WeatherResponse,
};
pub use error::LemonaidError;

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

    // ==================== TELESCOPE ENDPOINTS ====================

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

    pub async fn list_my_telescopes(&self) -> Result<Vec<Telescope>, LemonaidError> {
        let url = format!("{}my/telescopes", self.base_url);
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
        Ok(tasks)
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
        Ok(tasks)
    }

    pub async fn list_images_for_telescope(
        &self,
        telescope_id: &str,
        query: Option<&ImageListQuery>,
    ) -> Result<Vec<ImageStatus>, LemonaidError> {
        let mut url = format!("{}telescopes/{}/images", self.base_url, telescope_id);
        if let Some(q) = query {
            let mut params = Vec::new();
            if let Some(offset) = q.offset {
                params.push(format!("offset={}", offset));
            }
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(ref status) = q.status {
                params.push(format!("status={}", status));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let images = response.json::<Vec<ImageStatus>>().await?;
        Ok(images)
    }

    // ==================== GROUND STATION ENDPOINTS ====================

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

    pub async fn list_my_groundstations(&self) -> Result<Vec<Groundstation>, LemonaidError> {
        let url = format!("{}my/ground-stations", self.base_url);
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
        let url = format!("{}ground-stations", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(groundstation)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let created = response.json::<Groundstation>().await?;
        Ok(created)
    }

    pub async fn delete_groundstation(&self, groundstation_id: &str) -> Result<(), LemonaidError> {
        let url = format!("{}ground-stations/{}", self.base_url, groundstation_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
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
        let url = format!("{}ground-stations/{}", self.base_url, groundstation_id);
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(groundstation)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let groundstation = response.json::<Groundstation>().await?;
        Ok(groundstation)
    }

    pub async fn list_telescopes_for_groundstation(
        &self,
        groundstation_id: &str,
    ) -> Result<Vec<Telescope>, LemonaidError> {
        let url = format!(
            "{}ground-stations/{}/telescopes",
            self.base_url, groundstation_id
        );
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

    pub async fn list_antennas_for_groundstation(
        &self,
        groundstation_id: &str,
    ) -> Result<Vec<Antenna>, LemonaidError> {
        let url = format!(
            "{}ground-stations/{}/antennas",
            self.base_url, groundstation_id
        );
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

    // ==================== ANTENNA ENDPOINTS ====================

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

    pub async fn list_my_antennas(&self) -> Result<Vec<Antenna>, LemonaidError> {
        let url = format!("{}my/antennas", self.base_url);
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
        Ok(tasks)
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
        Ok(tasks)
    }

    // ==================== TASK ENDPOINTS ====================

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

    pub async fn list_images_for_task(
        &self,
        task_id: &str,
    ) -> Result<Vec<ImageStatus>, LemonaidError> {
        let url = format!("{}tasks/{}/images", self.base_url, task_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let images = response.json::<Vec<ImageStatus>>().await?;
        Ok(images)
    }

    // ==================== ACCESS ENDPOINTS ====================

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

    pub async fn solve_access_groundstations_to_satellite(
        &self,
        access_request: &GroundStationAccessToSatelliteRequest,
    ) -> Result<Vec<HorizonAccess>, LemonaidError> {
        let url = format!(
            "{}access/window/ground_stations_to_satellite",
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

    pub async fn solve_access_satellites_to_location(
        &self,
        access_request: &SatellitesToLocationRequest,
    ) -> Result<Vec<LocationAccess>, LemonaidError> {
        let url = format!("{}access/window/satellitesToLocation", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(access_request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let accesses = response.json::<Vec<LocationAccess>>().await?;
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

    pub async fn get_geo_access(
        &self,
        query: &GeoAccessQuery,
    ) -> Result<Vec<GeoAccessResult>, LemonaidError> {
        let mut params = Vec::new();
        if let Some(min_lon) = query.min_longitude_deg {
            params.push(format!("minLongitude={}", min_lon));
        }
        if let Some(max_lon) = query.max_longitude_deg {
            params.push(format!("maxLongitude={}", max_lon));
        }
        if let Some(min_sma) = query.min_semi_major_axis_km {
            params.push(format!("minSemiMajorAxis={}", min_sma));
        }
        if let Some(max_sma) = query.max_semi_major_axis_km {
            params.push(format!("maxSemiMajorAxis={}", max_sma));
        }
        if let Some(max_inc) = query.max_inclination_deg {
            params.push(format!("maxInclination={}", max_inc));
        }
        let url = if params.is_empty() {
            format!("{}access/geo", self.base_url)
        } else {
            format!("{}access/geo?{}", self.base_url, params.join("&"))
        };
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let results = response.json::<Vec<GeoAccessResult>>().await?;
        Ok(results)
    }

    pub async fn generate_ephemeris(
        &self,
        request: &EphemerisRequest,
    ) -> Result<EphemerisResponse, LemonaidError> {
        let url = format!("{}access/ephemeris", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let ephemeris = response.json::<EphemerisResponse>().await?;
        Ok(ephemeris)
    }

    // ==================== RF CAPTURE ENDPOINTS ====================

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

    pub async fn list_rf_captures_for_satellite(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<RFCaptureSummary>, LemonaidError> {
        let url = format!("{}satellites/{}/rf-captures", self.base_url, satellite_id);
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

    // ==================== SATELLITE ENDPOINTS ====================

    pub async fn list_satellites(
        &self,
        query: Option<&SatelliteListQuery>,
    ) -> Result<SatellitePageResponse, LemonaidError> {
        let mut url = format!("{}satellites", self.base_url);
        if let Some(q) = query {
            let mut params = Vec::new();
            if let Some(ref ids) = q.ids {
                for id in ids {
                    params.push(format!("ids={}", id));
                }
            }
            if let Some(ref search) = q.search {
                params.push(format!("search={}", search));
            }
            if let Some(ref country) = q.country {
                params.push(format!("country={}", country));
            }
            if let Some(ref object_type) = q.object_type {
                params.push(format!("objectType={}", object_type));
            }
            if let Some(include_decayed) = q.include_decayed {
                params.push(format!("includeDecayed={}", include_decayed));
            }
            if let Some(ref sort_by) = q.sort_by {
                params.push(format!("sortBy={}", sort_by));
            }
            if let Some(ref sort_order) = q.sort_order {
                params.push(format!("sortOrder={}", sort_order));
            }
            if let Some(offset) = q.offset {
                params.push(format!("offset={}", offset));
            }
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let page_response = response.json::<SatellitePageResponse>().await?;
        Ok(page_response)
    }

    pub async fn get_satellite(&self, satellite_id: &str) -> Result<Satellite, LemonaidError> {
        let url = format!("{}satellites/{}", self.base_url, satellite_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let satellite = response.json::<Satellite>().await?;
        Ok(satellite)
    }

    pub async fn get_satellites_overview(&self) -> Result<SatelliteOverview, LemonaidError> {
        let url = format!("{}satellites/overview", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let overview = response.json::<SatelliteOverview>().await?;
        Ok(overview)
    }

    pub async fn get_satellite_countries(&self) -> Result<Vec<CountryCount>, LemonaidError> {
        let url = format!("{}satellites/countries", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let countries = response.json::<Vec<CountryCount>>().await?;
        Ok(countries)
    }

    pub async fn get_satellites_page(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<SatellitePaginatedResponse, LemonaidError> {
        let url = format!(
            "{}satellites/page?offset={}&limit={}",
            self.base_url, offset, limit
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let page = response.json::<SatellitePaginatedResponse>().await?;
        Ok(page)
    }

    pub async fn list_tasks_for_satellite(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<Task>, LemonaidError> {
        let url = format!("{}satellites/{}/tasks", self.base_url, satellite_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let tasks = response.json::<Vec<Task>>().await?;
        Ok(tasks)
    }

    pub async fn calculate_residuals(
        &self,
        satellite_id: &str,
        request: &ResidualsRequest,
    ) -> Result<Vec<ResidualResult>, LemonaidError> {
        let url = format!("{}satellites/{}/residuals", self.base_url, satellite_id);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let residuals = response.json::<Vec<ResidualResult>>().await?;
        Ok(residuals)
    }

    pub async fn get_close_approaches(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<CloseApproach>, LemonaidError> {
        let url = format!("{}satellites/{}/close-approaches", self.base_url, satellite_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let approaches = response.json::<Vec<CloseApproach>>().await?;
        Ok(approaches)
    }

    pub async fn get_relative_state(
        &self,
        satellite_id: &str,
        other_satellite_id: &str,
    ) -> Result<Vec<RelativeState>, LemonaidError> {
        let url = format!(
            "{}satellites/{}/relative/{}",
            self.base_url, satellite_id, other_satellite_id
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let states = response.json::<Vec<RelativeState>>().await?;
        Ok(states)
    }

    pub async fn get_satellite_ground_track(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<GroundTrackPoint>, LemonaidError> {
        let url = format!("{}satellites/{}/ground-track", self.base_url, satellite_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let track = response.json::<Vec<GroundTrackPoint>>().await?;
        Ok(track)
    }

    pub async fn get_satellite_observation_bounds(
        &self,
        satellite_id: &str,
    ) -> Result<ObservationBounds, LemonaidError> {
        let url = format!(
            "{}satellites/{}/observation-bounds",
            self.base_url, satellite_id
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let bounds = response.json::<ObservationBounds>().await?;
        Ok(bounds)
    }

    pub async fn get_satellite_groups_for_satellite(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<SatelliteGroup>, LemonaidError> {
        let url = format!("{}satellites/{}/groups", self.base_url, satellite_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let groups = response.json::<Vec<SatelliteGroup>>().await?;
        Ok(groups)
    }

    pub async fn list_images_for_satellite(
        &self,
        satellite_id: &str,
        query: Option<&ImageListQuery>,
    ) -> Result<Vec<ImageStatus>, LemonaidError> {
        let mut url = format!("{}satellites/{}/images", self.base_url, satellite_id);
        if let Some(q) = query {
            let mut params = Vec::new();
            if let Some(offset) = q.offset {
                params.push(format!("offset={}", offset));
            }
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let images = response.json::<Vec<ImageStatus>>().await?;
        Ok(images)
    }

    pub async fn list_collection_requests_for_satellite(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<CollectionRequest>, LemonaidError> {
        let url = format!(
            "{}satellites/{}/collection-requests",
            self.base_url, satellite_id
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let requests = response.json::<Vec<CollectionRequest>>().await?;
        Ok(requests)
    }

    pub async fn get_satellite_orbital_elements(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<OrbitalElements>, LemonaidError> {
        let url = format!(
            "{}satellites/{}/orbital-elements",
            self.base_url, satellite_id
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let elements = response.json::<Vec<OrbitalElements>>().await?;
        Ok(elements)
    }

    // ==================== ELSET ENDPOINTS ====================

    pub async fn get_satellite_elsets(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<Elset>, LemonaidError> {
        let url = format!("{}satellites/{}/elsets", self.base_url, satellite_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let elsets = response.json::<Vec<Elset>>().await?;
        Ok(elsets)
    }

    pub async fn create_elset(
        &self,
        satellite_id: &str,
        request: &CreateElsetRequest,
    ) -> Result<Elset, LemonaidError> {
        let url = format!("{}satellites/{}/elsets", self.base_url, satellite_id);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let elset = response.json::<Elset>().await?;
        Ok(elset)
    }

    pub async fn get_satellite_elset_history(
        &self,
        satellite_id: &str,
        query: Option<&ElsetHistoryQuery>,
    ) -> Result<Vec<Elset>, LemonaidError> {
        let mut url = format!("{}satellites/{}/elset-history", self.base_url, satellite_id);
        if let Some(q) = query {
            let mut params = Vec::new();
            if let Some(ref start) = q.start {
                params.push(format!("start={}", start.to_rfc3339()));
            }
            if let Some(ref end) = q.end {
                params.push(format!("end={}", end.to_rfc3339()));
            }
            if let Some(ref source) = q.source {
                params.push(format!("source={}", source));
            }
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let elsets = response.json::<Vec<Elset>>().await?;
        Ok(elsets)
    }

    pub async fn get_latest_elsets(&self, days: Option<i32>) -> Result<Vec<Elset>, LemonaidError> {
        let url = match days {
            Some(d) => format!("{}elsets/latest?days={}", self.base_url, d),
            None => format!("{}elsets/latest", self.base_url),
        };
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let elsets = response.json::<Vec<Elset>>().await?;
        Ok(elsets)
    }

    pub async fn get_elset_counts(&self) -> Result<Vec<ElsetCount>, LemonaidError> {
        let url = format!("{}elsets/counts", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let counts = response.json::<Vec<ElsetCount>>().await?;
        Ok(counts)
    }

    pub async fn get_near_geo_scatter(&self) -> Result<Vec<GeoScatterPoint>, LemonaidError> {
        let url = format!("{}elsets/near-geo-scatter", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let scatter = response.json::<Vec<GeoScatterPoint>>().await?;
        Ok(scatter)
    }

    pub async fn get_leo_scatter(
        &self,
        min_sma_km: Option<f64>,
        max_sma_km: Option<f64>,
    ) -> Result<Vec<LeoScatterPoint>, LemonaidError> {
        let mut params = Vec::new();
        if let Some(min) = min_sma_km {
            params.push(format!("minSma={}", min));
        }
        if let Some(max) = max_sma_km {
            params.push(format!("maxSma={}", max));
        }
        let url = if params.is_empty() {
            format!("{}elsets/leo-scatter", self.base_url)
        } else {
            format!("{}elsets/leo-scatter?{}", self.base_url, params.join("&"))
        };
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let scatter = response.json::<Vec<LeoScatterPoint>>().await?;
        Ok(scatter)
    }

    pub async fn reject_elset(&self, elset_id: &str) -> Result<(), LemonaidError> {
        let url = format!("{}elsets/{}/reject", self.base_url, elset_id);
        let response = self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    // ==================== SATELLITE GROUP ENDPOINTS ====================

    pub async fn list_satellite_groups(&self) -> Result<Vec<SatelliteGroup>, LemonaidError> {
        let url = format!("{}satellite-groups", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let groups = response.json::<Vec<SatelliteGroup>>().await?;
        Ok(groups)
    }

    pub async fn list_my_satellite_groups(&self) -> Result<Vec<SatelliteGroup>, LemonaidError> {
        let url = format!("{}my/satellite-groups", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let groups = response.json::<Vec<SatelliteGroup>>().await?;
        Ok(groups)
    }

    pub async fn list_favorite_satellite_groups(
        &self,
    ) -> Result<Vec<SatelliteGroup>, LemonaidError> {
        let url = format!("{}my/satellite-groups/favorites", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let groups = response.json::<Vec<SatelliteGroup>>().await?;
        Ok(groups)
    }

    pub async fn get_satellite_group(
        &self,
        group_id: &str,
    ) -> Result<SatelliteGroup, LemonaidError> {
        let url = format!("{}satellite-groups/{}", self.base_url, group_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let group = response.json::<SatelliteGroup>().await?;
        Ok(group)
    }

    pub async fn create_satellite_group(
        &self,
        request: &CreateSatelliteGroupRequest,
    ) -> Result<String, LemonaidError> {
        let url = format!("{}satellite-groups", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![request])
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let ids = response.json::<Vec<String>>().await?;
        Ok(ids.into_iter().next().unwrap())
    }

    pub async fn update_satellite_group(
        &self,
        request: &UpdateSatelliteGroupRequest,
    ) -> Result<String, LemonaidError> {
        let url = format!("{}satellite-groups", self.base_url);
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![request])
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let ids = response.json::<Vec<String>>().await?;
        Ok(ids.into_iter().next().unwrap())
    }

    pub async fn delete_satellite_group(&self, group_id: &str) -> Result<(), LemonaidError> {
        let url = format!("{}satellite-groups/{}", self.base_url, group_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn get_satellites_in_group(
        &self,
        group_id: &str,
    ) -> Result<Vec<Satellite>, LemonaidError> {
        let url = format!("{}satellite-groups/{}/satellites", self.base_url, group_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let satellites = response.json::<Vec<Satellite>>().await?;
        Ok(satellites)
    }

    pub async fn add_satellites_to_group(
        &self,
        group_id: &str,
        request: &SatelliteGroupMembersRequest,
    ) -> Result<(), LemonaidError> {
        let url = format!("{}satellite-groups/{}/satellites", self.base_url, group_id);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn remove_satellites_from_group(
        &self,
        group_id: &str,
        request: &SatelliteGroupMembersRequest,
    ) -> Result<(), LemonaidError> {
        let url = format!("{}satellite-groups/{}/satellites", self.base_url, group_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn favorite_satellite_group(&self, group_id: &str) -> Result<(), LemonaidError> {
        let url = format!("{}satellite-groups/{}/favorite", self.base_url, group_id);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn unfavorite_satellite_group(&self, group_id: &str) -> Result<(), LemonaidError> {
        let url = format!("{}satellite-groups/{}/favorite", self.base_url, group_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    // ==================== WEATHER ENDPOINT ====================

    pub async fn get_weather(&self, query: &WeatherQuery) -> Result<WeatherResponse, LemonaidError> {
        let mut params = vec![
            format!("lat={}", query.latitude),
            format!("lon={}", query.longitude),
        ];
        if let Some(ref units) = query.units {
            params.push(format!("units={}", units));
        }
        let url = format!("{}weather?{}", self.base_url, params.join("&"));
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let weather = response.json::<WeatherResponse>().await?;
        Ok(weather)
    }

    // ==================== ACCOUNT ENDPOINTS ====================

    pub async fn get_my_account(&self) -> Result<UserAccount, LemonaidError> {
        let url = format!("{}my/account", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let account = response.json::<UserAccount>().await?;
        Ok(account)
    }

    pub async fn update_my_preferences(
        &self,
        request: &UpdatePreferencesRequest,
    ) -> Result<UserPreferences, LemonaidError> {
        let url = format!("{}my/preferences", self.base_url);
        let response = self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let preferences = response.json::<UserPreferences>().await?;
        Ok(preferences)
    }

    pub async fn list_group_members(&self) -> Result<Vec<GroupMember>, LemonaidError> {
        let url = format!("{}my/group/members", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let members = response.json::<Vec<GroupMember>>().await?;
        Ok(members)
    }

    pub async fn add_group_member(
        &self,
        request: &AddGroupMemberRequest,
    ) -> Result<GroupMember, LemonaidError> {
        let url = format!("{}my/group/members", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let member = response.json::<GroupMember>().await?;
        Ok(member)
    }

    pub async fn remove_group_member(&self, user_id: &str) -> Result<(), LemonaidError> {
        let url = format!("{}my/group/members/{}", self.base_url, user_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    // ==================== IMAGE ENDPOINTS ====================

    pub async fn initiate_image_upload(
        &self,
        request: &ImageUploadRequest,
    ) -> Result<ImageUploadResponse, LemonaidError> {
        let url = format!("{}my/images", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let upload = response.json::<ImageUploadResponse>().await?;
        Ok(upload)
    }

    pub async fn list_my_images(
        &self,
        query: Option<&ImageListQuery>,
    ) -> Result<Vec<ImageStatus>, LemonaidError> {
        let mut url = format!("{}my/images", self.base_url);
        if let Some(q) = query {
            let mut params = Vec::new();
            if let Some(offset) = q.offset {
                params.push(format!("offset={}", offset));
            }
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(ref status) = q.status {
                params.push(format!("status={}", status));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let images = response.json::<Vec<ImageStatus>>().await?;
        Ok(images)
    }

    pub async fn get_image_status(&self, upload_id: &str) -> Result<ImageStatus, LemonaidError> {
        let url = format!("{}my/images/{}", self.base_url, upload_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let status = response.json::<ImageStatus>().await?;
        Ok(status)
    }

    pub async fn delete_image(&self, upload_id: &str) -> Result<(), LemonaidError> {
        let url = format!("{}my/images/{}", self.base_url, upload_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn get_image_data(
        &self,
        upload_id: &str,
        request: Option<&ImageDataRequest>,
    ) -> Result<ImageData, LemonaidError> {
        let mut url = format!("{}images/{}/data", self.base_url, upload_id);
        if let Some(req) = request {
            let mut params = Vec::new();
            if let Some(binning) = req.binning {
                params.push(format!("binning={}", binning));
            }
            if let Some(ref contrast) = req.contrast {
                params.push(format!("contrast={}", contrast));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let data = response.json::<ImageData>().await?;
        Ok(data)
    }

    // ==================== OBSERVATION ENDPOINTS ====================

    pub async fn get_optical_observation_counts(
        &self,
    ) -> Result<Vec<ObservationCount>, LemonaidError> {
        let url = format!("{}observations/optical/counts", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let counts = response.json::<Vec<ObservationCount>>().await?;
        Ok(counts)
    }

    pub async fn upload_optical_observations(
        &self,
        observations: &[CreateOpticalObservationRequest],
    ) -> Result<Vec<OpticalObservation>, LemonaidError> {
        let url = format!("{}observations/optical", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(observations)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let created = response.json::<Vec<OpticalObservation>>().await?;
        Ok(created)
    }

    pub async fn query_optical_observations(
        &self,
        query: &ObservationQuery,
    ) -> Result<Vec<OpticalObservation>, LemonaidError> {
        let mut params = Vec::new();
        if let Some(ref satellite_id) = query.satellite_id {
            params.push(format!("satelliteId={}", satellite_id));
        }
        if let Some(ref telescope_id) = query.telescope_id {
            params.push(format!("telescopeId={}", telescope_id));
        }
        if let Some(ref start) = query.start {
            params.push(format!("start={}", start.to_rfc3339()));
        }
        if let Some(ref end) = query.end {
            params.push(format!("end={}", end.to_rfc3339()));
        }
        if let Some(offset) = query.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(limit) = query.limit {
            params.push(format!("limit={}", limit));
        }
        let url = if params.is_empty() {
            format!("{}observations/optical", self.base_url)
        } else {
            format!("{}observations/optical?{}", self.base_url, params.join("&"))
        };
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let observations = response.json::<Vec<OpticalObservation>>().await?;
        Ok(observations)
    }

    // ==================== COLLECTION REQUEST ENDPOINTS ====================

    pub async fn create_collection_request(
        &self,
        request: &CreateCollectionRequestRequest,
    ) -> Result<CollectionRequest, LemonaidError> {
        let url = format!("{}collection-requests", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let collection_request = response.json::<CollectionRequest>().await?;
        Ok(collection_request)
    }

    // ==================== FILTER ENDPOINTS ====================

    pub async fn list_filters(&self) -> Result<Vec<Filter>, LemonaidError> {
        let url = format!("{}filters/list", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let filters = response.json::<Vec<Filter>>().await?;
        Ok(filters)
    }

    pub async fn expand_filters(
        &self,
        request: &FilterExpandRequest,
    ) -> Result<FilterExpandResponse, LemonaidError> {
        let url = format!("{}filters/expand", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let expanded = response.json::<FilterExpandResponse>().await?;
        Ok(expanded)
    }

    // ==================== MANEUVER ENDPOINTS ====================

    pub async fn list_maneuvers(
        &self,
        query: Option<&ManeuverListQuery>,
    ) -> Result<Vec<Maneuver>, LemonaidError> {
        let mut url = format!("{}maneuvers", self.base_url);
        if let Some(q) = query {
            let mut params = Vec::new();
            if let Some(ref satellite_id) = q.satellite_id {
                params.push(format!("satelliteId={}", satellite_id));
            }
            if let Some(ref status) = q.status {
                params.push(format!("status={}", status));
            }
            if let Some(ref start) = q.start {
                params.push(format!("start={}", start.to_rfc3339()));
            }
            if let Some(ref end) = q.end {
                params.push(format!("end={}", end.to_rfc3339()));
            }
            if let Some(offset) = q.offset {
                params.push(format!("offset={}", offset));
            }
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let maneuvers = response.json::<Vec<Maneuver>>().await?;
        Ok(maneuvers)
    }

    pub async fn get_maneuver(&self, maneuver_id: &str) -> Result<Maneuver, LemonaidError> {
        let url = format!("{}maneuvers/{}", self.base_url, maneuver_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let maneuver = response.json::<Maneuver>().await?;
        Ok(maneuver)
    }

    pub async fn create_maneuver(
        &self,
        request: &CreateManeuverRequest,
    ) -> Result<Maneuver, LemonaidError> {
        let url = format!("{}maneuvers", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let maneuver = response.json::<Maneuver>().await?;
        Ok(maneuver)
    }

    pub async fn update_maneuver(
        &self,
        maneuver_id: &str,
        request: &UpdateManeuverRequest,
    ) -> Result<Maneuver, LemonaidError> {
        let url = format!("{}maneuvers/{}", self.base_url, maneuver_id);
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let maneuver = response.json::<Maneuver>().await?;
        Ok(maneuver)
    }

    pub async fn list_maneuvers_for_satellite(
        &self,
        satellite_id: &str,
    ) -> Result<Vec<Maneuver>, LemonaidError> {
        let url = format!("{}satellites/{}/maneuvers", self.base_url, satellite_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let maneuvers = response.json::<Vec<Maneuver>>().await?;
        Ok(maneuvers)
    }

    // ==================== ALERT SUBSCRIPTION ENDPOINTS ====================

    pub async fn list_alert_subscriptions(
        &self,
    ) -> Result<Vec<AlertSubscription>, LemonaidError> {
        let url = format!("{}my/alert-subscriptions", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let subscriptions = response.json::<Vec<AlertSubscription>>().await?;
        Ok(subscriptions)
    }

    pub async fn get_alert_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<AlertSubscription, LemonaidError> {
        let url = format!(
            "{}my/alert-subscriptions/{}",
            self.base_url, subscription_id
        );
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let subscription = response.json::<AlertSubscription>().await?;
        Ok(subscription)
    }

    pub async fn create_alert_subscription(
        &self,
        request: &CreateAlertSubscriptionRequest,
    ) -> Result<AlertSubscription, LemonaidError> {
        let url = format!("{}my/alert-subscriptions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let subscription = response.json::<AlertSubscription>().await?;
        Ok(subscription)
    }

    pub async fn update_alert_subscription(
        &self,
        subscription_id: &str,
        request: &UpdateAlertSubscriptionRequest,
    ) -> Result<AlertSubscription, LemonaidError> {
        let url = format!(
            "{}my/alert-subscriptions/{}",
            self.base_url, subscription_id
        );
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let subscription = response.json::<AlertSubscription>().await?;
        Ok(subscription)
    }

    pub async fn delete_alert_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<(), LemonaidError> {
        let url = format!(
            "{}my/alert-subscriptions/{}",
            self.base_url, subscription_id
        );
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    // ==================== AUTH ENDPOINTS ====================

    pub async fn list_personal_access_tokens(
        &self,
    ) -> Result<Vec<PersonalAccessToken>, LemonaidError> {
        let url = format!("{}auth/personal-access-tokens", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let wrapper = response.json::<PersonalAccessTokenListResponse>().await?;
        Ok(wrapper.tokens)
    }

    pub async fn delete_personal_access_token(&self, token_id: &str) -> Result<(), LemonaidError> {
        let url = format!("{}auth/personal-access-tokens/{}", self.base_url, token_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    // ==================== ORBIT DETERMINATION ENDPOINTS ====================

    pub async fn solve_orbit_determination(
        &self,
        request: &ODRequest,
    ) -> Result<ODResult, LemonaidError> {
        let url = format!("{}od/", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let result = response.json::<ODResult>().await?;
        Ok(result)
    }
}
