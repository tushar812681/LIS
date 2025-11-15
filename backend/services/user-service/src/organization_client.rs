use serde::{Deserialize, Serialize};
use uuid::Uuid;
use common::error::{Error, Result};

/// Client for communicating with the organization-service
#[derive(Clone)]
pub struct OrganizationClient {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct CreateOrganizationResponse {
    #[serde(rename = "createOrganization")]
    create_organization: OrganizationData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrganizationData {
    pub id: String,
    #[serde(rename = "organizationName")]
    pub organization_name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct CreateOrganizationInput {
    pub organization_name: String,
    pub legal_name: Option<String>,
    pub organization_type: String,
    pub email: String,
    pub phone: Option<String>,
}

impl OrganizationClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Create a new organization via GraphQL mutation
    pub async fn create_organization(
        &self,
        input: CreateOrganizationInput,
    ) -> Result<OrganizationData> {
        let query = r#"
            mutation CreateOrganization($input: CreateOrganizationInput!) {
                createOrganization(input: $input) {
                    id
                    organizationName
                    email
                }
            }
        "#;

        let variables = serde_json::json!({
            "input": {
                "organizationName": input.organization_name,
                "legalName": input.legal_name,
                "organizationType": input.organization_type,
                "email": input.email,
                "phone": input.phone,
            }
        });

        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let url = format!("{}/graphql", self.base_url);

        tracing::info!("Creating organization via organization-service at {}", url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to call organization-service: {}", e);
                Error::ExternalService(format!("Failed to connect to organization-service: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            tracing::error!("Organization-service returned error {}: {}", status, body);
            return Err(Error::ExternalService(
                format!("Organization-service returned error {}: {}", status, body)
            ));
        }

        let graphql_response: GraphQLResponse<CreateOrganizationResponse> = response
            .json()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse organization-service response: {}", e);
                Error::ExternalService(format!("Invalid response from organization-service: {}", e))
            })?;

        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            let error_msg = error_messages.join(", ");
            tracing::error!("Organization-service GraphQL errors: {}", error_msg);
            return Err(Error::ExternalService(
                format!("Organization creation failed: {}", error_msg)
            ));
        }

        let data = graphql_response.data
            .ok_or_else(|| Error::ExternalService("No data returned from organization-service".to_string()))?;

        tracing::info!("Organization created successfully: {}", data.create_organization.id);

        Ok(data.create_organization)
    }

    /// Delete/cleanup an organization (for rollback scenarios)
    pub async fn delete_organization(&self, organization_id: Uuid) -> Result<()> {
        let query = r#"
            mutation DeleteOrganization($id: UUID!) {
                deleteOrganization(id: $id)
            }
        "#;

        let variables = serde_json::json!({
            "id": organization_id.to_string()
        });

        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let url = format!("{}/graphql", self.base_url);

        tracing::warn!("Rolling back organization creation: {}", organization_id);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to rollback organization: {}", e);
                Error::ExternalService(format!("Failed to rollback organization: {}", e))
            })?;

        if !response.status().is_success() {
            tracing::error!("Failed to rollback organization {}", organization_id);
        } else {
            tracing::info!("Organization {} rolled back successfully", organization_id);
        }

        Ok(())
    }
}
