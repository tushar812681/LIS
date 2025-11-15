use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::error;

use common::error::{Error, Result};

// Base HTTP client for all external APIs
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        headers: Vec<(&str, &str)>,
    ) -> Result<T> {
        let mut request = self.client.get(url);

        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await
            .map_err(|e| Error::ExternalService(format!("HTTP GET failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::ExternalService(
                format!("HTTP {} error", response.status())
            ));
        }

        response.json().await
            .map_err(|e| Error::ExternalService(format!("JSON parse failed: {}", e)))
    }

    pub async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        body: &T,
        headers: Vec<(&str, &str)>,
    ) -> Result<R> {
        let mut request = self.client.post(url).json(body);

        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await
            .map_err(|e| Error::ExternalService(format!("HTTP POST failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ExternalService(
                format!("HTTP {} error: {}", status, error_text)
            ));
        }

        response.json().await
            .map_err(|e| Error::ExternalService(format!("JSON parse failed: {}", e)))
    }
}

// UIDAI (Aadhaar) Client
pub mod uidai {
    use super::*;

    #[derive(Clone)]
    pub struct UidaiClient {
        http_client: HttpClient,
        api_url: String,
        client_id: String,
        client_secret: String,
    }

    #[derive(Serialize)]
    pub struct AadhaarOTPRequest {
        pub aadhaar: String,
        pub txn_id: String,
    }

    #[derive(Deserialize)]
    pub struct AadhaarOTPResponse {
        pub status: String,
        pub txn_id: String,
        pub message: String,
    }

    #[derive(Serialize)]
    pub struct AadhaarVerifyRequest {
        pub aadhaar: String,
        pub otp: String,
        pub txn_id: String,
    }

    #[derive(Deserialize)]
    pub struct AadhaarVerifyResponse {
        pub status: String,
        pub name: String,
        pub dob: String,
        pub gender: String,
        pub address: String,
    }

    impl UidaiClient {
        pub fn new(api_url: String, client_id: String, client_secret: String) -> Self {
            Self {
                http_client: HttpClient::new(),
                api_url,
                client_id,
                client_secret,
            }
        }

        pub async fn send_otp(&self, aadhaar: &str) -> Result<AadhaarOTPResponse> {
            let url = format!("{}/api/v1/otp/send", self.api_url);
            let request = AadhaarOTPRequest {
                aadhaar: aadhaar.to_string(),
                txn_id: uuid::Uuid::new_v4().to_string(),
            };

            let headers = vec![
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
            ];

            self.http_client.post(&url, &request, headers).await
                .map_err(|e| Error::UidaiError(e.to_string()))
        }

        pub async fn verify_otp(&self, aadhaar: &str, otp: &str, txn_id: &str) -> Result<AadhaarVerifyResponse> {
            let url = format!("{}/api/v1/otp/verify", self.api_url);
            let request = AadhaarVerifyRequest {
                aadhaar: aadhaar.to_string(),
                otp: otp.to_string(),
                txn_id: txn_id.to_string(),
            };

            let headers = vec![
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
            ];

            self.http_client.post(&url, &request, headers).await
                .map_err(|e| Error::UidaiError(e.to_string()))
        }
    }
}

// ABDM (Ayushman Bharat Digital Mission) Client
pub mod abdm {
    use super::*;

    #[derive(Clone)]
    pub struct AbdmClient {
        http_client: HttpClient,
        api_url: String,
        client_id: String,
        client_secret: String,
    }

    #[derive(Serialize)]
    pub struct HealthIdCreateRequest {
        pub mobile: String,
        pub name: String,
        pub gender: String,
        pub dob: String,
    }

    #[derive(Deserialize)]
    pub struct HealthIdResponse {
        pub health_id: String,
        pub health_id_number: String,
        pub name: String,
        pub gender: String,
        pub dob: String,
        pub mobile: String,
    }

    impl AbdmClient {
        pub fn new(api_url: String, client_id: String, client_secret: String) -> Self {
            Self {
                http_client: HttpClient::new(),
                api_url,
                client_id,
                client_secret,
            }
        }

        pub async fn create_health_id(&self, request: HealthIdCreateRequest) -> Result<HealthIdResponse> {
            let url = format!("{}/api/v1/registration/aadhaar/createHealthId", self.api_url);
            let headers = vec![
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
            ];

            self.http_client.post(&url, &request, headers).await
                .map_err(|e| Error::AbdmError(e.to_string()))
        }

        pub async fn verify_health_id(&self, health_id: &str) -> Result<HealthIdResponse> {
            let url = format!("{}/api/v1/search/searchByHealthId", self.api_url);

            let headers = vec![
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
            ];

            let body = serde_json::json!({ "healthId": health_id });

            self.http_client.post(&url, &body, headers).await
                .map_err(|e| Error::AbdmError(e.to_string()))
        }
    }
}

// WhatsApp Business API Client
pub mod whatsapp {
    use super::*;

    #[derive(Clone)]
    pub struct WhatsAppClient {
        http_client: HttpClient,
        api_url: String,
        phone_number_id: String,
        access_token: String,
    }

    #[derive(Serialize)]
    pub struct WhatsAppTextMessage {
        pub messaging_product: String,
        pub to: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub text: TextContent,
    }

    #[derive(Serialize)]
    pub struct TextContent {
        pub body: String,
    }

    #[derive(Serialize)]
    pub struct WhatsAppTemplateMessage {
        pub messaging_product: String,
        pub to: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub template: Template,
    }

    #[derive(Serialize)]
    pub struct Template {
        pub name: String,
        pub language: Language,
        pub components: Vec<Component>,
    }

    #[derive(Serialize)]
    pub struct Language {
        pub code: String,
    }

