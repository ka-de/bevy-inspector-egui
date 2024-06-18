use bevy_reflect::{ TypeData, TypeInfo, TypeRegistry };

use crate::{
    inspector_options::{ std_options::NumberOptions, Target },
    prelude::ReflectInspectorOptions,
    InspectorOptions,
};

fn insert_options_struct<T: 'static>(
    type_registry: &mut TypeRegistry,
    fields: &[(&'static str, &dyn TypeData)]
) {
    if let Some(registration) = type_registry.get_mut(std::any::TypeId::of::<T>()) {
        if registration.data::<ReflectInspectorOptions>().is_none() {
            let mut options = InspectorOptions::new();
            for (field, data) in fields {
                let info = match registration.type_info() {
                    TypeInfo::Struct(info) => info,
                    _ => unreachable!(),
                };
                if let Some(field_index) = info.index_of(field) {
                    options.insert_boxed(
                        Target::Field(field_index),
                        TypeData::clone_type_data(*data)
                    );
                } else {
                    bevy_log::warn!("Field index not found: {}", field);
                }
            }
            registration.insert(ReflectInspectorOptions(options));
        }
    } else {
        bevy_log::warn!(
            "Attempting to set default inspector options for {}, but it wasn't registered in the type registry.",
            std::any::type_name::<T>()
        );
    }
}

fn insert_options_enum<T: 'static>(
    type_registry: &mut TypeRegistry,
    fields: &[(&'static str, &'static str, &dyn TypeData)]
) {
    if let Some(registration) = type_registry.get_mut(std::any::TypeId::of::<T>()) {
        if registration.data::<ReflectInspectorOptions>().is_none() {
            let mut options = InspectorOptions::new();
            for (variant, field, data) in fields {
                let info = match registration.type_info() {
                    TypeInfo::Enum(info) => info,
                    _ => unreachable!(),
                };
                if let Some(variant_index) = info.index_of(variant) {
                    let field_index = match info.variant_at(variant_index) {
                        Some(bevy_reflect::VariantInfo::Struct(strukt)) => {
                            strukt.index_of(field)
                        }
                        Some(bevy_reflect::VariantInfo::Tuple(_)) => { field.parse().ok() }
                        Some(bevy_reflect::VariantInfo::Unit(_)) => unreachable!(),
                        None => {
                            bevy_log::warn!("Variant not found: {}", variant);
                            continue;
                        }
                    };

                    if let Some(field_index) = field_index {
                        options.insert_boxed(
                            Target::VariantField {
                                variant_index,
                                field_index,
                            },
                            TypeData::clone_type_data(*data)
                        );
                    } else {
                        bevy_log::warn!("Field index not found for variant: {}", variant);
                    }
                } else {
                    bevy_log::warn!("Variant index not found: {}", variant);
                }
            }
            registration.insert(ReflectInspectorOptions(options));
        }
    } else {
        bevy_log::warn!(
            "Attempting to set default inspector options for {}, but it wasn't registered in the type registry.",
            std::any::type_name::<T>()
        );
    }
}

pub fn register_default_options(type_registry: &mut TypeRegistry) {
    #[rustfmt::skip]
    insert_options_enum::<bevy_color::Color>(
        type_registry,
        &[
            ("Srgba", "red", &NumberOptions::<f32>::normalized()),
            ("Srgba", "green", &NumberOptions::<f32>::normalized()),
            ("Srgba", "blue", &NumberOptions::<f32>::normalized()),
            ("Srgba", "alpha", &NumberOptions::<f32>::normalized()),
            ("LinearRgba", "red", &NumberOptions::<f32>::normalized()),
            ("LinearRgba", "green", &NumberOptions::<f32>::normalized()),
            ("LinearRgba", "blue", &NumberOptions::<f32>::normalized()),
            ("LinearRgba", "alpha", &NumberOptions::<f32>::normalized()),
            ("Hsla", "hue", &NumberOptions::<f32>::between(0.0, 360.0)),
            ("Hsla", "saturation", &NumberOptions::<f32>::normalized()),
            ("Hsla", "lightness", &NumberOptions::<f32>::normalized()),
            ("Hsla", "alpha", &NumberOptions::<f32>::normalized()),
            // TODO
            // ("Hsva", "hue", f32),
            // ("Hsva", "saturation", f32),
            // ("Hsva", "value", f32),
            // ("Hsva", "alpha", f32),
            // ("Hwba", "alpha", f32)
            // ("Hwba", "alpha", f32)
            // ("Hwba", "alpha", f32)
            // ("Hwba", "alpha", f32)
            // ("Laba", "alpha", f32)
            // ("Laba", "alpha", f32)
            // ("Laba", "alpha", f32)
            // ("Laba", "alpha", f32)
            // ("Lcha", "alpha", f32)
            // ("Lcha", "alpha", f32)
            // ("Lcha", "alpha", f32)
            // ("Lcha", "alpha", f32)
            // ("Oklaba", "alpha", f32)
            // ("Oklaba", "alpha", f32)
            // ("Oklaba", "alpha", f32)
            // ("Oklaba", "alpha", f32)
            // ("Oklcha", "alpha", f32)
            // ("Oklcha", "alpha", f32)
            // ("Oklcha", "alpha", f32)
            // ("Oklcha", "alpha", f32)
            // ("Xyza", "alpha", f32)
            // ("Xyza", "alpha", f32)
            // ("Xyza", "alpha", f32)
            // ("Xyza", "alpha", f32)
        ],
    );

    insert_options_struct::<bevy_render::view::ColorGrading>(
        type_registry,
        &[
            ("exposure", &NumberOptions::<f32>::positive().with_speed(0.01)),
            ("gamma", &NumberOptions::<f32>::positive().with_speed(0.01)),
            ("pre_saturation", &NumberOptions::<f32>::positive().with_speed(0.01)),
            ("post_saturation", &NumberOptions::<f32>::positive().with_speed(0.01)),
        ]
    );

    insert_options_enum::<bevy_core_pipeline::core_3d::Camera3dDepthLoadOp>(
        type_registry,
        &[("Clear", "0", &NumberOptions::<f32>::normalized())]
    );

    type_registry.register::<bevy_time::Virtual>();

    insert_options_struct::<bevy_time::Virtual>(
        type_registry,
        &[
            ("relative_speed", &NumberOptions::<f64>::positive()),
            ("effective_speed", &NumberOptions::<f64>::positive()),
        ]
    );
}
