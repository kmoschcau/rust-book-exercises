#[derive(Debug)]
pub struct AddNode {
    pub name: String,
    pub department: String,
}

#[derive(Debug)]
pub struct ShowNode {
    pub department: String,
}

#[derive(Debug)]
pub enum CommandNode {
    Add(AddNode),
    Show(ShowNode),
    ShowAll,
    Quit,
}
