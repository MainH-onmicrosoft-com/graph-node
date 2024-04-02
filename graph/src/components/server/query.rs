use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;

use crate::data::query::QueryError;
use std::error::Error;
use std::fmt;

use crate::components::store::StoreError;

pub type GraphQLResponse = Response<Full<Bytes>>;
pub type GraphQLResult = Result<GraphQLResponse, GraphQLServerError>;

/// Errors that can occur while processing incoming requests.
#[derive(Debug)]
pub enum GraphQLServerError {
    ClientError(String),
    QueryError(QueryError),
    InternalError(String),
}

impl From<QueryError> for GraphQLServerError {
    fn from(e: QueryError) -> Self {
        GraphQLServerError::QueryError(e)
    }
}

impl From<StoreError> for GraphQLServerError {
    fn from(e: StoreError) -> Self {
        match e {
            StoreError::ConstraintViolation(s) => GraphQLServerError::InternalError(s),
            _ => GraphQLServerError::ClientError(e.to_string()),
        }
    }
}

impl fmt::Display for GraphQLServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GraphQLServerError::ClientError(ref s) => {
                write!(f, "GraphQL server error (client error): {}", s)
            }
            GraphQLServerError::QueryError(ref e) => {
                write!(f, "GraphQL server error (query error): {}", e)
            }
            GraphQLServerError::InternalError(ref s) => {
                write!(f, "GraphQL server error (internal error): {}", s)
            }
        }
    }
}

impl Error for GraphQLServerError {
    fn description(&self) -> &str {
        "Failed to process the GraphQL request"
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            GraphQLServerError::ClientError(_) => None,
            GraphQLServerError::QueryError(ref e) => Some(e),
            GraphQLServerError::InternalError(_) => None,
        }
    }
}
