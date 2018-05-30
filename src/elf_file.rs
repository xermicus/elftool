use elf_header::{Elf64Ehdr, parse_ehdr};
use program_header::{Elf64Phdr, parse_phdr, explain_phdr_table};
use section_header::{Elf64Shdr, parse_shdr, explain_shdr_table};
use helpers::Error;

use std::fs::File;
use std::io::Read;
use std::mem::size_of;
//use std::path::Path;

pub struct ElfFile {
    pub file_path: String,
    pub file_buffer: Vec<u8>,
    pub ehdr: Elf64Ehdr,
    pub phdr: Vec<Elf64Phdr>,
    pub shdr: Vec<Elf64Shdr>,
}

impl ElfFile {
    pub fn explain_ehdr(&self) {
        println!("ELF Header of {}", self.file_path);
        self.ehdr.e_ident.explain();
        self.ehdr.explain();
    }

    pub fn explain_phdr(&self) {
        println!("Program Header Table of {} with {} entries", self.file_path, self.ehdr.e_phnum);
        explain_phdr_table(&self.phdr, self.ehdr.e_phnum as usize);
    }

    pub fn explain_shdr(&self) {
        println!("Section Header Table of {} with {} entires", self.file_path, self.ehdr.e_shnum);
        explain_shdr_table(&self.shdr, self.ehdr.e_shnum as usize, self.ehdr.e_shstrndx as usize, &self.file_buffer);
    }

    pub fn explain_all(&self) {
        self.explain_ehdr();
        self.explain_phdr();
        self.explain_shdr();
    }
}

pub fn parse_from_disk(path: &str) -> Result<ElfFile, Error> {
    let mut file = File::open(path).unwrap();
    let mut file_buffer = Vec::new();
    let ehdr_size = size_of::<Elf64Ehdr>();
    assert!(file.read_to_end(&mut file_buffer).unwrap() > ehdr_size);

    let ehdr = parse_ehdr(&file_buffer)?;
    let phdr = parse_phdr(&file_buffer, &ehdr)?;
    let shdr = parse_shdr(&file_buffer, &ehdr)?;

    Ok (ElfFile {
        file_path: String::from(path),
        file_buffer: file_buffer,
        ehdr: ehdr,
        phdr: phdr,
        shdr: shdr,
    })
}

