use crate::ui::widget::Widget;


pub trait Container {
    fn children(&self) -> &[Box<dyn Widget>];
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>];
    fn children_vec_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;

    fn push(&mut self, child: Box<dyn Widget>) {
        self.children_vec_mut().push(child);
    }
}

pub struct ContainerData { 
    pub children: Vec<Box<dyn Widget>>, 
    pub gap: f32, 
    pub dirty_layout: bool, 
}

impl ContainerData {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: 0.0,
            dirty_layout: true,
        }
    }
}


impl Container for ContainerData {
    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.children[..]
    }

    fn children_vec_mut(&mut self) -> &mut Vec<Box<dyn Widget>> {
        &mut self.children
    }
}
 