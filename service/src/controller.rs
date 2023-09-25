#[derive(Default, Debug, serde::Serialize)]
pub struct GetProfileResponse {}

#[derive(thiserror::Error, Debug)]
pub enum GetProfileError {}

pub fn get_user_profile() -> Result<GetProfileResponse, GetProfileError> {
    Ok(GetProfileResponse::default())
}
