use crate::Error;
use ruma::api::{MatrixVersion, OutgoingRequest, SendAccessToken};
use ruma::client::{
    HttpClient, HttpClientExt, ResponseError, ResponseResult,
};
use ruma::UserId;

pub type RumaHttpClient = ruma::client::http_client::Reqwest;

fn add_user_id_to_query<C: HttpClient + ?Sized, R: OutgoingRequest>(
    user_id: &UserId,
) -> impl FnOnce(&mut http::Request<C::RequestBody>) -> Result<(), ResponseError<C, R>> + '_ {
    use assign::assign;
    use http::uri::Uri;
    use ruma::serde::urlencoded;

    move |http_request| {
        let extra_params = urlencoded::to_string([("user_id", user_id)]).unwrap();
        let uri = http_request.uri_mut();
        let new_path_and_query = match uri.query() {
            Some(params) => format!("{}?{params}&{extra_params}", uri.path()),
            None => format!("{}?{extra_params}", uri.path()),
        };
        *uri = Uri::from_parts(assign!(uri.clone().into_parts(), {
            path_and_query: Some(new_path_and_query.parse()?),
        }))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Client {
    homeserver_url: String,
    access_token: String,
    supported_matrix_versions: Vec<MatrixVersion>,
    http: RumaHttpClient,
}

impl Client {
    pub async fn new(homeserver_url: String, access_token: String) -> Result<Self, Error> {
        let http = RumaHttpClient::default();

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

    pub async fn send_request<R: OutgoingRequest>(
        &self,
        request: R,
    ) -> ResponseResult<RumaHttpClient, R> {
        let send_access_token = SendAccessToken::IfRequired(&self.access_token);

        self.http
            .send_matrix_request(
                &self.homeserver_url,
                send_access_token,
                &self.supported_matrix_versions,
                request,
            )
            .await
    }

    pub async fn send_request_force_auth<R: OutgoingRequest>(
        &self,
        request: R,
    ) -> ResponseResult<RumaHttpClient, R> {
        let send_access_token = SendAccessToken::Always(&self.access_token);

        self.http
            .send_matrix_request(
                &self.homeserver_url,
                send_access_token,
                &self.supported_matrix_versions,
                request,
            )
            .await
    }

    pub async fn send_request_as<R: OutgoingRequest>(
        &self,
        user_id: &UserId,
        request: R,
    ) -> ResponseResult<RumaHttpClient, R> {
        let send_access_token = SendAccessToken::IfRequired(&self.access_token);

        self.http
            .send_customized_matrix_request(
                &self.homeserver_url,
                send_access_token,
                &self.supported_matrix_versions,
                request,
                add_user_id_to_query::<RumaHttpClient, R>(user_id),
            )
            .await
    }

    pub async fn user(
        &self,
        user_id: &UserId,
    ) -> Result<UserClient, Error> {
        let client = matrix_sdk::Client::builder()
            .homeserver_url(self.homeserver_url.clone())
            .appservice_mode()
            .build()
            .await?;

        Ok(UserClient(client))
    }
}

pub struct UserClient(matrix_sdk::Client);
