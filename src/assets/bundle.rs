//! Bundle asset

use crate::assets::resource;
use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::{
        tracing::{self, instrument},
        BoxedFuture,
    },
};
use fluent::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use serde::Deserialize;
use std::{ops::Deref, path::PathBuf, str, sync::Arc};
use unic_langid::LanguageIdentifier;

#[instrument(fields(path = %load_context.path().display()), ret, skip_all)]
async fn load(data: Data, load_context: &mut LoadContext<'_>) -> Result<()> {
    let mut bundle = FluentBundle::new_concurrent(vec![data.locale.clone()]);
    let mut asset_paths = Vec::new();
    let parent = load_context.path().parent();
    for mut path in data.resources {
        if path.is_relative() {
            if let Some(parent) = parent {
                path = parent.join(path);
            }
        }
        let bytes = load_context.read_asset_bytes(&path).await?;
        let resource = resource::deserialize(&bytes)?;
        if let Err(errors) = bundle.add_resource(resource) {
            warn_span!("add_resource").in_scope(|| {
                for error in errors {
                    warn!(%error);
                }
            });
        }
        asset_paths.push(AssetPath::new(path, None));
    }
    load_context.set_default_asset(
        LoadedAsset::new(BundleAsset(Arc::new(bundle))).with_dependencies(asset_paths),
    );
    Ok(())
}

/// [`FluentBundle`](fluent::bundle::FluentBundle) wrapper
///
/// Collection of [`FluentResource`]s for a single locale
#[derive(Clone, TypeUuid)]
#[uuid = "929113bb-9187-44c3-87be-6027fc3b7ac5"]
pub struct BundleAsset(pub(crate) Arc<FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>);

impl Deref for BundleAsset {
    type Target = FluentBundle<Arc<FluentResource>, IntlLangMemoizer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`AssetLoader`](bevy::asset::AssetLoader) implementation for [`BundleAsset`]
#[derive(Default)]
pub struct BundleAssetLoader;

impl AssetLoader for BundleAssetLoader {
    fn load<'a>(
        &self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move { load(ron::de::from_bytes(bytes)?, load_context).await })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl.ron"]
    }
}

/// Data
#[derive(Debug, Deserialize)]
struct Data {
    locale: LanguageIdentifier,
    resources: Vec<PathBuf>,
}
