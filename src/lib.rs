#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use core::fmt;
use thiserror::Error;

#[cfg(feature = "rusqlite")]
use rusqlite::types::{
    FromSql,
    FromSqlResult,
    ToSql,
    ToSqlOutput,
    ValueRef
};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod tests;

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><=========================  TRAITS  ===========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

/// Trait for entities to define which struct field holds their primary key
pub trait Keyed {
    /// The type of the Key
    type KeyType;

    /// Returns the Key for the Entity
    fn key(&self) -> Result<&Key<Self::KeyType>>;
}

/// Trait for entities that optionally have a label defined
///
/// This is needed for using EntityLabels.
pub trait Label {
    /// The type of the Label
    type LabelType;

    /// Returns the Label for the Entity
    fn label(&self) -> Result<&Self::LabelType>;
}

/// Struct that holds both key and label for convenience
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Tag {
    /// Entity key
    pub key: String,
    /// Entity label
    pub label: String,
}

/// Convenience trait for returning both key and label as a Tag
///
/// There is a blanket implementation for all entities that
/// implement both Keyed and Label
pub trait Tagged {
    /// Returns the Tag for the Entity
    fn tag(&self) -> Result<Tag>;
    /// The entity does not have a valid tag if it doesn't have a Key
    fn has_tag(&self) -> bool;
}

impl<K, T, L> Tagged for T
where
    T: Keyed<KeyType = K> + Label<LabelType = L>,
    K: fmt::Display,
    L: fmt::Display,
{
    fn tag(&self) -> Result<Tag> {
        Ok(
            Tag {
                key: self.key()?.to_string(),
                label: self.label()?.to_string(),
            }
        )
    }

    fn has_tag(&self) -> bool {
        self.key().map(|v| v.is_some()).unwrap_or(false)
    }
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><==========================  KEY  =============================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

/// A newtype for defining a Key on entities
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Key<K>(pub Option<K>);

impl<K> Key<K> {
    /// Creates a new Key from a value `K`
    pub fn new(value: K) -> Self {
        Self(Some(value))
    }

    /// Converts this Key into an Entity
    pub fn into_entity<T>(self) -> Entity<K, T> {
        Entity::Key(self)
    }

    /// Converts this Key to an Entity
    pub fn to_entity<T>(&self) -> Entity<K, T> where K: Clone {
        Entity::Key(self.clone())
    }
}

impl<K> core::ops::Deref for Key<K> {
    type Target = Option<K>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> core::ops::DerefMut for Key<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: fmt::Display> fmt::Display for Key<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{value}"),
            None => write!(f, "None"),
        }
    }
}

impl<K> From<Option<K>> for Key<K> {
    fn from(value: Option<K>) -> Self {
        Self(value)
    }
}

impl<K: Clone> From<&Option<K>> for Key<K> {
    fn from(value: &Option<K>) -> Self {
        Self(value.as_ref().cloned())
    }
}

#[cfg(feature = "rusqlite")]
impl<K: FromSql> FromSql for Key<K> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => Ok(Key(None)),
            _ => FromSql::column_result(value).map(|v| Key(Some(v))),
        }
    }
}

#[cfg(feature = "rusqlite")]
impl<K: ToSql> ToSql for Key<K> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><=========================  ENTITY  ===========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

/// Enum for defining a simple entity that will hold a Key
/// or the created/fetched data
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum Entity<K, T> {
    /// Key of the entity
    Key(Key<K>),
    /// Created/Fetched data for the entity
    Data(Box<T>),
    #[default]
    /// For when you have no data to fill or null from database
    None,
}

impl<K, T> Keyed for Entity<K, T>
where
    T: Keyed<KeyType = K>,
{
    type KeyType = K;

    fn key(&self) -> Result<&Key<Self::KeyType>> {
        match self {
            Entity::Key(key) => Ok(key),
            Entity::Data(data) => data.key(),
            Entity::None => Err(Error::EntityEmpty),
        }
    }
}

impl<K, T> Entity<K, T> {
    /// Returns the data if it exists and was fetched/created
    pub fn data(&self) -> Result<&T> {
        match self {
            Entity::Data(data) => Ok(data),
            Entity::Key(_) => Err(Error::EntityNotFetched),
            Entity::None => Err(Error::EntityEmpty),
        }
    }

    /// Returns the mutable data if it exists and was fetched/created
    pub fn data_mut(&mut self) -> Result<&mut T> {
        match self {
            Entity::Data(ref mut data) => Ok(data),
            Entity::Key(_) => Err(Error::EntityNotFetched),
            Entity::None => Err(Error::EntityEmpty),
        }
    }

    /// Is this a Key variant?
    pub fn is_key(&self) -> bool {
        matches!(self, Self::Key(..))
    }

