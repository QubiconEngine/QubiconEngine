#[derive(Default, Debug, Clone, Copy)]
pub struct InstanceCreateInfo {
    #[cfg(feature = "windowing")]
    pub enable_windowing: bool,

    pub app_id: AppId
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppId {
    pub app_version: Version,
    pub engine_version: Version,
    pub vulkan_version: Version,

    pub app_name: &'static str,
    pub engine_name: &'static str
}

impl AppId {
    pub fn validate(&self) {
        if self.vulkan_version.into() != 0 && self.vulkan_version < Version( ash::vk::API_VERSION_1_0 ) {
            panic!("invalid Vulkan API version: {}", self.vulkan_version);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(u32);

impl Version {
    pub const fn new(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
        Self ( ash::vk::make_api_version(variant, major, minor, patch) )
    }

    pub const fn major(&self) -> u32 {
        ash::vk::api_version_major(self.0)
    }

    pub const fn minor(&self) -> u32 {
        ash::vk::api_version_minor(self.0)
    }

    pub const fn patch(&self) -> u32 {
        ash::vk::api_version_patch(self.0)
    }

    pub const fn variant(&self) -> u32 {
        ash::vk::api_version_variant(self.0)
    }
}

impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Version")
            .field("major", &self.major())
            .field("minor", &self.minor())
            .field("patch", &self.patch())
            .field("variant", &self.variant())
            .finish()
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Variant
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl core::str::FromStr for Version {
    type Err = <u32 as core::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('.');

        // TODO: Variant
        let major = s.next().ok_or(Self::Err)?.into();
        let minor = s.next().ok_or(Self::Err)?.into();
        let patch = s.next().ok_or(Self::Err)?.into();

        Ok ( Self::new(0, major, minor, patch) )
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let result = self.variant().cmp(&other.variant())
            .then_with(|| self.variant().cmp(&other.variant()))
            .then_with(|| self.minor().cmp(&other.minor()))
            .then_with(|| self.patch().cmp(&other.patch()));

        Some( result )
    }
}

impl From<Version> for u32 {
    fn from(value: Version) -> Self {
        value.0
    }
}