use crate::utils::api_spec::NoResponse;
use crate::utils::ApiRequest;
use crate::utils::ApiRequestSpec;
use reqwest::Method;

pub struct Pause;
impl ApiRequestSpec for Pause {
    type Resposne = NoResponse;

    fn request(&self) -> ApiRequest {
        ApiRequest::basic(Method::PUT, "me/player/pause")
    }
}

pub struct Resume;
impl ApiRequestSpec for Resume {
    type Resposne = NoResponse;

    fn request(&self) -> ApiRequest {
        ApiRequest::basic(Method::PUT, "me/player/resume")
    }
}

pub struct GotoNextTrack;
impl ApiRequestSpec for GotoNextTrack {
    type Resposne = NoResponse;

    fn request(&self) -> ApiRequest {
        ApiRequest::basic(Method::POST, "me/player/next")
    }
}

pub struct GotoPrevTrack;
impl ApiRequestSpec for GotoPrevTrack {
    type Resposne = NoResponse;

    fn request(&self) -> ApiRequest {
        ApiRequest::basic(Method::POST, "me/player/previous")
    }
}
