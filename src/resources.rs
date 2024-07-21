use bytemuck::checked::try_from_bytes;
use bytemuck::{Pod, Zeroable};
use std::mem;
use core::fmt;

pub fn parse_resource_section(binary: &[u8], offset: usize) -> Result<Resources, ParseResourcesError> {
    let mut offset = offset;
    let mut resources = Resources::new();
    
    let slice = binary.get(offset..offset+mem::size_of::<ResourceDirectoryTable>()).unwrap();
    let resource_directory_table = try_from_bytes::<ResourceDirectoryTable>(slice).ok().unwrap();
    offset += mem::size_of::<ResourceDirectoryTable>();

    resources.version = format!("{}.{}", resource_directory_table.major_version, resource_directory_table.minor_version);

    return Ok(resources)
}

pub enum ParseResourcesError {
    NoResourceSectionPresent,
}

pub struct Resources {
    pub version: String,
    pub resources: Vec<Resource>,
}

impl Resources {
    pub fn new() -> Self {
        Resources { version: String::from(""), resources: vec![] }
    }
}

pub struct Resource {
}

#[derive(Copy, Clone, Pod, Zeroable, Default)]
#[repr(C)]
struct ResourceDirectoryTable {
    characteristics: u32,
    time_date_stamp: u32,
    major_version: u16,
    minor_version: u16,
    number_of_name_entries: u16,
    number_of_id_entries: u16
}

impl fmt::Display for Resources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Resources")?;
        writeln!(f, "--------------")?;
        writeln!(f, "User Version:      {}", self.version)?;

        Ok(())
    }
}
