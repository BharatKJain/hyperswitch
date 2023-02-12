use serde::{Deserialize, Serialize};
use crate::{core::errors,types::{self,api, storage::enums}};
use common_utils::pii::Email;


//TODO: Fill the struct with respective fields
#[derive(Debug, Serialize,Deserialize)]
pub struct PaymentOptions {
    notification_url : Option<String>, 
    notification_methond : Option<String>,
    redirect_url: Option<String>,
    cancel_url : Option<String>,
    close_window: bool,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
     locale: Option<String>,
     ip_address: Option<String>,
     forward_ip: Option<String>,
     first_name: Option<String>,
     last_name: Option<String>,
     gender: Option<String>,
     birthday: Option<String>,
     address1: Option<String>,
     address2: Option<String>,
     house_number: Option<String>,
     zip_code: Option<String>,
     city: Option<String>,
     state: Option<String>,
     country: Option<String>,
     phone: Option<String>,
     email: Option<masking::Secret<String, Email>>, 
     user_agent: Option<String>,
     referrer: Option<String>,
     reference: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MultisafepayPaymentsRequest {
    #[serde(rename = "type")]
    _type :Option<String>,
    gateway: Option<String>,
    order_id: Option<String>,
    currency: Option<String>,
    amount:i64,
    description: Option<String>,    
    payment_options: PaymentOptions,
    customer:Customer,
}


impl TryFrom<&types::PaymentsAuthorizeRouterData> for MultisafepayPaymentsRequest  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_item: &types::PaymentsAuthorizeRouterData) -> Result<Self,Self::Error> {
        todo!()
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct MultisafepayAuthType {
    pub(super) api_key: String
}

impl TryFrom<&types::ConnectorAuthType> for MultisafepayAuthType  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        todo!()
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MultisafepayPaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<MultisafepayPaymentStatus> for enums::AttemptStatus {
    fn from(item: MultisafepayPaymentStatus) -> Self {
        match item {
            MultisafepayPaymentStatus::Succeeded => Self::Charged,
            MultisafepayPaymentStatus::Failed => Self::Failure,
            MultisafepayPaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayPaymentsResponse {
    status: MultisafepayPaymentStatus,
    id: String,
}

impl<F,T> TryFrom<types::ResponseRouterData<F, MultisafepayPaymentsResponse, T, types::PaymentsResponseData>> for types::RouterData<F, T, types::PaymentsResponseData> {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: types::ResponseRouterData<F, MultisafepayPaymentsResponse, T, types::PaymentsResponseData>) -> Result<Self,Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.status),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.id),
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct MultisafepayRefundRequest {}

impl<F> TryFrom<&types::RefundsRouterData<F>> for MultisafepayRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::RefundsRouterData<F>) -> Result<Self,Self::Error> {
       todo!()
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>> for types::RefundsRouterData<api::RSync>
{
     type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: types::RefundsResponseRouterData<api::RSync, RefundResponse>) -> Result<Self,Self::Error> {
         todo!()
     }
 }

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayErrorResponse {}
