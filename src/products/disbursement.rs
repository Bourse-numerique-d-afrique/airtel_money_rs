

use crate::{ authorization::get_valid_access_token, requests::disbursement_payment_request::DisbursementPaymentRequest, Country, Currency, Environment};



pub struct Disbursement {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl Disbursement {
    /*
        * Create a new instance of Disbursement
        @param country: Country
        @param currency: Currency
        @param environment: Environment
        @param client_id: String
        @param client_secret: String
        @return Disbursement
     */
    pub fn new(country: Country, currency: Currency, environment: Environment, client_id: String, client_secret: String) -> Self {
        Disbursement {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /*
        * Disburse
        @return Result<(), Box<dyn std::error::Error>>
     */
    pub async fn disburse(&self)  -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client.post(format!("{}/standard/v1/disbursements/", self.environment))
        .bearer_auth(access_token.access_token)
        .header("Content-Type", "application/json")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string())
        .header("x-signature", self.currency.to_string())
        .header("x-key", self.currency.to_string())
        .body(
            DisbursementPaymentRequest {
                payee: todo!(),
                reference: todo!(),
                pin: todo!(),
                transaction: todo!(),
            }
        );
        let res = req.send().await?;
        if  res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    /*
        * Get Status
        @param id: String
        @return Result<(), Box<dyn std::error::Error>>
     */
    pub async fn get_status(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client.get(format!("{}/standard/v1/disburse/{}", self.environment, id.to_string()))
        .bearer_auth(access_token.access_token)
        .header("Accept", "*/*")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string());

        let res = req.send().await?;

        if res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
        
    } 
}


