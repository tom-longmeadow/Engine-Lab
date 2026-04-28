// use std::{marker::PhantomData, ops::Range};

// use crate::{
//     property::{
//         config::PropertyConfig,
//         node::FlattenedProperty,
//         propertied::Propertied,
//     },
//     unit::UnitSystem,
// };

// #[derive(Clone, Debug)]
// pub struct PropertyPanelRow {
//     pub label: String,
//     pub value: String,
//     pub unit: String,
// }

// pub struct PropertyPanelModel<'a, C: PropertyConfig, T: Propertied<C>> {
//     pub properties: Vec<FlattenedProperty<C>>,
//     pub object: &'a T,
//     pub system: &'a UnitSystem<C>,
//     _c: PhantomData<C>,
// }

// impl<'a, C: PropertyConfig, T: Propertied<C>> PropertyPanelModel<'a, C, T> {
//     pub fn new(object: &'a T, system: &'a UnitSystem<C>) -> Self {
//         Self {
//             properties: T::get_schema().flatten(),
//             object,
//             system,
//             _c: PhantomData,
//         }
//     }

//     pub fn rows(&self, lang: C::Lang) -> Vec<PropertyPanelRow> {
//         self.properties
//             .iter()
//             .map(|property| PropertyPanelRow {
//                 label: property.header(lang),
//                 value: property.value(self.object, self.system),
//                 unit: property.unit_label(self.system),
//             })
//             .collect()
//     }
// }

// pub trait RowSource<C: PropertyConfig> {
//     type Row: Propertied<C>;

//     fn total_rows(&self) -> usize;
//     fn fetch_rows(&self, range: Range<usize>) -> Vec<&Self::Row>;
// }

// #[derive(Clone, Copy, Debug, Default)]
// pub struct Viewport {
//     pub row_offset: usize,
//     pub col_offset: usize,
//     pub visible_rows: usize,
//     pub visible_cols: usize,
//     pub row_buffer: usize,
// }

// impl Viewport {
//     pub fn fetch_range(&self) -> Range<usize> {
//         let start = self.row_offset.saturating_sub(self.row_buffer);
//         let end = self.row_offset + self.visible_rows + self.row_buffer;
//         start..end
//     }
// }

// pub struct PropertyGridModel<'a, C: PropertyConfig, S: RowSource<C>> {
//     pub all_properties: Vec<FlattenedProperty<C>>,
//     pub visible_columns: Vec<usize>,
//     pub viewport: Viewport,
//     pub source: &'a S,
//     pub system: &'a UnitSystem<C>,
//     _c: PhantomData<C>,
// }

// impl<'a, C: PropertyConfig, S: RowSource<C>> PropertyGridModel<'a, C, S>
// where
//     S::Row: Propertied<C>,
// {
//     pub fn new(source: &'a S, system: &'a UnitSystem<C>, viewport: Viewport) -> Self {
//         let all_properties = S::Row::get_schema().flatten();
//         let visible_columns = (0..all_properties.len()).collect();

//         Self {
//             all_properties,
//             visible_columns,
//             viewport,
//             source,
//             system,
//             _c: PhantomData,
//         }
//     }

//     pub fn headers(&self, lang: C::Lang) -> Vec<String> {
//         self.visible_columns
//             .iter()
//             .map(|&i| self.all_properties[i].header(lang))
//             .collect()
//     }

//     pub fn cells(&self) -> Vec<Vec<String>> {
//         let rows = self.source.fetch_rows(self.viewport.fetch_range());

//         rows.iter()
//             .map(|row| {
//                 self.visible_columns
//                     .iter()
//                     .map(|&i| self.all_properties[i].value(*row, self.system))
//                     .collect()
//             })
//             .collect()
//     }

//     pub fn scroll_to_row(&mut self, row: usize) {
//         self.viewport.row_offset = row.min(self.source.total_rows().saturating_sub(1));
//     }

//     pub fn show_column(&mut self, col_index: usize) {
//         if !self.visible_columns.contains(&col_index) {
//             self.visible_columns.push(col_index);
//             self.visible_columns.sort_unstable();
//         }
//     }

//     pub fn hide_column(&mut self, col_index: usize) {
//         self.visible_columns.retain(|&i| i != col_index);
//     }
// }
