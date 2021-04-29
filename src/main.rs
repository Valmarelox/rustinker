extern crate plain;
extern crate memmap;
use memmap::MmapOptions;
use memoize::memoize;
use std::env;
use std::fs::File;
use std::io::{Read, BufReader};
use std::mem::size_of;
use plain::Plain;
use std::convert::TryInto;
use buffered_offset_reader::{BufOffsetReader, OffsetReadMut};

type Elf32_Addr = u32;
type Elf32_Off = u32;
type Elf64_Addr = u64;
type Elf64_Off = u64;

#[cfg(target_pointer_width = "64")]
type ElfN_Addr = Elf64_Addr;
#[cfg(target_pointer_width = "64")]
type ElfN_Off = Elf64_Off;

trait Packed {
    fn size() -> usize;
    fn from_bytes(bytes: [u8]) -> Self;
    fn from_reader(r: BufReader<u8>) -> Self;
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum ElfClass {
    ELFCLASSNONE = 0,
    ELFCLASS32 = 32,
    ELFCLASS64 = 64,
}
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum ElfData {
    ELFDATANONE = 0,
    ELFDATA2LSB = 1,
    ELFDATA2MSB = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[repr(C)]
#[derive(Debug,Copy, Clone)]
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
#[cfg(target_pointer_width = "32")]
struct Phdr {
    p_type: u32,
    p_offset: Elf32_Off,
    p_vaddr: Elf32_Addr,
    p_paddr: Elf32_Addr

}
#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: Elf64_Off,
    p_vaddr: Elf64_Addr,
    p_paddr: Elf64_Addr,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

struct Elf {
    f: BufOffsetReader<File>
}

impl Elf {
    fn create(f: File) -> Elf {
        Elf { f: BufOffsetReader::new(f) }
    }

    fn header(&mut self) -> elf_header {
        assert_eq!(std::mem::size_of::<elf_ident>(), 16);
        let mut a: [u8; std::mem::size_of::<elf_header>()] = [0; std::mem::size_of::<elf_header>()];
        self.f.read_at(&mut a, 0).unwrap();
        let hdr = plain::from_bytes::<elf_header>(&a).unwrap();
        assert_eq!(hdr.e_ident.ei_magic, [ 0x7f,  b'E', b'L', b'F']);
        *hdr
    }

    fn phdrs(&mut self) -> Vec<Phdr> {
        let mut a: [u8; std::mem::size_of::<Phdr>()] = [0; std::mem::size_of::<Phdr>()];
        let mut v: Vec<Phdr> = vec!();
        let hdr = self.header();
        for i in 1..hdr.e_phnum {
            let off: u64 = (i * hdr.e_phentsize).try_into().unwrap();
            println!("Reading phdr at {}", off);
            self.f.read_at(&mut a, hdr.e_phoff + off).unwrap();
            v.push(*plain::from_bytes::<Phdr>(&a).unwrap());
        }
        v
    }

}
unsafe impl Plain for elf_ident {}
unsafe impl Plain for elf_header {}
unsafe impl Plain for Phdr {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let binary = args.get(1).unwrap();
    let f = File::open(binary).unwrap();
    let mut elf = Elf::create(f);
    println!("{:?}", elf.header());
    println!("{:?}", elf.header());
    let phdrs = elf.phdrs();
    let f = File::open(binary).unwrap();
    for phdr in phdrs {
        let map = unsafe {
            MmapOptions::new().offset(phdr.p_offset).map(&f).unwrap()
        };
    }
    println!("{:?}", elf.phdrs());
}
