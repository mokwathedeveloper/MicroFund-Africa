use uuid::Uuid;

pub struct MpesaService;

#[derive(Debug, serde::Serialize)]
pub struct MpesaResponse {
    pub checkout_request_id: String,
    pub response_code: String,
    pub customer_message: String,
}

impl MpesaService {
    /// Simulates initiating an STK Push (Lipa na M-Pesa Online).
    pub async fn initiate_stk_push(phone_number: &str, amount: f64) -> Result<MpesaResponse, String> {
        log::info!(
            "[M-PESA] Initiating STK Push for {} - Amount: KES {:.2}",
            phone_number,
            amount
        );
        
        // Simulate a short delay for the network request
        // tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        Ok(MpesaResponse {
            checkout_request_id: Uuid::new_v4().to_string(),
            response_code: "0".to_string(),
            customer_message: "Success. Request accepted for processing".to_string(),
        })
    }

    /// Simulates the callback from M-Pesa after user enters PIN.
    pub async fn verify_payment(checkout_request_id: &str) -> Result<bool, String> {
        log::info!("[M-PESA] Verifying payment for request ID: {}", checkout_request_id);
        // In a real app, this would check the status in the DB updated by a callback
        Ok(true)
    }
}
