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

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum MachineType {
    EM_NONE = 0,
    EM_M32 = 1,
    EM_SPARC = 2,
    EM_386 = 3,
    EM_68K = 4,
    EM_88K = 5,
    // RESERVED	6	Reserved for future use
    EM_860 = 7,
    EM_MIPS = 8,
    EM_S370 = 9,
    EM_MIPS_RS3_LE = 10,
    // RESERVED	11-14	Reserved for future use
    EM_PARISC = 15,
    // RESERVED	16	Reserved for future use
    EM_VPP500 = 17,
    EM_SPARC32PLUS = 18,
    EM_960 = 19,
    EM_PPC = 20,
    EM_PPC64 = 21,
    // RESERVED	22-35	Reserved for future use
    EM_V800 = 36,
    EM_FR20 = 37,
    EM_RH32 = 38,
    EM_RCE = 39,
    EM_ARM = 40,
    EM_ALPHA = 41,
    EM_SH = 42,
    EM_SPARCV9 = 43,
    EM_TRICORE = 44,
    EM_ARC = 45,
    EM_H8_300 = 46,
    EM_H8_300H = 47,
    EM_H8S = 48,
    EM_H8_500 = 49,
    EM_IA_64 = 50,
    EM_MIPS_X = 51,
    EM_COLDFIRE = 52,
    EM_68HC12 = 53,
    EM_MMA = 54,
    EM_PCP = 55,
    EM_NCPU = 56,
    EM_NDR1 = 57,
    EM_STARCORE = 58,
    EM_ME16 = 59,
    EM_ST100 = 60,
    EM_TINYJ = 61,
    // Reserved	62-65	Reserved for future use
    EM_FX66 = 66,
    EM_ST9PLUS = 67,
    EM_ST7 = 68,
    EM_68HC16 = 69,
    EM_68HC11 = 70,
    EM_68HC08 = 71,
    EM_68HC05 = 72,
    EM_SVX = 73,
    EM_ST19 = 74,
    EM_VAX = 75,
    EM_CRIS = 76,
    EM_JAVELIN = 77,
    EM_FIREPATH = 78,
    EM_ZSP = 79,
    EM_MMIX = 80,
    EM_HUANY = 81,
    EM_PRISM = 82,
    // TODO: This will cause all reserved to get the same value, hopefully not a big problem
    RESERVED,
    OTHER,
}

impl MachineType {
    pub fn parse(orig_i: &[u8]) -> nom::IResult<&[u8], MachineType> {
        let i = orig_i;
        let (i, selector) = nom::number::streaming::le_u16(i)?;

        let enum_def = match selector {
            6 | 11..=14 | 16 | 22..=35 | 62..=65 => MachineType::RESERVED,
            x if x >= MachineType::OTHER as u16 => MachineType::OTHER,
            x => unsafe { std::mem::transmute(x as u16) },
        };

        Ok((i, enum_def))
    }
}

#[derive(Debug, PartialEq, Nom)]
#[nom(LittleEndian)]
pub struct ElfHeader<'a> {
    ident: ElfIdent<'a>,
    file_type: FileType,
    machine_type: MachineType,
}
