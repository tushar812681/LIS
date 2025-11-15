use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Client for communicating with patient-service
#[derive(Clone)]
pub struct PatientClient {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub path: Option<Vec<String>>,
}

impl PatientClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Execute a GraphQL query/mutation against patient-service
    pub async fn execute<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: Option<Value>,
    ) -> Result<T, String> {
        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let url = format!("{}/graphql", self.base_url);

        tracing::debug!("Calling patient-service: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to call patient-service: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            tracing::error!("Patient-service returned error {}: {}", status, body);
            return Err(format!("Patient-service returned error {}: {}", status, body));
        }

        let graphql_response: GraphQLResponse<T> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse patient-service response: {}", e))?;

        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            let error_msg = error_messages.join(", ");
            tracing::error!("Patient-service GraphQL errors: {}", error_msg);
            return Err(format!("Patient-service errors: {}", error_msg));
        }

        graphql_response
            .data
            .ok_or_else(|| "No data returned from patient-service".to_string())
    }

    /// Query patients list
    pub async fn get_patients(
        &self,
        organization_id: Option<String>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Value, String> {
        let query = r#"
            query GetPatients($organizationId: String, $page: Int, $pageSize: Int) {
                patients(organizationId: $organizationId, page: $page, pageSize: $pageSize) {
                    id
                    mrnNumber
                    firstName
                    lastName
                    dateOfBirth
                    gender
                    mobileNumber
                    email
                    createdAt
                    updatedAt
                }
            }
        "#;

        let variables = serde_json::json!({
            "organizationId": organization_id,
            "page": page,
            "pageSize": page_size
        });

        self.execute(query, Some(variables)).await
    }

    /// Get patient by ID
    pub async fn get_patient(&self, id: String) -> Result<Value, String> {
        let query = r#"
            query GetPatient($id: String!) {
                patient(id: $id) {
                    id
                    mrnNumber
                    firstName
                    lastName
                    dateOfBirth
                    gender
                    mobileNumber
                    email
                    address
                    city
                    state
                    pincode
                    country
                    organizationId
                    createdAt
                    updatedAt
                    createdBy
                    updatedBy
                }
            }
        "#;

        let variables = serde_json::json!({ "id": id });

        self.execute(query, Some(variables)).await
    }

    /// Search patients
    pub async fn search_patients(
        &self,
        query: String,
        organization_id: Option<String>,
        limit: Option<i32>,
    ) -> Result<Value, String> {
        let graphql_query = r#"
            query SearchPatients($query: String!, $organizationId: String, $limit: Int) {
                searchPatients(query: $query, organizationId: $organizationId, limit: $limit) {
                    id
                    mrnNumber
                    firstName
                    lastName
                    dateOfBirth
                    gender
                    mobileNumber
                    email
                }
            }
        "#;

        let variables = serde_json::json!({
            "query": query,
            "organizationId": organization_id,
            "limit": limit
        });

        self.execute(graphql_query, Some(variables)).await
    }

    /// Create patient
    pub async fn create_patient(
        &self,
        input: Value,
        organization_id: String,
        created_by: String,
    ) -> Result<Value, String> {
        let query = r#"
            mutation CreatePatient($input: CreatePatientInput!, $organizationId: String!, $createdBy: String!) {
                createPatient(input: $input, organizationId: $organizationId, createdBy: $createdBy) {
                    id
                    mrnNumber
                    firstName
                    lastName
                    dateOfBirth
                    gender
                    mobileNumber
                    email
                    createdAt
                }
            }
        "#;

        let variables = serde_json::json!({
            "input": input,
            "organizationId": organization_id,
            "createdBy": created_by
        });

        self.execute(query, Some(variables)).await
    }

    /// Update patient
    pub async fn update_patient(
        &self,
        id: String,
        input: Value,
        updated_by: String,
    ) -> Result<Value, String> {
        let query = r#"
            mutation UpdatePatient($id: String!, $input: UpdatePatientInput!, $updatedBy: String!) {
                updatePatient(id: $id, input: $input, updatedBy: $updatedBy) {
                    id
                    mrnNumber
                    firstName
                    lastName
                    dateOfBirth
                    gender
                    mobileNumber
                    email
                    updatedAt
                }
            }
        "#;

        let variables = serde_json::json!({
            "id": id,
            "input": input,
            "updatedBy": updated_by
        });

        self.execute(query, Some(variables)).await
    }
}
