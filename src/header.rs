extern crate nom_derive;

use nom_derive::Nom;

#[derive(Debug, PartialEq, Nom)]
#[nom(LittleEndian)]
struct ElfIdent<'a> {
    #[nom(Tag(b"\x7FELF"))]
    magic: &'a [u8],
    class: ElfClass,
    data_encoding: ElfDataEncoding,
    version: ElfFileVersion,
    os_abi: ElfOsAbi,
    abi_version: u8,
    #[nom(Take = "7")]
    padding: &'a [u8],
}

#[derive(Debug, PartialEq, Nom)]
#[nom(LittleEndian)]
#[repr(u8)]
enum ElfClass {
    NONE,
    CLASS32,
    CLASS64,
}

#[derive(Debug, PartialEq, Nom)]
#[nom(LittleEndian)]
#[repr(u8)]
enum ElfDataEncoding {
    NONE,
    LSB,
    MSB,
}

#[derive(Debug, PartialEq, Nom)]
#[nom(LittleEndian)]
#[repr(u8)]
enum ElfFileVersion {
    NONE,
    VALID,
}

#[derive(Debug, PartialEq, Nom)]
#[nom(LittleEndian)]
#[repr(u8)]
enum ElfOsAbi {
    NONE,
    HPUX,
    NETBSD,
    LINUX,
    SOLARIS,
    AIX,
    IRIX,
    FREEBSD,
    TRU64,
    MODESTO,
    OPENBSD,
    OPENVMS,
    NSK,
}

#[derive(Debug, PartialEq, Nom)]
#[nom(LittleEndian)]
#[repr(u16)]
enum FileType {
    NONE = 0,
    REL = 1,
    EXEC = 2,
    DYN = 3,
    CORE = 4,
    LOPROC = 0xFF00,
    HIPROC = 0xFFFF,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum MachineType {
    NONE = 0,
    M32 = 1,
    SPARC = 2,
    I386 = 3,
    M68K = 4,
    M88K = 5,
    I860 = 7,
    MIPS = 8,
    MIPS_RS4_BE = 10,
    RESERVED1 = 11,
    RESERVED2 = 12,
    RESERVED3 = 13,
    RESERVED4 = 14,
    RESERVED5 = 15,
    RESERVED6 = 16,
    OTHER,
}

impl MachineType {
    pub fn parse(orig_i: &[u8]) -> nom::IResult<&[u8], MachineType> {
        let i = orig_i;
        let (i, selector) = nom::number::streaming::le_u16(i)?;

        match selector {
            0 => (),
        }

        if selector >= MachineType::OTHER as u16 {
            return Ok((i, MachineType::OTHER));
        }

        let x: MachineType = unsafe { std::mem::transmute(selector as u16) };
        Ok((i, x))
    }
}

#[derive(Debug, PartialEq, Nom)]
#[nom(LittleEndian)]
pub struct ElfHeader<'a> {
    ident: ElfIdent<'a>,
    // file_type: FileType,
    // machine_type: MachineType,
}
