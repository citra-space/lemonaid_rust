use chrono::Utc;
use lemonaid::*;
use std::env;

fn get_client() -> CitraClient {
    let api_key = env::var("CITRA_PAT").expect("CITRA_PAT environment variable must be set");
    CitraClient::new(&api_key, true) // Use dev environment
}

// ==================== ACCOUNT TESTS ====================

#[tokio::test]
async fn test_get_my_account() {
    let client = get_client();
    let result = client.get_my_account().await;
    assert!(result.is_ok(), "Failed to get account: {:?}", result.err());
    let account = result.unwrap();
    assert!(!account.id.is_empty());
}

// ==================== SATELLITE TESTS ====================

#[tokio::test]
async fn test_list_satellites() {
    let client = get_client();
    let result = client.list_satellites(None).await;
    assert!(result.is_ok(), "Failed to list satellites: {:?}", result.err());
    let page = result.unwrap();
    assert!(!page.items.is_empty(), "Expected at least one satellite");
}

#[tokio::test]
async fn test_list_satellites_with_query() {
    let client = get_client();
    let query = SatelliteListQuery {
        ids: None,
        search: Some("ISS".to_string()),
        country: None,
        object_type: None,
        include_decayed: Some(false),
        sort_by: None,
        sort_order: None,
        offset: None,
        limit: Some(5),
    };
    let result = client.list_satellites(Some(&query)).await;
    assert!(result.is_ok(), "Failed to list satellites with query: {:?}", result.err());
}

#[tokio::test]
async fn test_get_satellites_overview() {
    let client = get_client();
    let result = client.get_satellites_overview().await;
    assert!(result.is_ok(), "Failed to get satellites overview: {:?}", result.err());
    let overview = result.unwrap();
    assert!(overview.active_satellite_count > 0, "Expected active_satellite_count > 0");
}

#[tokio::test]
async fn test_get_satellite_countries() {
    let client = get_client();
    let result = client.get_satellite_countries().await;
    assert!(result.is_ok(), "Failed to get satellite countries: {:?}", result.err());
    let countries = result.unwrap();
    assert!(!countries.is_empty(), "Expected at least one country");
}

#[tokio::test]
async fn test_get_satellites_page() {
    let client = get_client();
    let result = client.get_satellites_page(0, 10).await;
    assert!(result.is_ok(), "Failed to get satellites page: {:?}", result.err());
    let page = result.unwrap();
    assert!(!page.satellites.is_empty(), "Expected at least one satellite in page");
}

#[tokio::test]
async fn test_get_satellite() {
    let client = get_client();
    // First list satellites to get an ID
    let page = client.list_satellites(None).await.unwrap();
    if let Some(sat) = page.items.first() {
        let result = client.get_satellite(&sat.id).await;
        assert!(result.is_ok(), "Failed to get satellite: {:?}", result.err());
        let satellite = result.unwrap();
        assert_eq!(satellite.id, sat.id);
    }
}

// ==================== ELSET TESTS ====================

#[tokio::test]
async fn test_get_elset_counts() {
    let client = get_client();
    let result = client.get_elset_counts().await;
    assert!(result.is_ok(), "Failed to get elset counts: {:?}", result.err());
}

// Skipped: /elsets/scatter/near-geo endpoint returns 404 on dev API
#[tokio::test]
#[ignore]
async fn test_get_near_geo_scatter() {
    let client = get_client();
    let result = client.get_near_geo_scatter().await;
    assert!(result.is_ok(), "Failed to get near GEO scatter: {:?}", result.err());
}

// Skipped: /elsets/scatter/leo endpoint returns 404 on dev API
#[tokio::test]
#[ignore]
async fn test_get_leo_scatter() {
    let client = get_client();
    let result = client.get_leo_scatter(Some(6500.0), Some(7000.0)).await;
    assert!(result.is_ok(), "Failed to get LEO scatter: {:?}", result.err());
}

// ==================== GROUND STATION TESTS ====================

