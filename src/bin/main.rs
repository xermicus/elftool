extern crate elftool;

use std::env;

#[derive(Default)]
struct Options {
    all: bool,
    help: bool,
    ehdr: bool,
    phdr: bool,
    shdr: bool,
    file: String
}

fn parse_args() -> Options {
    let mut opts = Options::default();
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        opts.help = true;
        return opts
    }

    for arg in args {
        match arg.as_ref() {
            "-a" => opts.all = true,
            "-e" => opts.ehdr = true,
            "-p" => opts.phdr = true,
            "-s" => opts.shdr = true,
            "-h" => opts.help = true,
            _ => opts.file = arg
        }
    }
    opts
}

fn help() {
    println!("Usage:\telftool [opts] <file>
Opts:\t-h\tdisplay this help
\t-a\tdisplay the Elf Header, Program Header Table and Section Header Table
\t-e\tdisplay the ELF Header
\t-p\tdisplay the Program Header Table
\t-s\tdisplay the Section Header Table");
}

fn main() {
    let opts = parse_args();
    if opts.help {
        help();
    } else {
         if let Ok(elf) = elftool::elf_file::parse_from_disk(&opts.file) {
             if opts.all { elf.explain_all(); return }
             if opts.ehdr { elf.explain_ehdr() }
             if opts.phdr { elf.explain_phdr() }
             if opts.shdr { elf.explain_shdr() }
         };
    };
}
