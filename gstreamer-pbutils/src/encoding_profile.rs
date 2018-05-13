
use gst;

use std::error;
use std::fmt;

use auto::EncodingProfile;
use auto::EncodingAudioProfile;


#[derive(Debug, Clone)]
pub struct EncodingProfileBuilderError;

impl fmt::Display for EncodingProfileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to build encoding profile")
    }
}

impl error::Error for EncodingProfileBuilderError {
    fn description(&self) -> &str {
        "invalid parameters to build encoding profile"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct EncodingProfileBuilderData<'a> {
    name: Option<& 'a str>,
    description: Option<& 'a str>,
    format: Option<& 'a gst::Caps>,
    preset: Option<& 'a str>,
    preset_name: Option<& 'a str>,
    restriction: Option<& 'a gst::Caps>,
    presence: u32,
    allow_dynamic_output: bool,
    enabled: bool
}

impl<'a> EncodingProfileBuilderData<'a> {
    fn new() -> EncodingProfileBuilderData<'a> {
        EncodingProfileBuilderData {
            name: None,
            description: None,
            format: None,
            preset: None,
            preset_name : None,
            restriction: None,
            presence: 0,
            allow_dynamic_output: true,
            enabled: true
        }
    }
}

pub trait EncodingProfileBuilder<'a>: Sized {
    fn get_encoding_profile_builder(&self) -> &'a mut EncodingProfileBuilderData<'a>;

    fn build(self) -> Result<EncodingProfile, EncodingProfileBuilderError>;

    fn format(self, format: & 'a gst::Caps) -> Self {
        let builder = self.get_encoding_profile_builder();
        builder.format = Some(format);
        self
    }
}

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
    base : EncodingProfileBuilderData<'a>
}

impl<'a> EncodingAudioProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingAudioProfileBuilder {
            base: EncodingProfileBuilderData::new(),
        }
    }
}

impl<'a> EncodingProfileBuilder<'a> for EncodingAudioProfileBuilder<'a> {

    fn get_encoding_profile_builder(&self) -> &'a mut EncodingProfileBuilderData<'a> {
        & self.base
    }

    fn build(self) -> Result<EncodingProfile, EncodingProfileBuilderError> {
        if self.format.is_none() {
            return Err(EncodingAudioProfileBuilderError);
        }

        let profile = EncodingAudioProfile::new(
            self.format.unwrap(), self.preset, self.restriction, self.presence);

        Ok(profile)
    }
}
