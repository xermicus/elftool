use {Elf64Addr, Elf64Off};
use elf_header::Elf64Ehdr;
use helpers::{Error, P_TYPES, cast_u32, cast_u64};

#[derive(Default, Debug)]
pub struct Elf64Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: Elf64Off,
    p_vaddr: Elf64Addr,
    p_paddr: Elf64Addr,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64
}

impl Elf64Phdr {
    pub fn print(&self) {
        println!("{:?}", &self);
    }
    
    pub fn explain(&self) {
        print!("0x{:016x}\t", self.p_offset);
        print!("0x{:016x}\t", self.p_paddr);
        print!("0x{:016x}\t", self.p_filesz);
        if let Some(t) = P_TYPES.get(&self.p_type) {
            match t {
                &"PT_INTERP" => println!("{}", t), // ToDo: Print linker name
                _ => println!("{}", t),
            }
        } else {
            println!("An unknown type");
        };
        print!("0x{:016x}\t", self.p_align);
        print!("0x{:016x}\t", self.p_vaddr);
        print!("0x{:016x}\t", self.p_memsz);
        let parse_flags = |p_flags| {
            let mut t = String::with_capacity(3);
            if p_flags & 1 == 1 { t.push_str("x"); } else { t.push_str("-"); };
            if p_flags & 2 == 2 { t.push_str("w"); } else { t.push_str("-"); };
            if p_flags & 4 == 4 { t.push_str("r"); } else { t.push_str("-"); };
            t
        };
        println!("{}", parse_flags(&self.p_flags));
    }
}

pub fn explain_phdr_table(phdr_table: &Vec<Elf64Phdr>, e_phnum: usize) {
    println!("Size\t\t\tPAddr\t\t\tFilesz\t\t\tType");
    println!("Entsize\t\t\tVaddr\t\t\tMemsz\t\t\tFlags");
    for i in 0..e_phnum {
        println!();
        phdr_table[i].explain();
    };
    println!()
}

pub fn is_section_in_segment(phdr_table: &Vec<Elf64Phdr>, sh_type: u32) -> bool {
    // ToDo
    false
}

pub fn parse_phdr(input_file: &Vec<u8>, ehdr: &Elf64Ehdr) -> Result<Vec<Elf64Phdr>, Error> {
    if ehdr.e_type < 2 || ehdr.e_type > 3 || ehdr.e_phoff == 0 {
        return Err(Error::PhdrNotRelevant)
    }

    let mut phdr_table = Vec::new();
    for i in 0..ehdr.e_phnum {
        let i_offset = (ehdr.e_phoff + (i * ehdr.e_phentsize) as u64) as usize;
        let input_buffer = &input_file[i_offset..i_offset + ehdr.e_phentsize as usize];
        let phdr = Elf64Phdr {
            p_type: cast_u32(&input_buffer[..4], false)?,
            p_flags: cast_u32(&input_buffer[4..8], false)?,
            p_offset: cast_u64(&input_buffer[8..16], false)? as Elf64Off,
            p_vaddr: cast_u64(&input_buffer[16..24], false)? as Elf64Addr,
            p_paddr: cast_u64(&input_buffer[24..32], false)? as Elf64Addr,
            p_filesz: cast_u64(&input_buffer[32..40], false)?,
            p_memsz: cast_u64(&input_buffer[40..48], false)?,
            p_align: cast_u64(&input_buffer[48..56], false)?,
        };
        phdr_table.push(phdr);
    }

    Ok(phdr_table)
}
