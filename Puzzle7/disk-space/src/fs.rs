use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type DirectoryLink = Rc<RefCell<Directory>>;
pub struct File {
    size: usize,
    name: String,
}
pub struct Directory {
    name: String,
    subdirectories: Vec<DirectoryLink>,
    files: Vec<File>,
    parent: Option<Weak<RefCell<Directory>>>,
}

impl File {
    pub fn new(size: usize, filename: String) -> Self {
        Self {
            size,
            name: filename,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn filename(&self) -> &str {
        &self.name
    }
}

impl Directory {
    // we wrap everything in Rc and RefCell
    pub fn new(name: String) -> DirectoryLink {
        Rc::new(RefCell::new(Self {
            name,
            subdirectories: vec![],
            files: vec![],
            parent: None,
        }))
    }
    pub fn new_with_parent(name: String, parent: &DirectoryLink) -> DirectoryLink {
        Rc::new(RefCell::new(Self {
            name,
            subdirectories: vec![],
            files: vec![],
            // we generate a weak reference from a Rc<RefCell<Directory>>
            parent: Some(Rc::<RefCell<Directory>>::downgrade(parent)),
        }))
    }
    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
    // I can add lifetimes to moved expressions as well
    pub fn add_subdirectory<'f>(
        &mut self,
        directory_name: &str,
        parent: &DirectoryLink,
    ) -> DirectoryLink {
        let new_subdirectory = Directory::new_with_parent(directory_name.to_string(), parent);
        self.subdirectories.push(Rc::clone(&new_subdirectory));
        new_subdirectory
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn subdirectories(&self) -> &Vec<DirectoryLink> {
        &self.subdirectories
    }
    pub fn find(&self, subdirectory_name: &str) -> Option<DirectoryLink> {
        for subdirectory in self.subdirectories.iter() {
            if subdirectory.borrow().name() == subdirectory_name {
                // By cloning Rc we add one to the counter!
                return Some(Rc::clone(subdirectory));
            }
        }
        None
    }
    pub fn parent(&self) -> Option<DirectoryLink> {
        match &self.parent {
            None => None,
            Some(parent) => Weak::upgrade(parent),
        }
    }
    pub fn files(&self) -> &Vec<File> {
        &self.files
    }
    pub fn get_directory_size(&self) -> usize {
        let mut size = 0;
        for file in self.files.iter() {
            size += file.size();
        }
        for dir in self.subdirectories.iter() {
            size += dir.borrow().get_directory_size();
        }
        size
    }
}
