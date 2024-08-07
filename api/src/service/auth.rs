use anyhow::{Context, Result};

use crate::domain::user::User;
use crate::domain::vo::providers::AuthProvider;
use crate::infra::{oidc::client::OidcClientTrait, repository::user::UserRepository};

pub struct AuthUsecase<C, R>
where
    C: OidcClientTrait,
    R: UserRepository,
{
    oidc_client: C,
    repository: R,
}

#[trait_variant::make]
pub trait Authenticate {
    async fn get_authenticate_url(&self, provider: &AuthProvider) -> Result<String>;
    async fn handle_callback(&self, provider: &AuthProvider, code: &str) -> Result<User>;
    async fn get_user_info(&self, user_id: &str) -> Result<User>;
}

impl<C, R> AuthUsecase<C, R>
where
    C: OidcClientTrait,
    R: UserRepository,
{
    pub fn new(oidc_client: C, repository: R) -> Self {
        Self {
            oidc_client,
            repository,
        }
    }
}

impl<C, R> Authenticate for AuthUsecase<C, R>
where
    C: OidcClientTrait,
    R: UserRepository,
{
    async fn get_authenticate_url(&self, provider: &AuthProvider) -> Result<String> {
        self.oidc_client
            .get_auth_url(&provider)
            .await
            .with_context(|| {
                format!(
                    "Failed to get authenticate url for provider: {:?}",
                    provider
                )
            })
    }

    async fn handle_callback(&self, provider: &AuthProvider, code: &str) -> Result<User> {
        let access_token = self.oidc_client.exchange_code(provider, code).await?;

        let user_info = self
            .oidc_client
            .get_user_info(provider, &access_token)
            .await?;

        let user = self
            .repository
            .find_by_provider_user_id(provider, &user_info.0)
            .await?;

        match user {
            Some(user) => {
                // todo!("Update user info");

                Ok(user)
            }
            None => {
                todo!("Create user info");
            }
        }
    }

    async fn get_user_info(&self, user_id: &str) -> Result<User> {
        match self.repository.find_by_user_id(user_id).await {
            Ok(user) => Ok(user),
            Err(_) => todo!("Handle error"),
        }
    }
}