#[tokio::test]
async fn test_groundstation_crud() {
    let client = get_client();

    // Create
    let create_request = GroundstationCreateRequest {
        name: format!("Test GS {}", Utc::now().timestamp()),
        latitude: 37.7749,
        longitude: -122.4194,
        altitude: 10.0,
    };
    let create_result = client.create_groundstation(&create_request).await;
    assert!(create_result.is_ok(), "Failed to create groundstation: {:?}", create_result.err());
    let created = create_result.unwrap();
    assert!(!created.id.is_empty());

    // Read
    let get_result = client.get_groundstation(&created.id).await;
    assert!(get_result.is_ok(), "Failed to get groundstation: {:?}", get_result.err());
    let fetched = get_result.unwrap();
    assert_eq!(fetched.id, created.id);

    // Update
    let update_request = GroundstationCreateRequest {
        name: format!("Updated GS {}", Utc::now().timestamp()),
        latitude: 37.7750,
        longitude: -122.4195,
        altitude: 15.0,
    };
    let update_result = client.update_groundstation(&created.id, &update_request).await;
    assert!(update_result.is_ok(), "Failed to update groundstation: {:?}", update_result.err());

    // List
    let list_result = client.list_groundstations().await;
    assert!(list_result.is_ok(), "Failed to list groundstations: {:?}", list_result.err());

    // List my groundstations
    let my_list_result = client.list_my_groundstations().await;
    assert!(my_list_result.is_ok(), "Failed to list my groundstations: {:?}", my_list_result.err());

    // Delete
    let delete_result = client.delete_groundstation(&created.id).await;
    assert!(delete_result.is_ok(), "Failed to delete groundstation: {:?}", delete_result.err());
}

// ==================== TELESCOPE TESTS ====================

#[tokio::test]
async fn test_telescope_list() {
    let client = get_client();
    let result = client.list_telescopes().await;
    assert!(result.is_ok(), "Failed to list telescopes: {:?}", result.err());
}

#[tokio::test]
async fn test_my_telescopes() {
    let client = get_client();
    let result = client.list_my_telescopes().await;
    assert!(result.is_ok(), "Failed to list my telescopes: {:?}", result.err());
}

// ==================== ANTENNA TESTS ====================

#[tokio::test]
async fn test_antenna_list() {
    let client = get_client();
    let result = client.list_antennas().await;
    assert!(result.is_ok(), "Failed to list antennas: {:?}", result.err());
}

#[tokio::test]
async fn test_my_antennas() {
    let client = get_client();
    let result = client.list_my_antennas().await;
    assert!(result.is_ok(), "Failed to list my antennas: {:?}", result.err());
}

// ==================== SATELLITE GROUP TESTS ====================

#[tokio::test]
async fn test_satellite_group_crud() {
    let client = get_client();

    // Create
    let create_request = CreateSatelliteGroupRequest {
        title: format!("Test Group {}", Utc::now().timestamp()),
        details: Some("Integration test group".to_string()),
        satellite_ids: vec![],
    };
    let create_result = client.create_satellite_group(&create_request).await;
    assert!(create_result.is_ok(), "Failed to create satellite group: {:?}", create_result.err());
    let group_id = create_result.unwrap();
    assert!(!group_id.is_empty());

    // Read
    let get_result = client.get_satellite_group(&group_id).await;
    assert!(get_result.is_ok(), "Failed to get satellite group: {:?}", get_result.err());
    let fetched = get_result.unwrap();
    assert_eq!(fetched.id, group_id);

    // Update
    let update_request = UpdateSatelliteGroupRequest {
        id: group_id.clone(),
        title: Some(format!("Updated Group {}", Utc::now().timestamp())),
        details: Some("Updated description".to_string()),
    };
    let update_result = client.update_satellite_group(&update_request).await;
    assert!(update_result.is_ok(), "Failed to update satellite group: {:?}", update_result.err());

    // List
    let list_result = client.list_satellite_groups().await;
    assert!(list_result.is_ok(), "Failed to list satellite groups: {:?}", list_result.err());

    // List my groups
    let my_list_result = client.list_my_satellite_groups().await;
    assert!(my_list_result.is_ok(), "Failed to list my satellite groups: {:?}", my_list_result.err());

    // Get satellites in group
    let sats_result = client.get_satellites_in_group(&group_id).await;
    assert!(sats_result.is_ok(), "Failed to get satellites in group: {:?}", sats_result.err());

    // Delete
    let delete_result = client.delete_satellite_group(&group_id).await;
    assert!(delete_result.is_ok(), "Failed to delete satellite group: {:?}", delete_result.err());
}

