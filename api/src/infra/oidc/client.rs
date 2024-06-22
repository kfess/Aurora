use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::{async_http_client, http_client},
    AccessToken, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, RedirectUrl, Scope,
};

use super::providers::AuthProvider;

struct OidcClient {
    clients: Arc<HashMap<AuthProvider, CoreClient>>,
}

#[trait_variant::make]
pub trait OidcClientTrait {
    async fn get_auth_url(&self, provider: &AuthProvider) -> Result<String>;
    async fn exchange_code(&self, provider: &AuthProvider, code: &str) -> Result<String>;
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

        let facebook_metadata = CoreProviderMetadata::discover(
            &IssuerUrl::new("https://facebook.com".to_string()).unwrap(),
            http_client,
        )
        .unwrap();

        let facebook_client = CoreClient::from_provider_metadata(
            facebook_metadata,
            ClientId::new("facebook_client_id".to_string()),
            Some(ClientSecret::new("facebook_client_secret".to_string())),
        )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080/auth/callback/facebook".to_string()).unwrap(),
        );

        let clients = Arc::new(
            [
                (AuthProvider::Google, google_client),
                (AuthProvider::Github, github_client),
                (AuthProvider::Facebook, facebook_client),
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

            AuthProvider::Facebook => {
                let client = self.clients.get(provider).unwrap();
                let (auth_url, _csrf_token, _nonce) = client
                    .authorize_url(
                        CoreAuthenticationFlow::AuthorizationCode,
                        CsrfToken::new_random,
                        Nonce::new_random,
                    )
                    .add_scope(Scope::new("email".to_string()))
                    .add_scope(Scope::new("public_profile".to_string()))
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
}
