use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusItem {
    result: Vec<ResultResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultResponse {
    #[serde(rename = "routeDetail")]
    route_detail: RouteDetail,
    #[serde(rename = "countryDetail")]
    country_detail: CountryDetail,
    #[serde(rename = "operatorDetail")]
    operator_detail: OperatorDetail,
    status: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryDetail {
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(rename = "mobileCountryCode")]
    mobile_country_code: String,
    iso: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatorDetail {
    #[serde(rename = "operatorCode")]
    operator_code: String,
    #[serde(rename = "operatorName")]
    operator_name: String,
    #[serde(rename = "mobileNumberCode")]
    mobile_number_code: String,
    #[serde(rename = "mobileRoutingCode")]
    mobile_routing_code: String,
    #[serde(rename = "carrierIdentificationCode")]
    carrier_identification_code: String,
    #[serde(rename = "lineType")]
    line_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteDetail {
    number: String,
    ported: i64,
}
