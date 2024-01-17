use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    ecs::reflect::AppTypeRegistry,
    ecs::world::{FromWorld, World},
    reflect::serde::{
        TypeRegistrationDeserializer, TypedReflectDeserializer, UntypedReflectDeserializer,
    },
    reflect::{Reflect, TypePath, TypeRegistry, TypeRegistryArc},
    utils::{
        thiserror::{self, Error},
        BoxedFuture, HashSet,
    },
};
use serde::de::{self, DeserializeSeed};

#[derive(Asset, TypePath, Debug)]
pub struct ItemAsset {
    pub components: Vec<Box<dyn Reflect>>,
}

pub struct ItemAssetLoader {
    type_registry: TypeRegistryArc,
}

impl FromWorld for ItemAssetLoader {
    fn from_world(world: &mut World) -> Self {
        Self {
            type_registry: world.resource::<AppTypeRegistry>().0.clone(),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ItemAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for ItemAssetLoader {
    type Asset = ItemAsset;
    type Settings = ();
    type Error = ItemAssetLoaderError;

    fn extensions(&self) -> &[&str] {
        &["item.ron"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _: &'a Self::Settings,
        _: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut deserializer = ron::de::Deserializer::from_bytes(&bytes)?;
            let components = ComponentsDeserializer {
                registry: &self.type_registry.read(),
            };

            Ok(Self::Asset {
                components: components
                    .deserialize(&mut deserializer)
                    .map_err(|e| deserializer.span_error(e))?,
            })
        })
    }
}

struct ComponentsDeserializer<'a> {
    registry: &'a TypeRegistry,
}

impl<'a, 'de> de::DeserializeSeed<'de> for ComponentsDeserializer<'a> {
    type Value = Vec<Box<dyn Reflect>>;

    fn deserialize<D: serde::Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_map(self)
    }
}

impl<'a, 'de> de::Visitor<'de> for ComponentsDeserializer<'a> {
    type Value = Vec<Box<dyn Reflect>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("map of reflect types")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut added = HashSet::new();
        let mut entries = Vec::new();
        while let Some(registration) =
            map.next_key_seed(TypeRegistrationDeserializer::new(self.registry))?
        {
            if !added.insert(registration.type_id()) {
                return Err(de::Error::custom(format_args!(
                    "duplicate reflect type: `{}`",
                    registration.type_info().type_path(),
                )));
            }

            let seed = TypedReflectDeserializer::new(registration, self.registry);
            entries.push(map.next_value_seed(seed)?);
        }
        Ok(entries)
    }

    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut dynamic_properties = Vec::new();
        while let Some(entity) =
            seq.next_element_seed(UntypedReflectDeserializer::new(self.registry))?
        {
            dynamic_properties.push(entity);
        }
        Ok(dynamic_properties)
    }
}
