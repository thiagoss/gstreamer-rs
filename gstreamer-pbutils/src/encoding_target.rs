use ffi;
use gst;

use std::error;
use std::fmt;

use glib::object::IsA;
use glib::translate::*;

use auto::EncodingContainerProfileExt;
use auto::EncodingProfile;
use auto::EncodingTarget;
use encoding_profile::EncodingContainerProfileBuilder;
use encoding_profile::EncodingProfileBuilder;

impl EncodingTarget {
    fn new(
        name: &str,
        category: &str,
        description: &str,
        profiles: &[EncodingProfile],
    ) -> EncodingTarget {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_encoding_target_new(
                name.to_glib_none().0,
                category.to_glib_none().0,
                description.to_glib_none().0,
                profiles.to_glib_none().0,
            ))
        }
    }
}

pub struct EncodingTargetBuilder<'a> {
    name: Option<&'a str>,
    category: Option<&'a str>,
    description: Option<&'a str>,

    /*
     * Not proud of this.
     * Couldn't figure out how to store a IsA<EncodingProfile> to be used
     * only when building the final profile object. So I'm using a real
     * EncodingContainerProfile object just as a means to store it under
     * the ffi APIs. This preserves the public APIs of this builder as
     * they should be at the end.
     */
    helper_profile: EncodingContainerProfileBuilder<'a>,
}

impl<'a> EncodingTargetBuilder<'a> {
    pub fn new() -> EncodingTargetBuilder<'a> {
        EncodingTargetBuilder {
            name: None,
            category: None,
            description: None,
            helper_profile: EncodingContainerProfileBuilder::new(),
        }
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn category(mut self, category: &'a str) -> Self {
        self.category = Some(category);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn add_profile<P: IsA<EncodingProfile>>(mut self, profile: &P) -> Self {
        self.helper_profile = self.helper_profile.add_profile(profile);
        self
    }

    pub fn build(self) -> Result<EncodingTarget, EncodingTargetBuilderError> {
        if !self.has_required_fields() {
            return Err(EncodingTargetBuilderError);
        }
        let profile = self
            .helper_profile
            .name("helper")
            .format(&gst::Caps::new_any())
            .build()
            .unwrap();
        let encoding_profiles = profile.get_profiles();
        Ok(EncodingTarget::new(
            self.name.unwrap(),
            self.category.unwrap(),
            self.description.unwrap(),
            &encoding_profiles,
        ))
    }

    fn has_required_fields(&self) -> bool {
        self.name.is_some() && self.category.is_some() && self.description.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct EncodingTargetBuilderError;

impl fmt::Display for EncodingTargetBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to build encoding target")
    }
}

impl error::Error for EncodingTargetBuilderError {
    fn description(&self) -> &str {
        "invalid parameters to build encoding target"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto::EncodingTargetExt;
    use encoding_profile::EncodingAudioProfileBuilder;
    use gst;

    const ENCODING_TARGET_NAME: &'static str = "encoding-target";
    const ENCODING_TARGET_CATEGORY: &'static str = "encoding-cat";
    const ENCODING_TARGET_DESCRIPTION: &'static str = "encoding-dec";

    #[test]
    fn test_encoding_target_builder() {
        gst::init().unwrap();

        let caps = gst::Caps::new_simple("audio/x-raw", &[]);
        let audio_profile = EncodingAudioProfileBuilder::new()
            .name("audio-profile")
            .format(&caps)
            .build()
            .unwrap();

        let encoding_target = EncodingTargetBuilder::new()
            .name(ENCODING_TARGET_NAME)
            .category(ENCODING_TARGET_CATEGORY)
            .description(ENCODING_TARGET_DESCRIPTION)
            .add_profile(&audio_profile)
            .build()
            .unwrap();

        assert_eq!(encoding_target.get_name(), ENCODING_TARGET_NAME);
        assert_eq!(encoding_target.get_category(), ENCODING_TARGET_CATEGORY);
        assert_eq!(
            encoding_target.get_description(),
            ENCODING_TARGET_DESCRIPTION
        );
        assert_eq!(encoding_target.get_profiles().len(), 1);
        assert_eq!(encoding_target.get_profiles()[0], audio_profile);
    }
}