#[tokio::test]
async fn test_satellite_group_favorites() {
    let client = get_client();

    // List favorite groups
    let result = client.list_favorite_satellite_groups().await;
    assert!(result.is_ok(), "Failed to list favorite satellite groups: {:?}", result.err());
}

// ==================== ACCESS TESTS ====================

#[tokio::test]
async fn test_get_geo_access() {
    let client = get_client();
    let query = GeoAccessQuery {
        min_longitude_deg: Some(0.0),
        max_longitude_deg: Some(90.0),
        min_semi_major_axis_km: None,
        max_semi_major_axis_km: None,
        max_inclination_deg: Some(15.0),
    };
    let result = client.get_geo_access(&query).await;
    assert!(result.is_ok(), "Failed to get GEO access: {:?}", result.err());
}

// ==================== WEATHER TESTS ====================

#[tokio::test]
async fn test_get_weather() {
    let client = get_client();
    let query = WeatherQuery {
        latitude: 37.7749,
        longitude: -122.4194,
        units: Some("metric".to_string()),
    };
    let result = client.get_weather(&query).await;
    assert!(result.is_ok(), "Failed to get weather: {:?}", result.err());
    let weather = result.unwrap();
    assert!(weather.lat > 0.0);
}

// ==================== FILTER TESTS ====================

#[tokio::test]
// Skipped: /filters endpoint returns 404 on dev API
#[ignore]
async fn test_list_filters() {
    let client = get_client();
    let result = client.list_filters().await;
    assert!(result.is_ok(), "Failed to list filters: {:?}", result.err());
}

// ==================== AUTH TESTS ====================

#[tokio::test]
async fn test_list_personal_access_tokens() {
    let client = get_client();
    let result = client.list_personal_access_tokens().await;
    assert!(result.is_ok(), "Failed to list personal access tokens: {:?}", result.err());
}

// ==================== OBSERVATION TESTS ====================

#[tokio::test]
async fn test_get_optical_observation_counts() {
    let client = get_client();
    let result = client.get_optical_observation_counts().await;
    assert!(result.is_ok(), "Failed to get optical observation counts: {:?}", result.err());
}

// ==================== ALERT SUBSCRIPTION TESTS ====================

#[tokio::test]
async fn test_alert_subscription_crud() {
    let client = get_client();

    // First get a satellite ID to use as the target
    let page = client.list_satellites(Some(&SatelliteListQuery {
        ids: None,
        search: None,
        country: None,
        object_type: None,
        include_decayed: Some(false),
        sort_by: None,
        sort_order: None,
        offset: None,
        limit: Some(1),
    })).await.expect("Failed to list satellites for alert test");

    let satellite_id = page.items.first().map(|s| s.id.clone())
        .expect("Need at least one satellite for alert subscription test");

    // Create
    let create_request = CreateAlertSubscriptionRequest {
        alert_type: AlertType::Maneuver,
        target_type: TargetType::Satellite,
        satellite_id: Some(satellite_id),
        satellite_group_id: None,
        enabled: Some(true),
        email_enabled: Some(false),
        webhook_url: None,
        threshold_value: None,
    };
    let create_result = client.create_alert_subscription(&create_request).await;
    assert!(create_result.is_ok(), "Failed to create alert subscription: {:?}", create_result.err());
    let created = create_result.unwrap();
    assert!(!created.id.is_empty());

    // Read
    let get_result = client.get_alert_subscription(&created.id).await;
    assert!(get_result.is_ok(), "Failed to get alert subscription: {:?}", get_result.err());

    // List
    let list_result = client.list_alert_subscriptions().await;
    assert!(list_result.is_ok(), "Failed to list alert subscriptions: {:?}", list_result.err());

    // Update
    let update_request = UpdateAlertSubscriptionRequest {
        enabled: Some(false),
        email_enabled: None,
        webhook_url: None,
        threshold_value: None,
    };
    let update_result = client.update_alert_subscription(&created.id, &update_request).await;
    assert!(update_result.is_ok(), "Failed to update alert subscription: {:?}", update_result.err());

    // Delete
    let delete_result = client.delete_alert_subscription(&created.id).await;
    assert!(delete_result.is_ok(), "Failed to delete alert subscription: {:?}", delete_result.err());
}

