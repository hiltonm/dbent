
use super::*;

#[derive(Default)]
struct Model {
    id: Key<Int>,
    label: String,
}

impl Keyed for Model {
    type KeyType = Int;

    fn key(&self) -> Result<&Key<Self::KeyType>> {
        Ok(&self.id)
    }
}

impl Label for Model {
    type LabelType = String;

    fn label(&self) -> Result<&Self::LabelType> {
        Ok(&self.label)
    }
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><=========================  TRAITS  ===========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

#[test]
fn test_keyed() -> Result<()> {
    struct Entity {
        id: Key<i32>,
    }

    impl Keyed for Entity {
        type KeyType = i32;

        fn key(&self) -> Result<&Key<Self::KeyType>> {
            Ok(&self.id)
        }
    }

    let entity = Entity { id: Key::new(1) };
    assert_eq!(*entity.key()?, Key::new(1));
    assert_eq!(**entity.key()?, Some(1));
    Ok(())
}

#[test]
fn test_label() -> Result<()> {
    struct Entity {
        label: String,
    }

    impl Label for Entity {
        type LabelType = String;

        fn label(&self) -> Result<&Self::LabelType> {
            Ok(&self.label)
        }
    }

    let entity = Entity { label: "Entity".to_owned() };
    assert_eq!(*entity.label()?, "Entity");
    Ok(())
}

#[test]
fn test_tagged() -> Result<()> {
    let entity = Model { id: Key::new(1), label: "Entity".to_owned() };
    let tag = entity.tag()?;
    assert_eq!(tag.key, "1");
    assert_eq!(tag.label, "Entity");
    assert!(entity.has_tag());
    Ok(())
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><==========================  KEY  =============================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

#[test]
fn test_key() {
    let id = Key::new(1);
    assert_eq!(*id, Some(1));
    assert_eq!(format!("{}", id), "1");

    let id = Key::from(Some(2));
    assert_eq!(*id, Some(2));
    assert_eq!(format!("{}", id), "2");

    let id = Key::from(&Some(3));
    assert_eq!(*id, Some(3));
    assert_eq!(format!("{}", id), "3");
}

#[test]
fn test_key_display() {
    let id = Key::new(1);
    assert_eq!(format!("{}", id), "1");
    let id = Key::<i32>(None);
    assert_eq!(format!("{}", id), "None");
}

#[test]
fn test_key_from_option() {
    let id = Key::from(Some(1));
    assert_eq!(id, Key::new(1));
    let id = Key::<i32>::from(None);
    assert_eq!(id, Key::<i32>(None));
}

#[test]
fn test_key_from_reference() {
    let id = Key::from(&Some(1));
    assert_eq!(id, Key::new(1));
    let id = Key::from(&None);
    assert_eq!(id, Key::<i32>(None));
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_key_to_from_sql() {
    let id = Key::new(1);
    let result: Result<Key<i32>, _> = ValueRef::from(&Some(1)).from_sql();
    assert_eq!(result.unwrap(), id);
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_key_from_sql() {
    use rusqlite::types::Value;
    let value = Value::from(1);
    let id = Key::<i32>::column_result(value).unwrap();
    assert_eq!(id, Key::new(1));
    let value = Value::Null;
    let id = Key::<i32>::column_result(value).unwrap();
    assert_eq!(id, Key::<i32>(None));
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_key_to_sql() {
    use rusqlite::types::ToSqlOutput;
    let id = Key::new(1);
    let value = id.to_sql().unwrap();
    assert_eq!(value, ToSqlOutput::from(1));
    let id = Key::<i32>(None);
    let value = id.to_sql().unwrap();
    assert_eq!(value, ToSqlOutput::from(rusqlite::types::Value::Null));
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><=========================  ENTITY  ===========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

#[test]
fn test_entity_key() -> Result<()> {
    let entity = Model { id: Key::new(1), label: "Entity".to_owned() };
    assert_eq!(entity.key()?.unwrap(), 1);
    assert_eq!(**entity.key()?, Some(1));

    let entity: Entity<Int, Model> = Key::new(1).into_entity();
    assert_eq!(entity.key()?.unwrap(), 1);
    assert_eq!(**entity.key()?, Some(1));
    Ok(())
}

#[test]
fn test_entity_key_none() -> Result<()> {
    let entity = Model::default();
    assert!(entity.key()?.is_none());
    Ok(())
}

#[test]
fn test_entity_data() -> Result<()> {
    let entity: EntityInt<Model> = Model { id: Key::new(1), label: "Entity".to_owned() }.into();
    let result = entity.data()?;
    assert_eq!(entity.key()?.unwrap(), 1);
    assert_eq!(result.key()?.unwrap(), 1);
    assert_eq!(result.label()?, "Entity");
    Ok(())
}

#[test]
fn test_entity_data_none() {
    let entity = Entity::<Int, Model>::None;
    assert!(entity.data().is_err());
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><======================  ENTITY LABEL  ========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

#[test]
fn test_entity_label_label() -> Result<()> {
    let entity_label = EntityLabelInt::<Model>::KeyLabel(Key::new(1), String::from("Label"));
    assert_eq!(entity_label.label()?, &String::from("Label"));
    Ok(())
}

#[test]
fn test_entity_label_key() -> Result<()>{
    let entity_label = EntityLabelInt::<Model>::KeyLabel(Key::new(1), String::from("Label"));
    assert_eq!(entity_label.key()?.unwrap(), 1);
    Ok(())
}

#[test]
fn test_entity_label_data() -> Result<()> {
    let entity_label: EntityLabelInt<Model> = Model { id: Key::new(1), label: "Entity".to_owned() }.into();
    let result = entity_label.data()?;
    assert_eq!(entity_label.key()?.unwrap(), 1);
    assert_eq!(result.key()?.unwrap(), 1);
    assert_eq!(result.label()?, "Entity");
    Ok(())
}

#[test]
fn test_entity_label_data_none() {
    let entity_label = EntityLabelInt::<Model>::None;
    assert!(entity_label.data().is_err());
}

#[test]
fn test_entity_label_missing_data() {
    let entity_label = EntityLabelInt::<Model>::KeyLabel(Key::new(1), String::from("Label"));
    assert!(entity_label.data().is_err());
}

#[test]
fn test_entity_label_tag() -> Result<()> {
    let entity_label = EntityLabelInt::<Model>::KeyLabel(Key::new(1), String::from("Label"));
    let tag = entity_label.tag()?;
    assert_eq!(tag.key, "1");
    assert_eq!(tag.label, "Label");
    Ok(())
}

#[test]
fn test_entity_label_has_tag() {
    let entity_label = EntityLabelInt::<Model>::KeyLabel(Key::new(1), String::from("Label"));
    assert!(entity_label.has_tag());
}

#[test]
fn test_entity_label_eq() {
    let entity_label_1 = EntityLabelInt::<i32>::KeyLabel(Key::new(1), String::from("Label"));
    let entity_label_2 = EntityLabelInt::<i32>::KeyLabel(Key::new(1), String::from("Label"));
    assert_eq!(entity_label_1, entity_label_2);
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><==========================  MANY  ============================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

#[test]
fn test_many_not_fetched() {
    let mut many = Many::<Model>::NotFetched;
    assert!(many.data().is_err());
    assert!(many.data_mut().is_err());
}

#[test]
fn test_many_none() {
    let mut many = Many::<Model>::None;
    assert!(many.data().is_err());
    assert!(many.data_mut().is_err());
}

#[test]
fn test_many_data() -> Result<()> {
    let mut data = vec![1, 2, 3];
    let mut many = Many::Data(data.clone());
    assert_eq!(many.data()?, &data);
    assert_eq!(many.data_mut()?, &mut data);
    Ok(())
}
