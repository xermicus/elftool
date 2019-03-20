use {Elf64Addr, Elf64Off};
use elf_header::Elf64Ehdr;
use helpers::{Error, SH_TYPES, SH_FLAGS, cast_u32, cast_u64};

#[derive(Default, Debug)]
pub struct Elf64Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: Elf64Addr,
    sh_offset: Elf64Off,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64
}

impl Elf64Shdr {
    pub fn print(&self) {
        println!("{:?}", self);
    }

    pub fn parse_flags(&self) -> String {
        let mut t = String::new();
        if self.sh_flags & (1 << 0) > 0 { t.push_str("w"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 1) > 0 { t.push_str("a"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 2) > 0 { t.push_str("x"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 4) > 0 { t.push_str("m"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 5) > 0 { t.push_str("s"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 6) > 0 { t.push_str("i"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 7) > 0 { t.push_str("l"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 8) > 0 { t.push_str("n"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 9) > 0 { t.push_str("g"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 10) > 0 { t.push_str("t"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 11) > 0 { t.push_str("c"); } else { t.push_str("-"); };
        if self.sh_flags & 0x0ff00000 > 0 { t.push_str("O"); } else { t.push_str("-"); };
        if self.sh_flags & 0xf0000000 > 0 { t.push_str("P"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 30) > 0 { t.push_str("o"); } else { t.push_str("-"); };
        if self.sh_flags & (1 << 31) > 0 { t.push_str("e"); } else { t.push_str("-"); };
        t
    }

    pub fn explain(&self, name: &String) {
        print!("0x{:016x}\t", self.sh_offset);
        print!("0x{:016x}\t", self.sh_size);
        print!("0x{:016x}\t", self.sh_link);
        print!("  {}\t", self.parse_flags());
        if let Some(t) = SH_TYPES.get(&self.sh_type) {
            println!("{}", t);
        } else {
            println!("Unknown");
        }
        print!("0x{:016x}\t", self.sh_addr);
        print!("0x{:016x}\t", self.sh_entsize);
        print!("0x{:016x}\t", self.sh_info);
        print!("0x{:016x}\t", self.sh_addralign);
        println!("{}", name);
    }

    pub fn get_name(&self, shstrtab: &[u8]) -> String {
        let mut string_table = String::new();
        let mut index = self.sh_name as usize;
        loop {
            if shstrtab[index] > 0 {
                string_table.push(shstrtab[index] as char);
                index += 1;
            } else {
                break;
            }
        }
        string_table
    }
}

pub fn explain_shdr_table(shdr_table: &Vec<Elf64Shdr>, e_shstrndx: usize, input_file: &Vec<u8>) {
    let shstrtab_start = shdr_table[e_shstrndx].sh_offset as usize;
    let shstrtab_end = (shdr_table[e_shstrndx].sh_offset + shdr_table[e_shstrndx].sh_size) as usize;
    let shstrtab = &input_file[shstrtab_start..shstrtab_end];

    println!("Offset\t\t\tSize\t\t\tLink\t\t\tFlags\t\t\tType");
    println!("Address\t\t\tEntsize\t\t\tInfo\t\t\tAlign\t\t\tName");

	shdr_table.iter().for_each(|n| {
		println!();
		n.explain(&n.get_name(shstrtab));
	});

    println!("\nFlags:\n{}\n", SH_FLAGS);
}

pub fn parse_shdr(input_file: &Vec<u8>, ehdr: &Elf64Ehdr) -> Result<Vec<Elf64Shdr>, Error> {
    let mut shdr_table = Vec::new();
    for i in 0..ehdr.e_shnum {
        let i_offset = (ehdr.e_shoff + (i * ehdr.e_shentsize) as u64) as usize;
        let input_buffer = &input_file[i_offset..i_offset + ehdr.e_shentsize as usize];
        let shdr = Elf64Shdr {
            sh_name: cast_u32(&input_buffer[..4], false)?,
            sh_type: cast_u32(&input_buffer[4..8], false)?,
            sh_flags: cast_u64(&input_buffer[8..16], false)?,
            sh_addr: cast_u64(&input_buffer[16..24], false)? as Elf64Addr,
            sh_offset: cast_u64(&input_buffer[24..32], false)? as Elf64Off,
            sh_size: cast_u64(&input_buffer[32..40], false)?,
            sh_link: cast_u32(&input_buffer[40..44], false)?,
            sh_info: cast_u32(&input_buffer[44..48], false)?,
            sh_addralign: cast_u64(&input_buffer[48..56], false)?,
            sh_entsize: cast_u64(&input_buffer[56..64], false)?
        };
        shdr_table.push(shdr);
    };
    Ok(shdr_table)
}
