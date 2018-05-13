
use gst;

use std::error;
use std::fmt;

use auto::EncodingAudioProfile;
use encoding_profile::EncodingProfileBuilder;
use encoding_profile::EncodingProfileBuilderData;


#[derive(Debug, Clone)]
pub struct EncodingAudioProfileBuilderError;

impl fmt::Display for EncodingAudioProfileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to build encoding audio profile")
    }
}

impl error::Error for EncodingAudioProfileBuilderError {
    fn description(&self) -> &str {
        "invalid parameters to build encoding audio profile"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct EncodingAudioProfileBuilder<'a> {
    base : EncodingProfileBuilderData
}

impl EncodingAudioProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingAudioProfileBuilder {
            base: EncodingProfileBuilderData::new(),
        }
    }
}

impl<'a> EncodingProfileBuilder for EncodingAudioProfileBuilder<'a> {

    fn get_encoding_profile_builder(&self) -> &'a mut EncodingProfileBuilderData<'a> {
        &self.base
    }

    fn build(self) -> Result<EncodingAudioProfile, EncodingAudioProfileBuilderError> {
        if self.format.is_none() {
            return Err(EncodingAudioProfileBuilderError);
        }

        let profile = EncodingAudioProfile::new(
            self.format.unwrap(), self.preset, self.restriction, self.presence);

        Ok(profile)
    }
}
