extern crate memmap;
extern crate byteorder;
use std::fs::File;
use std::env;
use std::ffi::CString;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct Section {
    name: [u8; 4],
    length: u32,
}

impl Section {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_length(&self) -> usize {
        self.length as usize
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct FileSection {
    header: Section,
    string: [u8; 4]
}

impl FileSection {
    fn verify_string(&self) -> bool {
        self.string == "PFF2".as_bytes()
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct NameSection {
    header: Section,
    font_name: CString,
}

impl NameSection {
    fn get_font_name(&self) -> String {
        self.font_name.to_str().unwrap().to_string()
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct FamilyNameSection {
    header: Section,
    family_name: CString,
}

impl FontWeightSection {
    fn get_family_name(&self) -> String {
        self.family_name.to_str().unwrap().to_string()
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct FontWeightSection {
    header: Section,
    weight: CString,
}

impl FontWeightSection {
    fn get_weight(&self) -> String {
        self.weight.to_str().unwrap().to_string()
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct FontSlantSection {
    header: Section,
    slant: CString,
}

impl FontSlantSection {
    fn get_slant(&self) -> String {
        self.slant.to_str().unwrap().to_string()
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct FontPointSizeSection {
    header: Section,
    point_size: u16,
}

impl FontPointSizeSection {
    fn get_point_size(&self) -> u16 {
        self.point_size
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct MaxFontWidthSection {
    header: Section,
    width: u16,
}

impl MaxFontWidthSection {
    fn get_width(&self) -> u16 {
        self.width
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct MaxFontHeightSection {
    header: Section,
    height: u16,
}

impl MaxFontHeightSection {
    fn get_height(&self) -> u16 {
        self.height
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct FontAscentSection {
    header: Section,
    ascent: u16,
}

impl FontAscentSection {
    fn get_ascent(&self) -> u16 {
        self.ascent
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
struct FontDescentSection {
    header: Section,
    descent: u16,
}

impl FontDescentSection {
    fn get_descent(&self) -> u16 {
        self.descent
    }
}

//#[repr(C, packed)]
//#[derive(Copy, Clone, Debug, Default)]
//struct 

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Only filename expected.");
    }

    let file = unsafe {
        memmap::MmapOptions::new()
        .map(&File::open(&args[1]).unwrap())
        .unwrap()
    };

    println!("{:?}", file);
}
