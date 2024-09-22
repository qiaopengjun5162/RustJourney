use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::SystemTime,
};

use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    middleware::{from_fn, AddExtension},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Extension, Json, RequestPartsExt, Router, Server, TypedHeader,
};
use jsonwebtoken as jwt;
use jwt::Validation;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower::ServiceBuilder;

const SECRET: &[u8] = b"deadbeef";

// 定义一个 Todo 结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    name: String,
    exp: usize,
}

#[derive(Debug, Default, Clone)]
struct TodoStore {
    items: Arc<RwLock<Vec<Todo>>>,
}

#[tokio::main]
async fn main() {
    let store = TodoStore::default();
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/todos", get(todos_handler).post(create_todo_handler))
        .route("/login", post(login_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn todos_handler() -> Json<Vec<Todo>> {
    Json(vec![
        Todo {
            id: 1,
            title: "Buy milk".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Buy eggs".to_string(),
            completed: false,
        },
    ])
}

// "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IlhpYW8gUWlhbyJ9.7PdpzWjZLN4KKNLoM07nfnhKnYdrc0IjumKcOUREXzI"
async fn create_todo_handler(
    claims: Claims,
    Json(_todo): Json<CreateTodo>,
    // Extension(store): Extension<TodoStore>,
) -> StatusCode {
    println!("{:?}", claims);
    // println!("{:?}", store);
    StatusCode::CREATED
}

async fn login_handler(Json(login): Json<LoginRequest>) -> Json<LoginResponse> {
    // skip login info validation
    println!("{:?}", login);

    let claims = Claims {
        id: 1,
        name: "Xiao Qiao".to_string(),
        exp: get_epoch() + 14 * 24 * 60 * 60,
    };
    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}

// #[async_trait]
// impl<S, B> FromRequest<S, B> for Claims
// where
//     // these bounds are required by `async_trait`
//     B: Send + 'static,
//     S: Send + Sync,
//     // {
//     //     type Rejection = http::StatusCode;

//     //     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//     //         // ...
//     //         let TypedHeader(Authorization(bearer)) =
//     //             TypedHeader::<Authorization<Bearer>>::from_request(req, state)
//     //                 .await
//     //                 .map_err(|_| http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED)?;

//     //         let key = jwt::DecodingKey::from_secret(SECRET);
//     //         let claims = jwt::decode::<Claims>(bearer.token(), &key, &jwt::Validation::default())
//     //             .map_err(|_| http::StatusCode::UNAUTHORIZED)?;
//     //         Ok(claims.claims)
//     //     }
//     // }
// {
//     type Rejection = HttpError;

//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         // ...
//         let TypedHeader(Authorization(bearer)) =
//             TypedHeader::<Authorization<Bearer>>::from_request(req, state)
//                 .await
//                 .map_err(|_| HttpError::Auth)?;

//         let key = jwt::DecodingKey::from_secret(SECRET);
//         let token = jwt::decode::<Claims>(bearer.token(), &key, &Validation::default())
//             .map_err(|_| HttpError::Auth)?;
//         Ok(token.claims)
//     }
// }

// #[derive(Debug)]
// enum HttpError {
//     Auth,
//     Internal,
//     NotFound,
//     InternalServerError,
// }

// impl IntoResponse for HttpError {
//     fn into_response(self) -> axum::response::Response {
//         let (code, msg) = match self {
//             HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
//             HttpError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
//             HttpError::InternalServerError => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
//             }
//             HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
//         };
//         (code, msg).into_response()
//     }
// }

fn get_epoch() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        let key = jwt::DecodingKey::from_secret(SECRET);
        // Decode the user data
        let token_data = jwt::decode::<Claims>(bearer.token(), &key, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            // AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            // AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            // AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug)]
enum AuthError {
    // WrongCredentials,
    // MissingCredentials,
    // TokenCreation,
    InvalidToken,
}
