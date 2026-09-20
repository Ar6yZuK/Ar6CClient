// pub mod client;

use progenitor::progenitor_client;
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
use reqwest::Response;
use serde::{Deserialize, de::DeserializeOwned};
#[doc = r" Types used as operation parameters and responses."]
#[allow(clippy::all)]
pub mod types {
    #[doc = "`OperationDto`"]
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OperationDto {
        pub description: ::std::string::String,
        pub name: ::std::string::String,
        pub parameter: ParameterDto,
        #[serde(rename = "return")]
        pub return_: ReturnDto,
    }
    #[doc = "`OperationType`"]
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct OperationType(pub i64);

    impl ::std::ops::Deref for OperationType {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<OperationType> for i64 {
        fn from(value: OperationType) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<i64> for OperationType {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }
    impl ::std::fmt::Display for OperationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl ::std::str::FromStr for OperationType {
        type Err = <i64 as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl ::std::convert::TryFrom<&str> for OperationType {
        type Error = <i64 as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<String> for OperationType {
        type Error = <i64 as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }
    #[doc = "`OperationValue`"]
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct OperationValue(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);

    impl ::std::ops::Deref for OperationValue {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<OperationValue>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: OperationValue) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OperationValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    #[doc = "`ParameterDto`"]
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ParameterDto {
        pub description: ::std::string::String,
        pub name: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    #[doc = "`ReturnDto`"]
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ReturnDto {
        pub description: ::std::string::String,
        pub name: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    #[doc = " Error types."]
    pub mod error {
        #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
        pub struct ConversionError(::std::borrow::Cow<'static, str>);

        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Client for Ar6CServer | v1\n\nVersion: 1.0.0"]
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    #[doc = r" Create a new client."]
    #[doc = r""]
    #[doc = r" `baseurl` is the base URL provided to the internal"]
    #[doc = r" `reqwest::Client`, and should include a scheme and hostname,"]
    #[doc = r" as well as port and a path stem if applicable."]
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    #[doc = r" Construct a new client with an existing `reqwest::Client`,"]
    #[doc = r" allowing more control over its configuration."]
    #[doc = r""]
    #[doc = r" `baseurl` is the base URL provided to the internal"]
    #[doc = r" `reqwest::Client`, and should include a scheme and hostname,"]
    #[doc = r" as well as port and a path stem if applicable."]
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0.0"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}

#[allow(clippy::all)]
impl Client {
    #[doc = "Sends a `GET` request to `/AddAllowed/{ip}`\n\n"]
    pub async fn add_allowed_ip<'a>(&'a self, ip: &'a str) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/AddAllowed/{}",
            self.baseurl,
            encode_path(&ip.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "add_allowed_ip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    #[doc = "Sends a `GET` request to `/RemoveAllowed/{ip}`\n\n"]
    pub async fn remove_allowed_ip<'a>(
        &'a self,
        ip: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/RemoveAllowed/{}",
            self.baseurl,
            encode_path(&ip.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "remove_allowed_ip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    #[doc = "Sends a `GET` request to `/getFile/{guid}`\n\n"]
    pub async fn get_server_file<'a>(
        &'a self,
        guid: &'a ::uuid::Uuid,
    ) -> Result<Response, Error<()>> {
        let url = format!(
            "{}/getFile/{}",
            self.baseurl,
            encode_path(&guid.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "get_server_file",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;

        match response.status().as_u16() {
            200u16 => Ok(response),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    #[doc = "Sends a `POST` request to `/{operation}`\n\n"]
    pub async fn invoke_operation_stream<'a>(
        &'a self,
        operation: &'a types::OperationType,
        body: Option<&'a types::OperationValue>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let response = self.invoke_operation(operation, body).await?;

        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    #[doc = "Sends a `POST` request to `/{operation}`\n\n"]
    pub async fn invoke_operation<'a>(
        &'a self,
        operation: &'a types::OperationType,
        body: Option<&'a types::OperationValue>,
    ) -> Result<Response, Error<()>> {
        let url = format!("{}/{}", self.baseurl, encode_path(&operation.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "invoke_operation",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;

        match response.status().as_u16() {
            200u16 => Ok(response),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/{operation}`\n\n"]
    pub async fn invoke_operation_for<'a, TResult>(
        &'a self,
        operation: &'a types::OperationType,
        body: Option<&'a types::OperationValue>,
    ) -> Result<ResponseValue<TResult>, Error<()>>
    where
        TResult: DeserializeOwned,
    {
        let response = self.invoke_operation(operation, body).await?;

        match response.status().as_u16() {
            200u16 => {
                let status = response.status();
                let headers = response.headers().clone();
                Ok(ResponseValue::new(
                    response.json::<TResult>().await?,
                    status,
                    headers,
                ))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/getNames`\n\n"]
    pub async fn get_names<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<::std::string::String>>, Error<()>> {
        let url = format!("{}/getNames", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_names",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    #[doc = "Sends a `POST` request to `/V2/{operationName}`\n\n"]
    pub async fn invoke_operation_v2<'a>(
        &'a self,
        operation_name: &'a str,
        body: &'a types::OperationValue,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/V2/{}",
            self.baseurl,
            encode_path(&operation_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "invoke_operation_v2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    #[doc = "Sends a `POST` request to `/V2/addOperation`\n\n"]
    pub async fn add_operation_v2<'a>(
        &'a self,
        body: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/V2/addOperation", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "add_operation_v2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    #[doc = "Sends a `POST` request to `/V2/unloadOperation`\n\n"]
    pub async fn unload_operation_v2<'a>(
        &'a self,
        body: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/V2/unloadOperation", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "unload_operation_v2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    #[doc = "Sends a `GET` request to `/V2/getNames`\n\n"]
    pub async fn get_names_v2<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::OperationDto>>, Error<()>> {
        let url = format!("{}/V2/getNames", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_names_v2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
#[doc = r" Items consumers will typically use such as the Client."]
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
const _: &str = include_str!("C:\\rusts\\Ar6CClient\\Ar6CClient\\Ar6CClient\\openapi_spec.json");
