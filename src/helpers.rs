use std::collections::HashMap;
use std::io;

// Sources:
// man 5 elf
// /usr/include/elf.h
lazy_static! {
    pub static ref OSABI: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "UNIX System V");
        m.insert(1, "HP-UX");
        m.insert(2, "NetBSD");
        m.insert(3, "Object uses GNU ELF extensions.");
        m.insert(6, "Sun Solaris");
        m.insert(7, "IBM AIX");
        m.insert(8, "SGI Irix");
        m.insert(9, "FreeBSD");
        m.insert(10, "Compaq TRU64 UNIX");
        m.insert(11, "Novell Modesto");
        m.insert(12, "OpenBSD");
        m.insert(64, "ARM EABI");
        m.insert(97, "ARM");
        m.insert(255, "Standalone (embedded) application");
        m
    };

    pub static ref E_TYPES: HashMap<u16, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "No file type");
        m.insert(1, "Relocatable file");
        m.insert(2, "Executable file");
        m.insert(3, "Shared object file");
        m.insert(4, "Core file");
        m
    };

    pub static ref MACHINES: HashMap<u16, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "AT&T WE 32100");
        m.insert(2, "SPARC");
        m.insert(3, "Intel 80386");
        m.insert(4, "Motorola 68000");
        m.insert(5, "Motorola 88000");
        m.insert(6, "Intel MCU");
        m.insert(7, "Intel 80860");
        m.insert(8, "MIPS I Architecture");
        m.insert(9, "IBM System/370 Processor");
        m.insert(10, "MIPS RS3000 Little-endian");
        m.insert(15, "Hewlett-Packard PA-RISC");
        m.insert(16, "Reserved for future use");
        m.insert(17, "Fujitsu VPP500");
        m.insert(18, "Enhanced instruction set SPARC");
        m.insert(19, "Intel 80960");
        m.insert(20, "PowerPC");
        m.insert(21, "64-bit PowerPC");
        m.insert(22, "IBM System/390 Processor");
        m.insert(23, "IBM SPU/SPC");
        m.insert(36, "NEC V800");
        m.insert(37, "Fujitsu FR20");
        m.insert(38, "TRW RH-32");
        m.insert(39, "Motorola RCE");
        m.insert(40, "ARM 32-bit architecture (AARCH32)");
        m.insert(41, "Digital Alpha");
        m.insert(42, "Hitachi SH");
        m.insert(43, "SPARC Version 9");
        m.insert(44, "Siemens TriCore embedded processor");
        m.insert(45, "Argonaut RISC Core, Argonaut Technologies Inc.");
        m.insert(46, "Hitachi H8/300");
        m.insert(47, "Hitachi H8/300H");
        m.insert(48, "Hitachi H8S");
        m.insert(49, "Hitachi H8/500");
        m.insert(50, "Intel IA-64 processor architecture");
        m.insert(51, "Stanford MIPS-X");
        m.insert(52, "Motorola ColdFire");
        m.insert(53, "Motorola M68HC12");
        m.insert(54, "Fujitsu MMA Multimedia Accelerator");
        m.insert(55, "Siemens PCP");
        m.insert(56, "Sony nCPU embedded RISC processor");
        m.insert(57, "Denso NDR1 microprocessor");
        m.insert(58, "Motorola Star*Core processor");
        m.insert(59, "Toyota ME16 processor");
        m.insert(60, "STMicroelectronics ST100 processor");
        m.insert(61, "Advanced Logic Corp. TinyJ embedded processor family");
        m.insert(62, "AMD x86-64 architecture");
        m.insert(63, "Sony DSP Processor");
        m.insert(64, "Digital Equipment Corp. PDP-10");
        m.insert(65, "Digital Equipment Corp. PDP-11");
        m.insert(66, "Siemens FX66 microcontroller");
        m.insert(67, "STMicroelectronics ST9+ 8/16 bit microcontroller");
        m.insert(68, "STMicroelectronics ST7 8-bit microcontroller");
        m.insert(69, "Motorola MC68HC16 Microcontroller");
        m.insert(70, "Motorola MC68HC11 Microcontroller");
        m.insert(71, "Motorola MC68HC08 Microcontroller");
        m.insert(72, "Motorola MC68HC05 Microcontroller");
        m.insert(73, "Silicon Graphics SVx");
        m.insert(74, "STMicroelectronics ST19 8-bit microcontroller");
        m.insert(75, "Digital VAX");
        m.insert(76, "Axis Communications 32-bit embedded processor");
        m.insert(77, "Infineon Technologies 32-bit embedded processor");
        m.insert(78, "Element 14 64-bit DSP Processor");
        m.insert(79, "LSI Logic 16-bit DSP Processor");
        m.insert(80, "Donald Knuth's educational 64-bit processor");
        m.insert(81, "Harvard University machine-independent object files");
        m.insert(82, "SiTera Prism");
        m.insert(83, "Atmel AVR 8-bit microcontroller");
        m.insert(84, "Fujitsu FR30");
        m.insert(85, "Mitsubishi D10V");
        m.insert(86, "Mitsubishi D30V");
        m.insert(87, "NEC v850");
        m.insert(88, "Mitsubishi M32R");
        m.insert(89, "Matsushita MN10300");
        m.insert(90, "Matsushita MN10200");
        m.insert(91, "picoJava");
        m.insert(92, "OpenRISC 32-bit embedded processor");
        m.insert(93, "ARC International ARCompact processor (old spelling/synonym: EM_ARC_A5)");
        m.insert(94, "Tensilica Xtensa Architecture");
        m.insert(95, "Alphamosaic VideoCore processor");
        m.insert(96, "Thompson Multimedia General Purpose Processor");
        m.insert(97, "National Semiconductor 32000 series");
        m.insert(98, "Tenor Network TPC processor");
        m.insert(99, "Trebia SNP 1000 processor");
        m.insert(100, "STMicroelectronics (www.st.com) ST200 microcontroller");
        m.insert(101, "Ubicom IP2xxx microcontroller family");
        m.insert(102, "MAX Processor");
        m.insert(103, "National Semiconductor CompactRISC microprocessor");
        m.insert(104, "Fujitsu F2MC16");
        m.insert(105, "Texas Instruments embedded microcontroller msp430");
        m.insert(106, "Analog Devices Blackfin (DSP) processor");
        m.insert(107, "S1C33 Family of Seiko Epson processors");
        m.insert(108, "Sharp embedded microprocessor");
        m.insert(109, "Arca RISC Microprocessor");
        m.insert(110, "Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University");
        m.insert(111, "eXcess: 16/32/64-bit configurable embedded CPU");
        m.insert(112, "Icera Semiconductor Inc. Deep Execution Processor");
        m.insert(113, "Altera Nios II soft-core processor");
        m.insert(114, "National Semiconductor CompactRISC CRX microprocessor");
        m.insert(115, "Motorola XGATE embedded processor");
        m.insert(116, "Infineon C16x/XC16x processor");
        m.insert(117, "Renesas M16C series microprocessors");
        m.insert(118, "Microchip Technology dsPIC30F Digital Signal Controller");
        m.insert(119, "Freescale Communication Engine RISC core");
        m.insert(120, "Renesas M32C series microprocessors");
        m.insert(131, "Altium TSK3000 core");
        m.insert(132, "Freescale RS08 embedded processor");
        m.insert(133, "Analog Devices SHARC family of 32-bit DSP processors");
        m.insert(134, "Cyan Technology eCOG2 microprocessor");
        m.insert(135, "Sunplus S+core7 RISC processor");
        m.insert(136, "New Japan Radio (NJR) 24-bit DSP Processor");
        m.insert(137, "Broadcom VideoCore III processor");
        m.insert(138, "RISC processor for Lattice FPGA architecture");
        m.insert(139, "Seiko Epson C17 family");
        m.insert(140, "The Texas Instruments TMS320C6000 DSP family");
        m.insert(141, "The Texas Instruments TMS320C2000 DSP family");
        m.insert(142, "The Texas Instruments TMS320C55x DSP family");
        m.insert(143, "Texas Instruments Application Specific RISC Processor, 32bit fetch");
        m.insert(144, "Texas Instruments Programmable Realtime Unit");
        m.insert(160, "STMicroelectronics 64bit VLIW Data Signal Processor");
        m.insert(161, "Cypress M8C microprocessor");
        m.insert(162, "Renesas R32C series microprocessors");
        m.insert(163, "NXP Semiconductors TriMedia architecture family");
        m.insert(164, "QUALCOMM DSP6 Processor");
        m.insert(165, "Intel 8051 and variants");
        m.insert(166, "STMicroelectronics STxP7x family of configurable and extensible RISC processors");
        m.insert(167, "Andes Technology compact code size embedded RISC processor family");
        m.insert(168, "Cyan Technology eCOG1X family");
        m.insert(168, "Cyan Technology eCOG1X family");
        m.insert(169, "Dallas Semiconductor MAXQ30 Core Micro-controllers");
        m.insert(170, "New Japan Radio (NJR) 16-bit DSP Processor");
        m.insert(171, "M2000 Reconfigurable RISC Microprocessor");
        m.insert(172, "Cray Inc. NV2 vector architecture");
        m.insert(173, "Renesas RX family");
        m.insert(174, "Imagination Technologies META processor architecture");
        m.insert(175, "MCST Elbrus general purpose hardware architecture");
        m.insert(176, "Cyan Technology eCOG16 family");
        m.insert(177, "National Semiconductor CompactRISC CR16 16-bit microprocessor");
        m.insert(178, "Freescale Extended Time Processing Unit");
        m.insert(179, "Infineon Technologies SLE9X core");
        m.insert(180, "Intel L10M");
        m.insert(181, "Intel K10M");
        m.insert(182, "Reserved for future Intel use");
        m.insert(183, "ARM 64-bit architecture (AARCH64)");
        m.insert(184, "Reserved for future ARM use");
        m.insert(185, "Atmel Corporation 32-bit microprocessor family");
        m.insert(186, "STMicroeletronics STM8 8-bit microcontroller");
        m.insert(187, "Tilera TILE64 multicore architecture family");
        m.insert(188, "Tilera TILEPro multicore architecture family");
        m.insert(189, "Xilinx MicroBlaze 32-bit RISC soft processor core");
        m.insert(190, "NVIDIA CUDA architecture");
        m.insert(191, "Tilera TILE-Gx multicore architecture family");
        m.insert(192, "CloudShield architecture family");
        m.insert(193, "KIPO-KAIST Core-A 1st generation processor family");
        m.insert(194, "KIPO-KAIST Core-A 2nd generation processor family");
        m.insert(195, "Synopsys ARCompact V2");
        m.insert(196, "Open8 8-bit RISC soft processor core");
        m.insert(197, "Renesas RL78 family");
        m.insert(198, "Broadcom VideoCore V processor");
        m.insert(199, "Renesas 78KOR family");
        m.insert(200, "Freescale 56800EX Digital Signal Controller (DSC)");
        m.insert(201, "Beyond BA1 CPU architecture");
        m.insert(202, "Beyond BA2 CPU architecture");
        m.insert(203, "XMOS xCORE processor family");
        m.insert(204, "Microchip 8-bit PIC(r) family");
        m.insert(205, "Reserved by Intel");
        m.insert(206, "Reserved by Intel");
        m.insert(207, "Reserved by Intel");
        m.insert(208, "Reserved by Intel");
        m.insert(209, "Reserved by Intel");
        m.insert(210, "KM211 KM32 32-bit processor");
        m.insert(211, "KM211 KMX32 32-bit processor");
        m.insert(212, "KM211 KMX16 16-bit processor");
        m.insert(213, "KM211 KMX8 8-bit processor");
        m.insert(214, "KM211 KVARC processor");
        m.insert(215, "Paneve CDP architecture family");
        m.insert(216, "Cognitive Smart Memory Processor");
        m.insert(217, "Bluechip Systems CoolEngine");
        m.insert(218, "Nanoradio Optimized RISC");
        m.insert(219, "CSR Kalimba architecture family");
        m.insert(220, "Zilog Z80");
        m.insert(221, "Controls and Data Services VISIUMcore processor");
        m.insert(222, "FTDI Chip FT32 high performance 32-bit RISC architecture");
        m.insert(223, "Moxie processor family");
        m.insert(224, "AMD GPU architecture");
        m.insert(243, "RISC-V");
        m
    };

    pub static ref P_TYPES: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "PT_NULL");
        m.insert(1, "PT_LOAD");
        m.insert(2, "PT_DYNAMIC");
        m.insert(3, "PT_INTERP");
        m.insert(4, "PT_NOTE");
        m.insert(5, "PT_SHLIB");
        m.insert(6, "PT_PHDR");
        m.insert(7, "Thread-local storage segment");
        m.insert(8, "Number of defined types");
        m.insert(0x60000000, "Start of OS-specific");
        m.insert(0x6474e550, "GCC .eh_frame_hdr segment");
        m.insert(0x6474e551, "Indicates stack executability");
        m.insert(0x6474e552, "Read-only after relocation");
        m.insert(0x6ffffffa, "Unkown");
        m.insert(0x6ffffffa, "Sun Specific segment");
        m.insert(0x6ffffffb, "Stack segment");
        m.insert(0x6fffffff, "Unknown");
        m.insert(0x6fffffff, "End of OS-specific");
        m.insert(0x70000000, "Start of processor-specific");
        m.insert(0x7fffffff, "End of processor-specific");
        m
    };

    pub static ref SH_TYPES: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "Section header table entry unused");
        m.insert(1, "Program data");
        m.insert(2, "Symbol table");
        m.insert(3, "String table");
        m.insert(4, "Relocation entries with addends");
        m.insert(5, "Symbol hash table");
        m.insert(6, "Dynamic linking information");
        m.insert(7, "Notes");
        m.insert(8, "Program space with no data (bss)");
        m.insert(9, "Relocation entries, no addends");
        m.insert(10, "Reserved");
        m.insert(11, "Dynamic linker symbol table");
        m.insert(14, "Array of constructors");
        m.insert(15, "Array of destructors");
        m.insert(16, "Array of pre-constructors");
        m.insert(17, "Section group");
        m.insert(18, "Extended section indeces");
        m.insert(19, "Number of defined types.");
        m.insert(0x60000000, "Start OS-specific.");
        m.insert(0x6ffffff5, "Object attributes. ");
        m.insert(0x6ffffff6, "GNU-style hash table. ");
        m.insert(0x6ffffff7, "Prelink library list");
        m.insert(0x6ffffff8, "Checksum for DSO content. ");
        m.insert(0x6ffffffa, "Sun-specific low bound.");
        m.insert(0x6ffffffa, "SHT_SUNW_move");
        m.insert(0x6ffffffb, "SHT_SUNW_COMDAT");
        m.insert(0x6ffffffc, "SHT_SUNW_syminfo");
        m.insert(0x6ffffffd, "Version definition section.");
        m.insert(0x6ffffffe, "Version needs section.");
        m.insert(0x6fffffff, "Version symbol table. ");
        m.insert(0x6fffffff, "Sun-specific high bound.");
        m.insert(0x6fffffff, "End OS-specific type");
        m.insert(0x70000000, "Start of processor-specific");
        m.insert(0x7fffffff, "End of processor-specific");
        m.insert(0x80000000, "Start of application-specific");
        m.insert(0x8fffffff, "End of application-specific");
        m
    };
}