// ==================== MANEUVER TESTS ====================

#[tokio::test]
async fn test_list_maneuvers() {
    let client = get_client();
    let result = client.list_maneuvers(None).await;
    assert!(result.is_ok(), "Failed to list maneuvers: {:?}", result.err());
}

// ==================== IMAGE TESTS ====================

// Skipped: /images endpoint returns 404 on dev API
#[tokio::test]
#[ignore]
async fn test_list_my_images() {
    let client = get_client();
    let result = client.list_my_images(None).await;
    assert!(result.is_ok(), "Failed to list my images: {:?}", result.err());
}

// ==================== COMPLEX INTEGRATION TESTS ====================

/// Tests the complete flow of creating a ground station, listing it, and deleting it
#[tokio::test]
async fn test_groundstation_complete_flow() {
    let client = get_client();

    // 1. Create a new ground station
    let create_request = GroundstationCreateRequest {
        name: format!("Integration Test GS {}", Utc::now().timestamp()),
        latitude: 40.7128,
        longitude: -74.0060,
        altitude: 5.0,
    };

    let created = client.create_groundstation(&create_request).await
        .expect("Failed to create groundstation");
    let gs_id = created.id.clone();

    // 2. Verify it exists in the list
    let all_gs = client.list_groundstations().await
        .expect("Failed to list groundstations");
    assert!(
        all_gs.iter().any(|gs| gs.id == gs_id),
        "Created groundstation not found in list"
    );

    // 3. Verify it's in "my" groundstations
    let my_gs = client.list_my_groundstations().await
        .expect("Failed to list my groundstations");
    assert!(
        my_gs.iter().any(|gs| gs.id == gs_id),
        "Created groundstation not found in my list"
    );

    // 4. Get the specific groundstation
    let fetched = client.get_groundstation(&gs_id).await
        .expect("Failed to get groundstation");
    assert_eq!(fetched.name, create_request.name);
    assert!((fetched.latitude - create_request.latitude).abs() < 0.001);

    // 5. Update the groundstation
    let update_request = GroundstationCreateRequest {
        name: format!("Updated Integration Test GS {}", Utc::now().timestamp()),
        latitude: 40.7130,
        longitude: -74.0065,
        altitude: 10.0,
    };
    let updated = client.update_groundstation(&gs_id, &update_request).await
        .expect("Failed to update groundstation");
    assert_eq!(updated.name, update_request.name);

    // 6. List telescopes and antennas for this groundstation (should be empty)
    let telescopes = client.list_telescopes_for_groundstation(&gs_id).await
        .expect("Failed to list telescopes for groundstation");
    assert!(telescopes.is_empty(), "Expected no telescopes for new groundstation");

    let antennas = client.list_antennas_for_groundstation(&gs_id).await
        .expect("Failed to list antennas for groundstation");
    assert!(antennas.is_empty(), "Expected no antennas for new groundstation");

    // 7. Delete the groundstation
    client.delete_groundstation(&gs_id).await
        .expect("Failed to delete groundstation");

    // 8. Verify it no longer exists
    let result = client.get_groundstation(&gs_id).await;
    assert!(result.is_err(), "Groundstation should be deleted");
}

