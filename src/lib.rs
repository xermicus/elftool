#[macro_use]
extern crate lazy_static;

// Support 32bit later
pub type Elf64Addr     = u64;
pub type Elf64Off      = u64;
pub type Elf64Section  = u64;
pub type Elf64Versym   = u64;
pub type ElfByte       = u8;
pub type Elf64Half     = u16;
pub type Elf64Sword    = i32;
pub type Elf64Word     = u32;
pub type Elf64Sxword   = i64;
pub type Elf64Xword    = u64;

pub mod elf_file;
pub mod elf_header;
pub mod program_header;
pub mod section_header;
pub mod helpers;
// ToDo: Symbol Table, Segments, Sections to Segments mapping