pub static SH_FLAGS: &'static str = "w\tSHF_WRITE\t\tWritable
a\tSHF_ALLOC\t\tOccupies memory during execution
x\tSHF_EXECINSTR\t\tExecutable
m\tSHF_MERGE\t\tMight be merged
s\tSHF_STRINGS\t\tContains nul-terminated strings
i\tSHF_INFO_LINK\t\tsh_info' contains SHT index
l\tSHF_LINK_ORDER\t\tPreserve order after combining
n\tSHF_OS_NONCONFORMING\tNon-standard OS specific handling
g\tSHF_GROUP\t\tSection is member of a group.
t\tSHF_TLS\t\t\tSection hold thread-local data.
c\tSHF_COMPRESSED\t\tSection with compressed data.
O\tSHF_MASKOS\t\tOS-specific.
P\tSHF_MASKPROC\t\tProcessor-specific
o\tSHF_ORDERED\t\tSpecial ordering requirement
e\tSHF_EXCLUDE\t\tSection is excluded unless";

#[derive(Debug)]
pub enum Error {
    ByteCastError,
    EIdentParseError,
    EhdrParseError,
    PhdrNotRelevant,
    FileError(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::FileError(e)
    }
}

pub fn cast_u64(slice: &[u8], keep_endian: bool) -> Result<u64, Error> {
    assert!(slice.len() == 8);
    let mut result: u64 = 0;
    if keep_endian {
        for i in 0..8 {
            result += (slice[i] as u64) << 56 - i * 8;
        }
    } else {
        for i in 0..8 {
            result += (slice[i] as u64) << 0 + i * 8;
        }
    }
    Ok(result)
}

pub fn cast_u32(slice: &[u8], keep_endian: bool) -> Result<u32, Error> {
    assert!(slice.len() == 4);
    let mut result: u32 = 0;
    if keep_endian {
        for i in 0..4 {
            result += (slice[i] as u32) << 24 - i * 8;
        }
    } else {
        for i in 0..4 {
            result += (slice[i] as u32) << 0 + i * 8;
        }
    }
    Ok(result)
}

pub fn cast_u16(slice: &[u8], keep_endian: bool) -> Result<u16, Error> {
    assert!(slice.len() == 2);
    let mut result: u16 = 0;
    if keep_endian {
        for i in 0..2 {
            result += (slice[i] as u16) << 8 - i * 8;
        }
    } else {
        for i in 0..2 {
            result += (slice[i] as u16) << 0 + i * 8;
        }
    }
    Ok(result)
}

