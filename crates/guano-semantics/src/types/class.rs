use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Class {
    pub super_class: Option<()>,
    pub prototypes: Vec<()>,
    pub fields: HashMap<(), ()>,
    pub methods: HashMap<(), ()>
}