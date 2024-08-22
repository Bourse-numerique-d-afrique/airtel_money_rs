use crate::{authorization::get_valid_access_token, requests::cash_in_payment_request::CashInPaymentRequest, Country, Currency, Environment};


pub struct CashIn {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl CashIn {
    /*
        * Create a new instance of CashIn
        @param country: Country
        @param currency: Currency
        @param environment: Environment
        @param client_id: String
        @param client_secret: String
        @return CashIn
     */
    pub fn new(country: Country, currency: Currency, environment: Environment, client_id: String, client_secret: String) -> Self {
        CashIn {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /*
        * Cash in
        @return Result<(), Box<dyn std::error::Error>>
     */
    pub async fn cash_in(&self)  -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client.post(format!("{}/standard/v2/cashin/", self.environment))
        .bearer_auth(access_token.access_token)
        .header("Content-Type", "application/json")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string())
        .header("x-signature", self.currency.to_string())
        .header("x-key", self.currency.to_string());
        // .body(
        //     CashInPaymentRequest {
        //         todo!(),
        //         subscriber: todo!(),
        //         transaction: todo!(),
        //         additional_info: vec![AdditionalInfo {
        //             key: "remark".to_string(),
        //             value: "AIRTXXXXXX".to_string(),
        //         }],
        //         pin,
        //     }
        // );
        let res = req.send().await?;
        if  res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    /*
        * Get the status of a cash in
        @param id: String
        @return Result<(), Box<dyn std::error::Error>>
     */
    pub async fn get_status(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client.get(format!("{}/standard/v1/cashin/{}", self.environment, id.to_string()))
        .bearer_auth(access_token.access_token)
        .header("Content-Type", "application/json")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string());
        let res = req.send().await?;
        if  res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }
}