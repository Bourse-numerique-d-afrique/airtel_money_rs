
use reqwest::Body;

use crate::{Country, Currency};

pub struct RemittanceTransferStatusRequest {
    country: Country,
    ext_tr_id: String,
}

impl From<RemittanceTransferStatusRequest> for Body {
    fn from(remittance_transfer_status_request: RemittanceTransferStatusRequest) -> Self {
        let t = format!("country={}&extTRID={}", remittance_transfer_status_request.country, remittance_transfer_status_request.ext_tr_id);
        Body::from(t)
    }
}