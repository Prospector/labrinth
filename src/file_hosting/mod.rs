use thiserror::Error;

mod authorization;
mod delete;
mod upload;

pub use authorization::authorize_account;
pub use authorization::get_upload_url;
pub use authorization::AuthorizationData;
pub use authorization::AuthorizationPermissions;
pub use authorization::UploadUrlData;

pub use upload::upload_file;
pub use upload::UploadFileData;

pub use delete::delete_file_version;
pub use delete::DeleteFileData;

#[derive(Error, Debug)]
pub enum FileHostingError {
    #[error("Error while accessing the data from backblaze")]
    BackblazeError(#[from] reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_authorization() {
        let authorization_data = authorize_account(
            dotenv::var("BACKBLAZE_KEY_ID").unwrap(),
            dotenv::var("BACKBLAZE_KEY").unwrap(),
        )
        .await
        .unwrap();

        get_upload_url(
            authorization_data,
            dotenv::var("BACKBLAZE_BUCKET_ID").unwrap(),
        )
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn test_file_management() {
        let authorization_data = authorize_account(
            dotenv::var("BACKBLAZE_KEY_ID").unwrap(),
            dotenv::var("BACKBLAZE_KEY").unwrap(),
        )
        .await
        .unwrap();
        let upload_url_data = get_upload_url(
            authorization_data.clone(),
            dotenv::var("BACKBLAZE_BUCKET_ID").unwrap(),
        )
        .await
        .unwrap();
        let upload_data = upload_file(
            upload_url_data,
            "text/plain".to_string(),
            "test.txt".to_string(),
            "test file".to_string().into_bytes(),
        )
        .await
        .unwrap();

        delete_file_version(
            authorization_data,
            upload_data.file_id,
            upload_data.file_name,
        )
        .await
        .unwrap();
    }
}
