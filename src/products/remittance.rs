

use crate::{ authorization::get_valid_access_token, Country, Currency, Environment};


pub struct Remittance {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl Remittance {
    /*
        * Create a new instance of Remittance
        @param country: Country
        @param currency: Currency
        @param environment: Environment
        @param client_id: String
        @param client_secret: String
        @return Remittance
     */
    pub fn new(country: Country, currency: Currency, environment: Environment, client_id: String, client_secret: String) -> Self {
        Remittance {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }


    /*
        * Check eligibility
        @return Result<(), Box<dyn std::error::Error>>
     */
    async fn check_eligibility(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client.get(format!("{} /openapi/moneytransfer/v2/validate", self.environment))
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send().await?;

        if res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    /*
        * Get money transfer status
        @return Result<(), Box<dyn std::error::Error>>
     */
    pub async fn money_transfer_status(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client.post(format!("{}/openapi/moneytransfer/v2/checkstatus/", self.environment))
        .bearer_auth(token.access_token)
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .send().await?;

        if res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    /*
        * Credit a money transfer
        @param id: String
        @return Result<(), Box<dyn std::std::error::Error>>
     */
    pub async fn money_transfer_credit(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client.post(format!("{}/openapi/moneytransfer/v2/credit", self.environment))
        .bearer_auth(token.access_token)
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .send().await?;

        if res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    /*
        * Refund a money transfer
        @param id: String
        @return Result<(), Box<dyn std::std::error::Error>>
     */
    pub async fn refund(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client.post(format!("{}/openapi/moneytransfer/v2/refund", self.environment))
        .bearer_auth(token.access_token)
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .send().await?;

        if res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }
    
}

