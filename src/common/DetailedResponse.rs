use actix_web::body::BodySize;
use serde;
use std::fmt;

use super::StructSize::StructSize;

#[derive(serde::Serialize)]
pub struct DetailedResponse<T> {
    pub data: Option<T>,
    pub success: bool,
    pub message: String,
}

impl<T> fmt::Debug for DetailedResponse<T>
    where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DetailedResponse")
            .field("Data", &self.data)
            .field("Success", &self.success)
            .field("Message", &self.message)
            .finish()
    }
}

type Size = usize;

// impl<T> actix_web::body::MessageBody for DetailedResponse<T>
//     where T: fmt::Display 
// {
//     type Error = crate::StdErr;
//     fn size(&self) -> BodySize {
//         BodySize::Sized((self.get_size()) as u64);
//     }

//     fn poll_next(
//         self: std::pin::Pin<&mut Self>, _: &mut std::task::Context<'_>
//     ) -> std::task::Poll<
//         std::option::Option<
//             std::result::Result<
//                 actix_web::web::Bytes,
//                 <Self as actix_web::body::MessageBody>::Error
//         >>> 
//     { 
//         todo!()
//     }
// }

impl<T> DetailedResponse::<T> where T: {
    pub fn new() -> DetailedResponse<T> {
        DetailedResponse {
            data: None,
            success: false,
            message: "".to_string(),
        }
    }
}

// impl<T> StructSize for DetailedResponse<T>
//     where T StructSize:
// {
//     fn get_size(&self) -> u64 {
//         return &self.data.get_size() + &self.success + &self.message.len();
//     }
// }


