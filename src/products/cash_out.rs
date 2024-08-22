use crate::{authorization::get_valid_access_token, requests::cash_out_request_payment::CashOutRequestPayment, Country, Currency, Environment};




pub struct CashOut{
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl CashOut {
    /*
        * Create a new instance of CashOut
        @param country: Country
        @param currency: Currency
        @param environment: Environment
        @param client_id: String
        @param client_secret: String
        @return CashOut
     */
    pub fn new(country: Country, currency: Currency, environment: Environment, client_id: String, client_secret: String) -> Self {
        CashOut {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /*
        * Cash out
        @return Result<(), Box<dyn std::error::Error>>
     */
    pub async fn cash_out(&self)  -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment,  &self.client_id, &self.client_secret).await?;
        let req = client.post(format!("{} /standard/v1/cashout/", self.environment))
        .bearer_auth(access_token.access_token)
        .header("Content-Type", "application/json")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string())
        .header("x-signature", self.currency.to_string())
        .header("x-key", self.currency.to_string());
        // .body(
        //     CashOutRequestPayment {
        //         reference,
        //         subscriber: Subscriber {
        //             msisdn,
        //         },
        //         transaction: Transaction {
        //             amount,
        //             id,
        //         },
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
        * Get the status of a cash out
        @param id: String
        @return Result<(), Box<dyn std::error::Error>>
     */
    pub async fn get_status(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment,  &self.client_id, &self.client_secret).await?;
        let req = client.get(format!("{}/standard/v1/payments/{}", self.environment, id.to_string()))
        .bearer_auth(access_token.access_token)
        .header("Content-Type", "application/json")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string())
        .header("x-signature", self.currency.to_string())
        .header("x-key", self.currency.to_string());
        let res = req.send().await?;
        if  res.status().is_success() {
            Ok(())
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }
}