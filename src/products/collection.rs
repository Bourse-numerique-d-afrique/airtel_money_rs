use crate::requests::collection_refund_request::RefundCollectionRequest;
use crate::requests::ussd_push_request::{USSDSubscriberRequest, USSDTransactionRequest, UssdPushRequest};
use crate::{Country, Currency, Environment};
use crate::authorization::get_valid_access_token;


pub struct Collection {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}


impl Collection {
    /*
        * Create a new instance of Collection
        @param country: Country
        @param currency: Currency
        @param environment: Environment
        @param client_id: String
        @param client_secret: String
        @return Collection
    
     */
    pub fn new(country: Country, currency: Currency, environment: Environment, client_id:String, client_secret:String) -> Collection {
        Collection {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }


    /*
        * Push USSD
        @param reference: String
        @param msisdn: String
        @param amount: i32
        @param id: String
        @return Result<(), Box<dyn std::error::Error>>
     */
    async fn ussd_push(&self, reference: String, msisdn: String, amount: i32, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client.post(format!("{}/merchant/v1/payments/", self.environment))
        .bearer_auth(access_token.access_token)
        .header("Content-Type", "application/json")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string())
        .body(
            UssdPushRequest {
                reference,
                subscriber: USSDSubscriberRequest {
                    country: self.country,
                    msisdn,
                    currency: self.currency,
                },
                transaction: USSDTransactionRequest {
                    amount,
                    country: self.country,
                    currency: self.currency,
                    id,
                },
            }
        );


        let res = req.send().await?;
        if res.status().is_success() {
            Ok(())
        } else{
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }


    /*
        * Refund a collection
        @param id: String
        @return Result<(), Box<dyn std::std::error::Error>>
     */
    async fn refund(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client.post(format!("{}/standard/v1/payments/refund", self.environment))
        .bearer_auth(access_token.access_token)
        .header("Content-Type", "application/json")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string())
        .body(
            RefundCollectionRequest {
                airtel_money_id: id,
            }
        );


        let res = req.send().await?;
        if res.status().is_success() {
            Ok(())
        } else{
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }


    /*
        * Get the status of a payment
        @param id: String
        @return Result<(), Box<dyn std::std::error::Error>>
     */
    async fn status(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let res = client.get(format!("{}/standard/v1/payments/status/{}", self.environment, id))
        .bearer_auth(access_token.access_token)
        .header("Content-Type", "application/json")
        .header("X-Country", self.country.to_string())
        .header("X-Currency", self.currency.to_string())
        .send().await?;

        if res.status().is_success() {
            Ok(())
        } else{
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }
}