use crate::blueprint::FieldDefinition;
use crate::config::{self, ConfigModule, Field};
use crate::lambda::{Context, Expression};
use crate::try_fold::TryFold;
use crate::valid::Valid;

pub fn update_protected<'a>(
    type_name: &'a str,
) -> TryFold<'a, (&'a ConfigModule, &'a Field, &'a config::Type, &'a str), FieldDefinition, String>
{
    TryFold::<(&ConfigModule, &Field, &config::Type, &'a str), FieldDefinition, String>::new(
        |(config, field, type_, _), mut b_field| {
            if field.protected.is_some() // check the field itself has marked as protected
                || type_.protected.is_some() // check the type that contains current field
                || config // check that output type of the field is protected
                    .find_type(&field.type_of)
                    .and_then(|type_| type_.protected.as_ref())
                    .is_some()
            {
                if config.input_types().contains(&type_name.to_string()) {
                    return Valid::fail("Input types can not be protected".to_owned());
                }

                if !config.extensions.has_auth() {
                    return Valid::fail(
                        "@protected operator is used but there is no @link definitions for auth providers".to_owned(),
                    );
                }

                b_field.resolver = Some(Expression::Protect(Box::new(b_field.resolver.unwrap_or(
                    Expression::Context(Context::Path(vec![b_field.name.clone()])),
                ))));
            }

            Valid::succeed(b_field)
        },
    )
}