    /// Is this a Data variant?
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data(..))
    }

    /// Is this a None variant?
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl<K, T> From<T> for Entity<K, T> {
    fn from(entity: T) -> Self {
        Self::Data(Box::new(entity))
    }
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><======================  ENTITY LABEL  ========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

/// Enum for situations when you want to define not just a Key but also a Label
///
/// This may be useful for faster access to avoid an extra LEFT JOIN or because
/// you may have missing data, thus no Key, and having the Label makes the entity
/// still valid
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum EntityLabel<K, T, L> {
    /// Key and Label for this entity
    KeyLabel(Key<K>, L),
    /// Created/Fetched data for the entity
    Data(Box<T>),
    /// For when you have no data to fill or null from database
    #[default]
    None,
}

impl<K, T, L> Keyed for EntityLabel<K, T, L>
where
    T: Keyed<KeyType = K>,
{
    type KeyType = K;

    fn key(&self) -> Result<&Key<Self::KeyType>> {
        match self {
            EntityLabel::KeyLabel(key, _) => Ok(key),
            EntityLabel::Data(data) => data.key(),
            EntityLabel::None => Err(Error::EntityLabelEmpty),
        }
    }
}

impl<K, T, L> Label for EntityLabel<K, T, L>
where
    T: Label<LabelType = L>,
{
    type LabelType = L;

    fn label(&self) -> Result<&Self::LabelType> {
        match self {
            EntityLabel::KeyLabel(_, label) => Ok(label),
            EntityLabel::Data(data) => data.label(),
            EntityLabel::None => Err(Error::EntityLabelEmpty),
        }
    }
}

impl<K, T, L> EntityLabel<K, T, L> {
    /// Returns the data if it exists and was fetched/created
    pub fn data(&self) -> Result<&T> {
        match self {
            EntityLabel::Data(data) => Ok(data),
            EntityLabel::KeyLabel(..) => Err(Error::EntityLabelNotFetched),
            EntityLabel::None => Err(Error::EntityLabelEmpty),
        }
    }

    /// Returns the mutable data if it exists and was fetched/created
    pub fn data_mut(&mut self) -> Result<&mut T> {
        match self {
            EntityLabel::Data(ref mut data) => Ok(data),
            EntityLabel::KeyLabel(..) => Err(Error::EntityLabelNotFetched),
            EntityLabel::None => Err(Error::EntityLabelEmpty),
        }
    }

    /// Is this a KeyLabel variant?
    pub fn is_keylabel(&self) -> bool {
        matches!(self, Self::KeyLabel(..))
    }

    /// Is this a Data variant?
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data(..))
    }

    /// Is this a None variant?
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl<K, T, L> From<T> for EntityLabel<K, T, L> {
    fn from(entity: T) -> Self {
        Self::Data(Box::new(entity))
    }
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><==========================  MANY  ============================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

/// Enum for defining one-to-many or many-to-many relationships
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum Many<T> {
    /// This holds the created/fetched data in a vector
    Data(Vec<T>),
    /// For when the data exists but is not fetched
    NotFetched,
    /// For when you have no data to fill or fetch from the DB
    #[default]
    None,
}

impl<T> Many<T> {
    /// Returns the `Vec` of data if they exist and were fetched/created
    pub fn data(&self) -> Result<&Vec<T>> {
        match self {
            Many::Data(data) => Ok(data),
            Many::NotFetched => Err(Error::ManyNotFetched),
            Many::None => Err(Error::ManyEmpty),
        }
    }

    /// Returns the mutable `Vec` of data if they exist and were fetched/created
    pub fn data_mut(&mut self) -> Result<&mut Vec<T>> {
        match self {
            Many::Data(ref mut data) => Ok(data),
            Many::NotFetched => Err(Error::ManyNotFetched),
            Many::None => Err(Error::ManyEmpty),
        }
    }

    /// Is this a Data variant?
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data(..))
    }

    /// Is this a NotFetched variant?
    pub fn is_not_fetched(&self) -> bool {
        matches!(self, Self::NotFetched)
    }

    /// Is this a None variant?
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl<T> From<Vec<T>> for Many<T> {
    fn from(entities: Vec<T>) -> Self {
        Self::Data(entities)
    }
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><==========================  ERROR  ===========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

/// The error type for all errors in this crate
#[derive(Error, Debug)]
pub enum Error {
    /// for an empty entity
    #[error("nothing set for this Entity")]
    EntityEmpty,
    /// for an empty entity label
    #[error("nothing set for this EntityLabel")]
    EntityLabelEmpty,
    /// for an entity that was not fetched
    #[error("data was not fetched from the database for this Entity")]
    EntityNotFetched,
    /// for an entity label that was not fetched
    #[error("data was not fetched from the database for this EntityLabel")]
    EntityLabelNotFetched,
    /// for a Many that has no data
    #[error("no data set for this Many")]
    ManyEmpty,
    /// for a Many that has no data fetched
    #[error("data were not fetched from the database for this Many")]
    ManyNotFetched,
}

/// The result typedef for this crate for convenience
pub type Result<T> = core::result::Result<T, Error>;

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><======================  CONVENIENCE  =========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

/// An Int typedef for convenience to be used as Key number
pub type Int = usize;
/// An Entity that has an Int as key
pub type EntityInt<T> = Entity<Int, T>;
/// An Entity that has a String as key
pub type EntityString<T> = Entity<String, T>;
/// An EntityLabel that has an Int as key
pub type EntityLabelInt<T> = EntityLabel<Int, T, String>;
/// An EntityLabel that has a String as key
pub type EntityLabelString<T> = EntityLabel<String, T, String>;

pub mod prelude {
    //! Convenience re-export of common members
    //!
    //! This module simplifies importing of common items, while excluding
    //! ones that can easily clash with other crates.
    //!
    //! ```
    //! use dbent::prelude::*;
    //! ```

    #[cfg(feature = "derive")]
    pub use dbent_derive::{
        Entity,
        Label,
    };

    pub use crate::{
        Key,
        Keyed,
        Label,
        Tagged,
        Tag,
        Entity,
        EntityLabel,
        Many,
        Int,
        EntityInt,
        EntityString,
        EntityLabelInt,
        EntityLabelString,
    };
}

