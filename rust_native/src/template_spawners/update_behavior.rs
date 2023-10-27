use std::marker::PhantomData;

use godot::prelude::*;

use super::template_spawner::{Template, TemplateGenerics};

pub trait TemplateSpawnerUpdateBehavior {
    type Generics: TemplateGenerics;

    fn initialize(
        template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
    );
    fn place_after(
        template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
        previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    );
}

pub struct SignalsUpdate<Generics: TemplateGenerics>
where
    Generics::Value: ToGodot,
{
    _data: PhantomData<Generics>,
}

impl<Generics: TemplateGenerics> TemplateSpawnerUpdateBehavior for SignalsUpdate<Generics>
where
    Generics::Value: ToGodot,
{
    type Generics = Generics;

    fn initialize(
        template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        _context: &<Self::Generics as TemplateGenerics>::Context,
    ) {
        template
            .upcast::<Node>()
            .emit_signal("instantiate_template".into(), &[value.to_variant()]);
    }

    fn place_after(
        template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        _value: &<Self::Generics as TemplateGenerics>::Value,
        _context: &<Self::Generics as TemplateGenerics>::Context,
        previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    ) {
        template.upcast::<Node>().emit_signal(
            "place_after".into(),
            &[if let Some(previous) = previous {
                previous.to_variant()
            } else {
                Variant::nil()
            }],
        );
    }
}

pub struct UpdateSpawnedTemplate<Generics: TemplateGenerics>
where
    Generics::TemplateType: GodotClass + Inherits<Node>,
{
    _data: PhantomData<Generics>,
}

impl<Generics: TemplateGenerics> TemplateSpawnerUpdateBehavior for UpdateSpawnedTemplate<Generics>
where
    Generics::TemplateType: Template<Value = Generics::Value, Context = Generics::Context>,
{
    type Generics = Generics;

    fn initialize(
        mut template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
    ) {
        template.bind_mut().instantiate_template(value, context);
    }

    fn place_after(
        mut template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
        previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    ) {
        template.bind_mut().place_after(value, context, previous);
    }
}
