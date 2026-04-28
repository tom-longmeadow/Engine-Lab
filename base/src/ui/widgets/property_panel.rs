use std::marker::PhantomData;

use crate::{
    property::{
        config::PropertyConfig,
        node::{FlattenedProperty, PropertyNode},
        propertied::Propertied,
    },
    ui::{
        layout::layout_params::LayoutParams,
        widget::Widget,
        widgets::{column::Column, label::Label, row::Row, text_field::TextField},
    },
    unit::UnitSystem,
};

pub struct PropertyPanel<C: PropertyConfig> {
    column: Column,
    _c: PhantomData<C>,
}

impl<C: PropertyConfig> PropertyPanel<C> {
    pub fn new<T: Propertied<C>>(
        object: &T,
        system: &UnitSystem<C>,
        lang: C::Lang,
    ) -> Self {
        let mut column = Column::new();
        let schema = T::get_schema();
        
        Self::build_tree(&mut column, &schema, object, system, lang, 0);

        Self {
            column,
            _c: PhantomData,
        }
    }

    pub fn into_column(self) -> Column {
        self.column
    }

    pub fn column(&self) -> &Column {
        &self.column
    }

    pub fn column_mut(&mut self) -> &mut Column {
        &mut self.column
    }

    fn build_tree<T: Propertied<C>>(
        parent: &mut Column,
        node: &PropertyNode<C>,
        object: &T,
        system: &UnitSystem<C>,
        lang: C::Lang,
        depth: usize,
    ) {
        match node {
            PropertyNode::Group { name, children } => {
                // Group header
                let mut group_header = Row::new();
                group_header.push(Box::new(Label::new(name.label(lang))));
                parent.push(Box::new(group_header));

                // Nested column with padding indent
                let mut nested = Column::new();
                nested = nested.with_padding(
                    crate::ui::layout::edge_insets::EdgeInsets {
                        left: 16.0 * (depth + 1) as f32,
                        right: 0.0,
                        top: 0.0,
                        bottom: 0.0,
                    },
                );

                for child in children {
                    Self::build_tree(&mut nested, child, object, system, lang, depth + 1);
                }

                parent.push(Box::new(nested));
            }
            PropertyNode::Leaf(schema) => {
                // Property row: label | value field | unit
                let mut row = Row::new();

                row.push(Box::new(Label::new(schema.name.label(lang))));

                let value = schema.get_formatted_value(object, system);
                row.push(Box::new(TextField::new(value).with_placeholder("—")));

                let unit = match schema.unit {
                    Some(cat) => system.symbol(cat).to_string(),
                    None => String::new(),
                };

                if !unit.is_empty() {
                    row.push(Box::new(Label::new(unit)));
                }

                parent.push(Box::new(row));
            }
        }
    }
}

impl<C: PropertyConfig> std::fmt::Debug for PropertyPanel<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyPanel").finish()
    }
}