/// Tests satellite group membership operations
#[tokio::test]
async fn test_satellite_group_membership() {
    let client = get_client();

    // 1. Create a satellite group
    let create_request = CreateSatelliteGroupRequest {
        title: format!("Membership Test Group {}", Utc::now().timestamp()),
        details: Some("Testing membership operations".to_string()),
        satellite_ids: vec![],
    };
    let group_id = client.create_satellite_group(&create_request).await
        .expect("Failed to create satellite group");

    // 2. Get some satellite IDs to add
    let page = client.list_satellites(Some(&SatelliteListQuery {
        ids: None,
        search: None,
        country: None,
        object_type: None,
        include_decayed: Some(false),
        sort_by: None,
        sort_order: None,
        offset: None,
        limit: Some(3),
    })).await.expect("Failed to list satellites");

    if page.items.len() >= 2 {
        let sat_ids: Vec<String> = page.items.iter().take(2).map(|s| s.id.clone()).collect();

        // 3. Add satellites to group
        let add_request = SatelliteGroupMembersRequest {
            satellite_ids: sat_ids.clone(),
        };
        client.add_satellites_to_group(&group_id, &add_request).await
            .expect("Failed to add satellites to group");

        // 4. Verify satellites are in group
        let group_sats = client.get_satellites_in_group(&group_id).await
            .expect("Failed to get satellites in group");
        assert!(group_sats.len() >= 2, "Expected at least 2 satellites in group");

        // 5. Remove satellites from group
        client.remove_satellites_from_group(&group_id, &add_request).await
            .expect("Failed to remove satellites from group");

        // 6. Verify group is empty
        let empty_group = client.get_satellites_in_group(&group_id).await
            .expect("Failed to get satellites in group after removal");
        assert!(empty_group.is_empty(), "Expected no satellites after removal");
    }

    // 7. Clean up - delete the group
    client.delete_satellite_group(&group_id).await
        .expect("Failed to delete satellite group");
}

/// Tests satellite-related queries
#[tokio::test]
async fn test_satellite_queries() {
    let client = get_client();

    // Get a satellite
    let page = client.list_satellites(Some(&SatelliteListQuery {
        ids: None,
        search: None,
        country: None,
        object_type: None,
        include_decayed: Some(false),
        sort_by: None,
        sort_order: None,
        offset: None,
        limit: Some(1),
    })).await.expect("Failed to list satellites");

    if let Some(sat) = page.items.first() {
        // Test various satellite-specific endpoints
        let sat_id = &sat.id;

        // Get satellite
        let fetched = client.get_satellite(sat_id).await
            .expect("Failed to get satellite");
        assert_eq!(fetched.id, *sat_id);

        // Get satellite groups for this satellite
        let _groups = client.get_satellite_groups_for_satellite(sat_id).await
            .expect("Failed to get satellite groups for satellite");
        // Groups may be empty, that's ok

        // Get tasks for satellite
        let _tasks = client.list_tasks_for_satellite(sat_id).await
            .expect("Failed to list tasks for satellite");
        // Tasks may be empty, that's ok

        // Get RF captures for satellite
        let _rf_captures = client.list_rf_captures_for_satellite(sat_id).await
            .expect("Failed to list RF captures for satellite");
        // RF captures may be empty

        // Get images for satellite
        let _images = client.list_images_for_satellite(sat_id, None).await
            .expect("Failed to list images for satellite");
        // Images may be empty

        // Get elsets for satellite
        let _elsets = client.get_satellite_elsets(sat_id).await
            .expect("Failed to get satellite elsets");
        // Elsets may be empty depending on the satellite
    }
}

/// Tests concurrent requests
#[tokio::test]
async fn test_concurrent_requests() {
    let client = get_client();

    // Make several requests concurrently
    let (satellites, overview, countries, weather) = tokio::join!(
        client.list_satellites(Some(&SatelliteListQuery {
            ids: None,
            search: None,
            country: None,
            object_type: None,
            include_decayed: Some(false),
            sort_by: None,
            sort_order: None,
            offset: None,
            limit: Some(5),
        })),
        client.get_satellites_overview(),
        client.get_satellite_countries(),
        client.get_weather(&WeatherQuery {
            latitude: 37.7749,
            longitude: -122.4194,
            units: None,
        })
    );

    assert!(satellites.is_ok(), "Concurrent satellites request failed");
    assert!(overview.is_ok(), "Concurrent overview request failed");
    assert!(countries.is_ok(), "Concurrent countries request failed");
    assert!(weather.is_ok(), "Concurrent weather request failed");
}

