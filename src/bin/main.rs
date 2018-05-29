extern crate elftool;

fn main() {
    if let Ok(elf) = elftool::elf_file::parse_from_disk("/bin/ls") {
        elf.explain_all();
    };
}
