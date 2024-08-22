use std::sync::Arc;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;


mod products;
mod enums;
mod requests;
mod responses;

pub type Environment = enums::environment::Environment;
pub type Currency = enums::currency::Currency;
pub type Country = enums::country::Country;
pub type Collection = products::collection::Collection;
pub type Disbursement = products::disbursement::Disbursement;
pub type Remittance = products::remittance::Remittance;
pub type CashIn = products::cash_in::CashIn;
pub type CashOut = products::cash_out::CashOut;


pub type TokenResponse = responses::token_response::TokenResponse;
pub type TokenRequest = requests::token_request::TokenRequest;

pub struct AirtelMoney {
    pub environment: Environment,
    pub country: Country,
    pub currency: Currency,
}

static ACCESS_TOKEN: Lazy<Arc<Mutex<Option<TokenResponse>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});


mod authorization {
    use chrono::Utc;
    use tokio::task;

    use crate::{ Environment, TokenRequest, TokenResponse, ACCESS_TOKEN};

    async fn create_access_token(environment: Environment, client_id: &str, client_secret: &str) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let res = client.post(format!("{}/auth/oauth2/token", environment))
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .body(TokenRequest {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            grant_type: "client_credentials".to_string(),
        })
        .send().await?;


        if res.status().is_success() {
            let body = res.text().await?;
            let token_response: TokenResponse = serde_json::from_str(&body)?;
            let cloned = token_response.clone();
            let _t = task::spawn(async move {
                let mut token = ACCESS_TOKEN.lock().await;
                *token = Some(token_response.clone());
            });
            Ok(cloned)
        }else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    pub async fn get_valid_access_token(environment: Environment, client_id: &str, client_secret: &str) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let token = ACCESS_TOKEN.lock().await;
        if token.is_some() {
            let token = token.clone().unwrap();
            if token.created_at.is_some() {
                let created_at = token.created_at.unwrap();
                let expires_in = token.expires_in;
                let now = Utc::now();
                let duration = now.signed_duration_since(created_at);
                if duration.num_seconds() < expires_in as i64 {
                    return Ok(token);
                }
                let token: TokenResponse = create_access_token(environment, client_id, client_secret).await?;
                return Ok(token);

            }
        }
        let token: TokenResponse = create_access_token(environment, client_id, client_secret).await?;
        return Ok(token);
    }
}



impl AirtelMoney {
    /*
        * Create a new instance of AirtelMoney
        @param environment: Environment
        @param country: Country
        @return AirtelMoney
     */
    pub fn new(environment: Environment, country: Country) -> Self {
        let currency = match country {
            Country::Uganda => Currency::UGX,
            Country::Kenya => Currency::KES,
            Country::Tanzania => Currency::TZS,
            Country::Madagascar => Currency::MGA,
            Country::DRC => Currency::CDF,
            Country::Zambia => Currency::ZMW,
            Country::Seychelles => Currency::SCR,
            Country::Rwanda => Currency::RWF,
            Country::Malawi => Currency::MWK,
            Country::Nigeria => Currency::NGN,
            Country::Niger => Currency::XOF,
            Country::Chad => Currency::XAF,
            Country::Gabon => Currency::XAF,
            Country::CongoB => Currency::XAF,
        };
        AirtelMoney {
            environment,
            country,
            currency,
        }
    }

    /*
        get the curency of the country
        @return Currency
    
     */
    pub fn get_currency(&self) -> Currency {
        self.currency
    }

    /*
        get the country of the instance
        @return Country
     */
    pub fn get_country(&self) -> Country {
        self.country
    }


    // /*
    //     get the collection product
    //     @return Collection
    //  */
    // pub fn collection(&self) -> Collection {
    //     Collection::new(self)
    // }

    // /*
    //     get the disbursement product
    //     @return Disbursement
    //  */
    // pub fn disbursement(&self) -> Disbursement {
    //     Disbursement::new(self)
    // }

    // /*
    //     get the remittance product
    //     @return Remittance
    //  */
    // pub fn remittance(&self) -> Remittance {
    //     Remittance::new(self)
    // }


    // /*
    //     get the cash in product
    //     @return CashIn
    //  */
    // pub fn cash_in(&self) -> CashIn {
    //     CashIn::new(self)
    // }


    // /*
    //     get the cash out product
    //     @return CashOut
    //  */
    // pub fn cash_out(&self) -> CashOut {
    //     CashOut::new(self)
    // }
}