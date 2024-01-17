use std::sync::Arc;
use async_trait::async_trait;
use bcrypt::{hash, verify};
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use uuid::Uuid;

use crate::account::repository::account_repository::AccountRepository;
use crate::account::repository::account_repository_impl::AccountRepositoryImpl;

use crate::account::service::account_service::AccountService;

use crate::account::service::request::account_login_request::AccountLoginRequest;
use crate::account::service::request::account_register_request::AccountRegisterRequest;

use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::account::service::response::account_register_response::AccountRegisterResponse;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;

use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;


pub struct AccountServiceImpl {
    repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl AccountServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {

        AccountServiceImpl {
            repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountServiceImpl::new(
                            AccountRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    async fn account_register(&self, account_register_request: AccountRegisterRequest) -> AccountRegisterResponse {
        println!("AccountServiceImpl: account_register()");

        let account_repository = self.repository.lock().await;
        let result = account_repository.save(account_register_request.to_account().unwrap()).await;

        if result.is_ok() {
            AccountRegisterResponse::new(true)
        } else {
            eprintln!("계정 생성 중 에러 발생");
            AccountRegisterResponse::new(false)
        }
    }

    async fn account_login(&self, account_login_request: AccountLoginRequest) -> AccountLoginResponse {
        println!("AccountServiceImpl: account_login()");

        let account_repository = self.repository.lock().await;
        let account = account_login_request.to_account().unwrap();
        if let Some(found_account) = account_repository.find_by_user_id(account.user_id()).await.unwrap() {
            // 비밀번호 매칭 확인
            if verify(&account_login_request.password(), &found_account.password()).unwrap() {
                // 로그인 성공
                let redis_token = Uuid::new_v4();
                let mut redis_repository_gaurd = self.redis_in_memory_repository.lock().await;
                redis_repository_gaurd.set(&*redis_token.to_string(), &found_account.id.to_string()).await;

                return AccountLoginResponse::new(redis_token.to_string());
            } else {
                // 비밀번호 불일치 - 로그인 실패
                eprintln!("Password mismatch for user_id: {}", account.user_id());
                eprintln!("Password mismatch for password: {}", account.password());
                eprintln!("login_request password: {}", account_login_request.password());

                // Output hashed password stored in the database
                println!("Stored hashed password: {}", found_account.password());
                println!("Generated hashed password: {}", hash(&account_login_request.password(), 12).unwrap());
            }
        } else {
            // 계정이 없음 - 로그인 실패
            eprintln!("Account not found for user_id: {}", account.user_id());
        }

        // 로그인 실패 시 기본 응답 반환
        AccountLoginResponse::new("".to_string())
    }
}


