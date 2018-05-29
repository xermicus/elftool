use {Elf64Addr, Elf64Off};
use helpers::{cast_u16, cast_u32, cast_u64};
use helpers::{Error, E_TYPES, MACHINES, OSABI};

use std::mem::size_of;

#[derive(Default, Debug)]
pub struct Elf64Ehdr {
    pub e_ident: EIdent,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16
}

#[derive(Default, Debug)]
pub struct EIdent {
    pub ei_mag: [u8; 4],
    pub ei_class: u8,
    pub ei_data: u8,
    pub ei_version: u8,
    pub ei_osabi: u8,
    pub ei_abiversion: u8,
    pub ei_pad: [u8; 7],
}

impl EIdent {
    pub fn print(&self) {
        println!("{:?}",self);
    }

    pub fn explain(&self) {
        println!("\tMagic:\t\t\t\t\t{:?}", self.ei_mag);
        print!("\tClass:\t\t\t\t\t0x{:x}\t", self.ei_class);
        match self.ei_class {
            1 => println!("32-bit architecture"),
            2 => println!("64-bit architecture"),
            _ => println!("This class is invalid")
        }

        print!("\tData:\t\t\t\t\t0x{:x}\t", self.ei_data);
        match self.ei_data {
            1 => println!("Two\'s complement, little-endian"),
            2 => println!("Two\'s complement, big-endian"),
            3 => println!("ELFDATANUM"),
            _ => println!("Unknown data format")
        }

        print!("\tELF Version:\t\t\t\t0x{:x}\t", self.ei_version);
        match self.ei_version {
            1 => println!("Current version"),
            _ => println!("Invalid version")
        }
        
        print!("\tOS ABI:\t\t\t\t\t0x{:x}\t", self.ei_osabi);
        if let Some(t) = OSABI.get(&self.ei_osabi) {
            println!("{}", t);
        } else {
            println!("Same as ELFOSABI_SYSV");
        }

        println!("\tOS ABI Version:\t\t\t\t0x{:x}\t", self.ei_abiversion);
        println!("\tEI_PAD:\t\t\t\t\t{:?}", self.ei_pad);
    }
} 

impl Elf64Ehdr {
    pub fn print(&self) {
        println!("{:?}",self);
    }

    pub fn explain(&self) {
        print!("\tType:\t\t\t\t\t0x{:x}\t", self.e_type);
        if let Some(t) = E_TYPES.get(&self.e_type) {
            println!("{}", t);
        } else {
            println!("An unknown type");
        };

        print!("\tMachine:\t\t\t\t0x{:x}\t", self.e_machine);
        if let Some(t) = MACHINES.get(&self.e_machine) {
            println!("{}", t);
        } else {
            println!("Unknown");
        };

        print!("\tFile Version:\t\t\t\t0x{:x}\t", self.e_version);
        match self.e_version {
            0 => println!("Invalid version"),
            _ => println!("Current version")
        };

        println!("\tEntrypoint Address:\t\t\t0x{:x}", self.e_entry);
        println!("\tProgram Headers Offset:\t\t\t0x{:x}", self.e_phoff);
        println!("\tSection Headers Offset:\t\t\t0x{:x}", self.e_shoff);
        println!("\tFlags:\t\t\t\t\t0x{:x}", self.e_flags);
        println!("\tELF Header Size:\t\t\t0x{:x}\tBytes", self.e_ehsize);
        println!("\tProgram Headers Size:\t\t\t0x{:x}\tBytes", self.e_phentsize);
        println!("\tProgram Headers:\t\t\t0x{:x}", self.e_phnum);
        println!("\tSection Headers Size:\t\t\t0x{:x}\tBytes", self.e_shentsize);
        println!("\tSection Headers:\t\t\t0x{:x}", self.e_shnum);
        println!("\tSection Header String Table Index:\t0x{:x}", self.e_shstrndx);
    }
}

pub fn parse_e_ident(input_buffer: &Vec<u8>) -> Result<EIdent, Error> {
    let mut e_ident = EIdent::default();
    
    e_ident.ei_mag.copy_from_slice(&input_buffer[..4]);
    e_ident.ei_class = input_buffer[4];
    e_ident.ei_data = input_buffer[5];
    e_ident.ei_version = input_buffer[6];
    e_ident.ei_osabi = input_buffer[7];
    e_ident.ei_abiversion = input_buffer[8];
    e_ident.ei_pad.copy_from_slice(&input_buffer[9..16]);
    
    Ok(e_ident)
}

pub fn parse_ehdr(input_file: &Vec<u8>) -> Result<Elf64Ehdr, Error> {
    let ehdr_size = size_of::<Elf64Ehdr>();
    let input_buffer = &input_file[..ehdr_size];

    let mut ehdr = Elf64Ehdr::default();
    ehdr.e_ident = parse_e_ident(&input_buffer[..16].to_vec())?;
    ehdr.e_type = cast_u16(&input_buffer[16..18], false)?;
    ehdr.e_machine = cast_u16(&input_buffer[18..20], false)?;
    ehdr.e_version = cast_u32(&input_buffer[20..24], false)?;
    ehdr.e_entry = cast_u64(&input_buffer[24..32], false)? as Elf64Addr;
    ehdr.e_phoff = cast_u64(&input_buffer[32..40], false)? as Elf64Off;
    ehdr.e_shoff = cast_u64(&input_buffer[40..48], false)? as Elf64Off;
    ehdr.e_flags = cast_u32(&input_buffer[48..52], false)?;
    ehdr.e_ehsize = cast_u16(&input_buffer[52..54], false)?;
    ehdr.e_phentsize = cast_u16(&input_buffer[54..56], false)?;
    ehdr.e_phnum = cast_u16(&input_buffer[56..58], false)?;
    ehdr.e_shentsize = cast_u16(&input_buffer[58..60], false)?;
    ehdr.e_shnum = cast_u16(&input_buffer[60..62], false)?;
    ehdr.e_shstrndx = cast_u16(&input_buffer[62..64], false)?;

    Ok(ehdr)
}
