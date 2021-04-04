extern crate plain;
use std::env;
use std::fs::File;
use std::io::{Read, BufReader};
use std::mem::size_of;
use plain::Plain;
use std::convert::TryInto;

#[cfg(target_pointer_width = "64")]
type ElfN_Addr = u64;
#[cfg(target_pointer_width = "64")]
type ElfN_Off = u64;

trait Packed {
    fn size() -> usize;
    fn from_bytes(bytes: [u8]) -> Self;
    fn from_reader(r: BufReader<u8>) -> Self;
}

#[derive(Debug)]
#[repr(u8)]
enum ElfClass {
    ELFCLASSNONE = 0,
    ELFCLASS32 = 1,
    ELFCLASS64 = 2,
}
#[derive(Debug)]
#[repr(u8)]
enum ElfData {
    ELFDATANONE = 0,
    ELFDATA2LSB = 1,
    ELFDATA2MSB = 2,
}
#[derive(Debug)]
struct elf_ident {
    ei_magic: [u8; 4],
    ei_class: ElfClass,
    ei_data: ElfData,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8; 6],
    ei_nident: u8,
}
#[derive(Debug)]
struct elf_header {
    e_ident: elf_ident,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: ElfN_Addr,
    e_phoff: ElfN_Off,
    e_shoff: ElfN_Off,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

unsafe impl Plain for elf_header {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let binary = args.get(1).unwrap();
    let f = File::open(binary).unwrap();
    let mut a: [u8; std::mem::size_of::<elf_header>()] = [0; std::mem::size_of::<elf_header>()];
    let mut r = f.take(a.len().try_into().unwrap());
    r.read(&mut a).unwrap();
    let hdr: &elf_header = plain::from_bytes(&a).unwrap();
    println!("{:?}",hdr.e_ident.ei_class);
}