    #[derive(Serialize)]
    pub struct Component {
        #[serde(rename = "type")]
        pub type_: String,
        pub parameters: Vec<Parameter>,
    }

    #[derive(Serialize)]
    pub struct Parameter {
        #[serde(rename = "type")]
        pub type_: String,
        pub text: String,
    }

    #[derive(Deserialize)]
    pub struct WhatsAppResponse {
        pub messaging_product: String,
        pub contacts: Vec<Contact>,
        pub messages: Vec<Message>,
    }

    #[derive(Deserialize)]
    pub struct Contact {
        pub input: String,
        pub wa_id: String,
    }

    #[derive(Deserialize)]
    pub struct Message {
        pub id: String,
    }

    impl WhatsAppClient {
        pub fn new(api_url: String, phone_number_id: String, access_token: String) -> Self {
            Self {
                http_client: HttpClient::new(),
                api_url,
                phone_number_id,
                access_token,
            }
        }

        pub async fn send_text(&self, to: &str, message: &str) -> Result<WhatsAppResponse> {
            let url = format!("{}/{}/messages", self.api_url, self.phone_number_id);

            let request = WhatsAppTextMessage {
                messaging_product: "whatsapp".to_string(),
                to: to.to_string(),
                type_: "text".to_string(),
                text: TextContent {
                    body: message.to_string(),
                },
            };

            let auth_header = format!("Bearer {}", self.access_token);
            let headers = vec![
                ("Authorization", auth_header.as_str()),
            ];

            self.http_client.post(&url, &request, headers).await
                .map_err(|e| Error::WhatsAppError(e.to_string()))
        }

        pub async fn send_template(
            &self,
            to: &str,
            template_name: &str,
            parameters: Vec<String>,
        ) -> Result<WhatsAppResponse> {
            let url = format!("{}/{}/messages", self.api_url, self.phone_number_id);

            let components = vec![Component {
                type_: "body".to_string(),
                parameters: parameters
                    .into_iter()
                    .map(|p| Parameter {
                        type_: "text".to_string(),
                        text: p,
                    })
                    .collect(),
            }];

            let request = WhatsAppTemplateMessage {
                messaging_product: "whatsapp".to_string(),
                to: to.to_string(),
                type_: "template".to_string(),
                template: Template {
                    name: template_name.to_string(),
                    language: Language {
                        code: "en".to_string(),
                    },
                    components,
                },
            };

            let auth_header = format!("Bearer {}", self.access_token);
            let headers = vec![
                ("Authorization", auth_header.as_str()),
            ];

            self.http_client.post(&url, &request, headers).await
                .map_err(|e| Error::WhatsAppError(e.to_string()))
        }
    }
}

// Payment Gateway Client (Razorpay)
pub mod payment {
    use super::*;
    use rust_decimal::Decimal;

    #[derive(Clone)]
    pub struct PaymentClient {
        http_client: HttpClient,
        api_url: String,
        key_id: String,
        key_secret: String,
    }

    #[derive(Serialize)]
    pub struct CreateOrderRequest {
        pub amount: i64, // Amount in paise
        pub currency: String,
        pub receipt: String,
        pub notes: serde_json::Value,
    }

    #[derive(Deserialize)]
    pub struct OrderResponse {
        pub id: String,
        pub entity: String,
        pub amount: i64,
        pub currency: String,
        pub receipt: String,
        pub status: String,
    }

    #[derive(Deserialize)]
    pub struct PaymentResponse {
        pub id: String,
        pub entity: String,
        pub amount: i64,
        pub currency: String,
        pub status: String,
        pub order_id: String,
        pub method: String,
    }

    impl PaymentClient {
        pub fn new(api_url: String, key_id: String, key_secret: String) -> Self {
            Self {
                http_client: HttpClient::new(),
                api_url,
                key_id,
                key_secret,
            }
        }

        pub async fn create_order(&self, request: CreateOrderRequest) -> Result<OrderResponse> {
            use base64::{Engine as _, engine::general_purpose};

            let url = format!("{}/v1/orders", self.api_url);

            let auth = general_purpose::STANDARD.encode(format!("{}:{}", self.key_id, self.key_secret));
            let auth_header = format!("Basic {}", auth);
            let headers = vec![
                ("Authorization", auth_header.as_str()),
            ];

            self.http_client.post(&url, &request, headers).await
                .map_err(|e| Error::PaymentGatewayError(e.to_string()))
        }

        pub async fn get_payment(&self, payment_id: &str) -> Result<PaymentResponse> {
            use base64::{Engine as _, engine::general_purpose};

            let url = format!("{}/v1/payments/{}", self.api_url, payment_id);

            let auth = general_purpose::STANDARD.encode(format!("{}:{}", self.key_id, self.key_secret));
            let auth_header = format!("Basic {}", auth);
            let headers = vec![
                ("Authorization", auth_header.as_str()),
            ];

            self.http_client.get(&url, headers).await
                .map_err(|e| Error::PaymentGatewayError(e.to_string()))
        }

        pub fn verify_signature(&self, order_id: &str, payment_id: &str, signature: &str) -> Result<bool> {
            use hmac::{Hmac, Mac};
            use sha2::Sha256;

            type HmacSha256 = Hmac<Sha256>;

            let message = format!("{}|{}", order_id, payment_id);

            let mut mac = HmacSha256::new_from_slice(self.key_secret.as_bytes())
                .map_err(|_| Error::PaymentGatewayError("Invalid key".to_string()))?;

            mac.update(message.as_bytes());

            let result = mac.finalize();
            let expected_signature = hex::encode(result.into_bytes());

            Ok(expected_signature == signature)
        }
    }
}
