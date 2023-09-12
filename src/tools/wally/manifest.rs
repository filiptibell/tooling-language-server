use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use tracing::error;

use super::dependency_spec::*;
use crate::clients::wally::models::*;
use crate::util::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestDependency {
    realm: MetadataRealm,
    key: TomlString,
    spec: TomlString,
}

impl ManifestDependency {
    fn from_toml_values(key: &TomlString, value: &TomlValue) -> Option<Self> {
        value.as_string().map(|s| Self {
            realm: MetadataRealm::Shared,
            key: key.clone(),
            spec: s.clone(),
        })
    }

    pub fn spec(&self) -> Result<DependencySpec, DependencySpecError> {
        self.spec.value().parse::<DependencySpec>()
    }

    pub fn span(&self) -> Range<usize> {
        self.spec.span()
    }

    pub fn source(&self) -> &str {
        self.spec.source()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Manifest {
    pub metadata: Option<Metadata>,
    pub dependencies: HashMap<String, ManifestDependency>,
    pub dev_dependencies: HashMap<String, ManifestDependency>,
    pub server_dependencies: HashMap<String, ManifestDependency>,
}

impl Manifest {
    fn from_toml_value(value: &TomlValue) -> Option<Self> {
        let tab = match value.as_table() {
            None => return None,
            Some(t) => t,
        };

        let mut manifest = Manifest::default();

        let fill =
            |map_key: &str, map: &mut HashMap<String, ManifestDependency>, realm: MetadataRealm| {
                if let Some((_, deps)) = tab.find(map_key) {
                    if let Some(deps_table) = deps.as_table() {
                        for (k, v) in deps_table.as_ref().iter() {
                            if let Some(mut tool) = ManifestDependency::from_toml_values(k, v) {
                                tool.realm = realm;
                                map.insert(k.value().to_string(), tool);
                            }
                        }
                    }
                }
            };

        fill(
            "dependencies",
            &mut manifest.dependencies,
            MetadataRealm::Shared,
        );
        fill(
            "dev-dependencies",
            &mut manifest.dev_dependencies,
            MetadataRealm::Dev,
        );
        fill(
            "server-dependencies",
            &mut manifest.server_dependencies,
            MetadataRealm::Server,
        );

        Some(manifest)
    }

    pub fn parse(source: impl AsRef<str>) -> Result<Self, TomlError> {
        let mut manifest = match TomlValue::new(source.as_ref()) {
            Ok(value) => Ok(Self::from_toml_value(&value).expect("Toml root should be a table")),
            Err(e) => {
                error!("failed to deserialize tools manifest - {e}");
                Err(e)
            }
        }?;

        if let Ok(metadata) = toml::from_str(source.as_ref()) {
            manifest.metadata = metadata;
        }

        Ok(manifest)
    }
}

impl FromStr for Manifest {
    type Err = TomlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}