/// Tests error handling for non-existent resources
#[tokio::test]
async fn test_error_handling_nonexistent() {
    let client = get_client();

    // Try to get non-existent groundstation
    let result = client.get_groundstation("nonexistent-id-12345").await;
    assert!(result.is_err(), "Expected error for non-existent groundstation");

    // Try to get non-existent satellite group
    let result = client.get_satellite_group("nonexistent-group-id").await;
    assert!(result.is_err(), "Expected error for non-existent satellite group");

    // Try to get non-existent alert subscription
    let result = client.get_alert_subscription("nonexistent-subscription").await;
    assert!(result.is_err(), "Expected error for non-existent subscription");
}

// Debug test for inspecting raw API responses - run with `cargo test test_debug_responses -- --ignored`
#[tokio::test]
#[ignore]
async fn test_debug_responses() {
    let api_key = env::var("CITRA_PAT").expect("CITRA_PAT environment variable must be set");
    let client = reqwest::Client::new();

    // Debug satellite list
    let response = client
        .get("https://dev.api.citra.space/satellites?limit=1")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw satellite list: {}", &text[..text.len().min(500)]);

    // Debug satellite overview
    let response = client
        .get("https://dev.api.citra.space/satellites/overview")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw satellite overview: {}", text);

    // Debug weather
    let response = client
        .get("https://dev.api.citra.space/weather?lat=40.7128&lon=-74.0060")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw weather: {}", &text[..text.len().min(500)]);

    // Debug satellite groups
    let response = client
        .get("https://dev.api.citra.space/satellite-groups")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw satellite groups: {}", &text[..text.len().min(500)]);

    // Debug filters
    let response = client
        .get("https://dev.api.citra.space/filters")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw filters: {}", &text[..text.len().min(500)]);

    // Debug telescopes - show more output to find the map issue around position 6506
    let response = client
        .get("https://dev.api.citra.space/telescopes")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw telescopes length: {}", text.len());
    if text.len() > 6400 {
        println!("Raw telescopes around 6506: {}", &text[6400..text.len().min(6700)]);
    }
    println!("Raw telescopes: {}", &text[..text.len().min(1000)]);

    // Debug personal access tokens
    let response = client
        .get("https://dev.api.citra.space/auth/personal-access-tokens")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw PAT: {}", &text[..text.len().min(500)]);

    // Debug images
    let response = client
        .get("https://dev.api.citra.space/images?limit=1")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw images: {}", &text[..text.len().min(1000)]);

    // Debug GEO scatter
    let response = client
        .get("https://dev.api.citra.space/elsets/scatter/near-geo")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw GEO scatter: {}", &text[..text.len().min(500)]);

    // Debug LEO scatter
    let response = client
        .get("https://dev.api.citra.space/elsets/scatter/leo")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw LEO scatter: {}", &text[..text.len().min(500)]);

    // Debug maneuvers
    let response = client
        .get("https://dev.api.citra.space/maneuvers")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw maneuvers: {}", &text[..text.len().min(500)]);

    // Debug GEO access
    let response = client
        .get("https://dev.api.citra.space/access/geo?minLongitude=0&maxLongitude=90&maxInclination=15")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw GEO access: {}", &text[..text.len().min(500)]);

    // Debug elsets for a satellite
    let response = client
        .get("https://dev.api.citra.space/satellites/7a42d88e-4516-4d10-aba5-f1f3d23dc50d/elsets?limit=1")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw elsets: {}", &text[..text.len().min(1000)]);

    // Debug satellites page
    let response = client
        .get("https://dev.api.citra.space/satellites/page?offset=0&limit=5")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    println!("Raw satellites page: {}", &text[..text.len().min(500)]);
}
