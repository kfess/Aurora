use super::external::{github::GithubUser, google::GoogleUser};
use crate::domain::vo::providers::AuthProvider;
use anyhow::{Context, Result};
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::{async_http_client, http_client},
    AccessToken, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, RedirectUrl, Scope,
};
use std::{collections::HashMap, sync::Arc};

pub struct OidcClient {
    clients: Arc<HashMap<AuthProvider, CoreClient>>,
}

const GOOGLE_API_URL: &str = "";
const GITHUB_API_URL: &str = "https://api.github.com/user";

#[trait_variant::make]
pub trait OidcClientTrait {
    async fn get_auth_url(&self, provider: &AuthProvider) -> Result<String>;
    async fn exchange_code(&self, provider: &AuthProvider, code: &str) -> Result<String>;
    async fn get_user_info(
        &self,
        provider: &AuthProvider,
        access_token: &str,
    ) -> Result<(String, Option<String>)>;
}

impl OidcClient {
    pub async fn new() -> Result<Self> {
        let google_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new("https://accounts.google.com".to_string())?,
            async_http_client,
        )
        .await?;

        let google_client = CoreClient::from_provider_metadata(
            google_metadata,
            ClientId::new("google_client_id".to_string()),
            Some(ClientSecret::new("google_client_secret".to_string())),
        )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080/auth/callback/google".to_string()).unwrap(),
        );

        let github_metadata = CoreProviderMetadata::discover(
            &IssuerUrl::new("https://github.com".to_string()).unwrap(),
            http_client,
        )
        .unwrap();

        let github_client = CoreClient::from_provider_metadata(
            github_metadata,
            ClientId::new("github_client_id".to_string()),
            Some(ClientSecret::new("github_client_secret".to_string())),
        )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080/auth/callback/github".to_string()).unwrap(),
        );

        let clients = Arc::new(
            [
                (AuthProvider::Google, google_client),
                (AuthProvider::Github, github_client),
            ]
            .iter()
            .cloned()
            .collect(),
        );

        Ok(Self { clients })
    }
}

impl OidcClientTrait for OidcClient {
    async fn get_auth_url(&self, provider: &AuthProvider) -> Result<String> {
        match provider {
            AuthProvider::Google => {
                let client = self.clients.get(provider).unwrap();
                let (auth_url, _csrf_token, _nonce) = client
                    .authorize_url(
                        CoreAuthenticationFlow::AuthorizationCode,
                        CsrfToken::new_random,
                        Nonce::new_random,
                    )
                    .add_scope(Scope::new("openid".to_string()))
                    .add_scope(Scope::new("email".to_string()))
                    .add_scope(Scope::new("profile".to_string()))
                    .url();

                Ok(auth_url.to_string())
            }

            AuthProvider::Github => {
                let client = self.clients.get(provider).unwrap();
                let (auth_url, _csrf_token, _nonce) = client
                    .authorize_url(
                        CoreAuthenticationFlow::AuthorizationCode,
                        CsrfToken::new_random,
                        Nonce::new_random,
                    )
                    .add_scope(Scope::new("read:user".to_string()))
                    .url();

                Ok(auth_url.to_string())
            }
        }
    }
    async fn exchange_code(&self, provider: &AuthProvider, code: &str) -> Result<String> {
        if let Some(client) = self.clients.get(provider) {
            let token_response = client
                .exchange_code(AuthorizationCode::new(code.to_string()))
                .request_async(async_http_client)
                .await
                .map_err(|e| anyhow::anyhow!(e))?;

            let access_token: AccessToken = token_response.access_token().to_owned();

            Ok(access_token.secret().to_string())
        } else {
            Err(anyhow::anyhow!("Invalid provider"))
        }
    }

    async fn get_user_info(
        &self,
        provider: &AuthProvider,
        access_token: &str,
    ) -> Result<(String, Option<String>)> {
        match provider {
            AuthProvider::Google => {
                let client = Arc::new(reqwest::Client::new());

                let user_info = client
                    .post(GOOGLE_API_URL)
                    .header("Authorization", format!("Bearer {access_token}"))
                    .send()
                    .await
                    .with_context(|| "Failed to fetch user info")
                    .unwrap()
                    .json::<GoogleUser>()
                    .await
                    .with_context(|| "Failed to parse json from user info")
                    .unwrap();

                Ok((user_info.sub.to_string(), user_info.email))
            }
            AuthProvider::Github => {
                let client = Arc::new(reqwest::Client::new());
                let user_info = client
                    .post(GITHUB_API_URL)
                    .header("Authorization", format!("Bearer {access_token}"))
                    .send()
                    .await
                    .with_context(|| "Failed to fetch user info")
                    .unwrap()
                    .json::<GithubUser>()
                    .await
                    .with_context(|| "Failed to parse json from user info")
                    .unwrap();

                Ok((user_info.id.to_string(), Some(user_info.id.to_string())))
            }
        }
    }
}
