use crate::dml;
use crate::validator::directive::DirectiveListValidator;
use std::collections::HashMap;

mod db;
mod default;
mod embedded;
mod ondelete;
mod primary;
mod relation;
mod scalarlist;
mod sequence;
mod unique;

pub fn new_builtin_field_directives() -> DirectiveListValidator<dml::Field> {
    let mut validator = DirectiveListValidator::<dml::Field> {
        known_directives: HashMap::new(),
    };

    validator.add(Box::new(db::DbDirectiveValidator {}));
    validator.add(Box::new(primary::PrimaryDirectiveValidator {}));
    validator.add(Box::new(scalarlist::ScalarListDirectiveValidator {}));
    validator.add(Box::new(sequence::SequenceDirectiveValidator {}));
    validator.add(Box::new(unique::UniqueDirectiveValidator {}));
    validator.add(Box::new(default::DefaultDirectiveValidator {}));
    validator.add(Box::new(relation::RelationDirectiveValidator {}));
    validator.add(Box::new(ondelete::OnDeleteDirectiveValidator {}));

    return validator;
}

pub fn new_builtin_model_directives() -> DirectiveListValidator<dml::Model> {
    let mut validator = DirectiveListValidator::<dml::Model> {
        known_directives: HashMap::new(),
    };

    validator.add(Box::new(db::DbDirectiveValidator {}));
    validator.add(Box::new(embedded::EmbeddedDirectiveValidator {}));

    return validator;
}

pub fn new_builtin_enum_directives() -> DirectiveListValidator<dml::Enum> {
    let validator = DirectiveListValidator::<dml::Enum> {
        known_directives: HashMap::new(),
    };

    // Adds are missing

    return validator;
}