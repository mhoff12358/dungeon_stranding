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
    fn update_template(
        template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
        previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    );
}

pub trait ValueTransformedSignals {
    type Generics: TemplateGenerics;

    fn transform_value(
        context: &<Self::Generics as TemplateGenerics>::Context,
        value: &<Self::Generics as TemplateGenerics>::Value,
    ) -> Vec<Variant>;
}

impl<Transformed: ValueTransformedSignals> TemplateSpawnerUpdateBehavior for Transformed {
    type Generics = <Transformed as ValueTransformedSignals>::Generics;

    fn initialize(
        template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
    ) {
        template.upcast::<Node>().emit_signal(
            "instantiate_template".into(),
            Self::transform_value(context, value).as_slice(),
        );
    }

    fn update_template(
        template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
        previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    ) {
        let mut args = vec![previous.to_variant()];
        args.extend(Self::transform_value(context, value));
        template
            .upcast::<Node>()
            .emit_signal("update_template".into(), args.as_slice());
    }
}

pub struct SignalsUpdate<Generics: TemplateGenerics>
where
    Generics::Value: ToGodot,
{
    _data: PhantomData<Generics>,
}

impl<Generics: TemplateGenerics> ValueTransformedSignals for SignalsUpdate<Generics>
where
    Generics::Value: ToGodot,
{
    type Generics = Generics;

    fn transform_value(
        context: &<Self::Generics as TemplateGenerics>::Context,
        value: &<Self::Generics as TemplateGenerics>::Value,
    ) -> Vec<Variant> {
        return vec![value.to_variant()];
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

    fn update_template(
        mut template: Gd<<Self::Generics as TemplateGenerics>::TemplateType>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
        previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    ) {
        template
            .bind_mut()
            .update_template(value, context, previous);
    }
}
