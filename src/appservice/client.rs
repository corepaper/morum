use crate::Error;
use ruma::api::{MatrixVersion, SendAccessToken, OutgoingRequest};
use ruma::client::{ResponseResult, DefaultConstructibleHttpClient, HttpClientExt, http_client::HyperNativeTls};

pub struct Client {
    homeserver_url: String,
    access_token: String,
    supported_matrix_versions: Vec<MatrixVersion>,
    http: HyperNativeTls,
}

impl Client {
    pub async fn new(homeserver_url: String, access_token: String) -> Result<Self, Error> {
        let http = HyperNativeTls::default();

        let supported_matrix_versions = http
            .send_matrix_request(
                &homeserver_url,
                SendAccessToken::None,
                &[MatrixVersion::V1_0],
                ruma::api::client::discovery::get_supported_versions::Request::new(),
            )
            .await?
            .known_versions()
            .collect();

        Ok(Client {
            homeserver_url,
            access_token,
            supported_matrix_versions,
            http,
        })
    }

    pub async fn send_request<R: OutgoingRequest>(&self, request: R) -> ResponseResult<HyperNativeTls, R> {
        let send_access_token = SendAccessToken::IfRequired(&self.access_token);

        self.http.send_matrix_request(
            &self.homeserver_url,
            send_access_token,
            &self.supported_matrix_versions,
            request
        ).await
    }

    pub async fn send_request_force_auth<R: OutgoingRequest>(&self, request: R) -> ResponseResult<HyperNativeTls, R> {
        let send_access_token = SendAccessToken::Always(&self.access_token);

        self.http.send_matrix_request(
            &self.homeserver_url,
            send_access_token,
            &self.supported_matrix_versions,
            request
        ).await
    }
}
