
#![allow(dead_code)]
use dbent::prelude::*;

type Result = dbent::Result<()>;


//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><=========================  ENTITY  ===========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

#[test]
fn test_key() -> Result {
    #[derive(Default, Entity)]
    struct Model {
        id: Key<Int>,
        data: String,
    }

    let model = Model { id: Key::new(1), data: "Data".to_owned() };
    assert_eq!(model.key()?, &Key::new(1));
    let model = Model::default();
    assert!(model.key()?.is_none());

    Ok(())
}

#[test]
fn test_key_on_entity() -> Result {
    #[derive(Clone, Entity)]
    struct Model1 {
        id: Key<Int>,
        data: String,
    }

    #[derive(Entity)]
    struct Model2 {
        id: Key<Int>,
        data: String,
        model1: EntityInt<Model1>,
    }

    let model1 = Model1 { id: Key::new(1), data: "Data".to_owned() };
    let mut model2 = Model2 { id: Key::new(1), data: "Data".to_owned(), model1: model1.clone().into() };
    assert_eq!(model1.key()?, model2.model1.key()?);

    model2.model1 = Key::new(2).into_entity();
    assert_eq!(model2.model1.key()?, &Key::new(2));

    model2.model1 = Entity::None;
    assert!(model2.model1.key().is_err());

    Ok(())
}

//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//
//<<>><==========================  LABEL  ===========================><<>>//
//<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>//

#[test]
fn test_label() -> Result {
    #[derive(Label)]
    struct Model {
        id: Key<Int>,
        #[label] data: String,
    }

    let model = Model { id: Key::new(1), data: "Data".to_owned() };
    assert_eq!(model.label()?, "Data");

    Ok(())
}

#[test]
fn test_label_on_entity_label() -> Result {
    #[derive(Clone, Entity, Label)]
    struct Model1 {
        id: Key<Int>,
        #[label] data: String,
    }

    #[derive(Entity)]
    struct Model2 {
        id: Key<Int>,
        data: String,
        model1: EntityLabelInt<Model1>,
    }

    let model1 = Model1 { id: Key::new(1), data: "Data".to_owned() };
    let mut model2 = Model2 { id: Key::new(1), data: "Data".to_owned(), model1: model1.clone().into() };
    assert_eq!(model1.label()?, model2.model1.label()?);

    model2.model1 = EntityLabel::KeyLabel(Key::new(2), "Data2".to_owned());
    assert_eq!(model2.model1.label()?, "Data2");

    model2.model1 = EntityLabel::None;
    assert!(model2.model1.label().is_err());

    Ok(())
}
