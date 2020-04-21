extern crate memmap;
extern crate byteorder;
extern crate bit_vec;
use byteorder::{ByteOrder, BigEndian};
use std::io::Cursor;
use std::fs::File;
use std::env;
use std::os::raw::c_char;
use std::ffi::CStr;
use core::marker::PhantomData;
use std::str::from_utf8;
use bit_vec::BitVec;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct Section {
    name: [u8; 4],
    length: [u8; 4],
}

impl Section {
    fn get_name(&self) -> &str {
        from_utf8(&(self.name[..])).unwrap()
    }

    fn get_length(&self) -> usize {
        BigEndian::read_u32(&(self.length)) as usize
    }
}

#[derive(Clone, Debug, Default)]
struct Font {
    name: String,
    family: String,
    weight: String,
    slant: String,
    point_size: u16,
    max_width: u16,
    max_height: u16,
    ascent: u16,
    descent: u16,
}

impl Font {
    fn new(head_ptr: *const Section) -> Self {
        let mut font_info: Self = Default::default();
        let mut eof = false;
        let mut ptr = head_ptr;
        //unsafe {
            //println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());
            //let addr = ptr as usize + (*ptr).get_length() as usize + 8;
            //println!("{:#x}", addr);
            //ptr = addr as *mut _;
            //println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());
        //}
        while !eof {
            match unsafe { (*ptr).get_name() } {
                "FILE" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());
                        let s = from_utf8(&(*((ptr as usize + 8) as *const [u8; 4]))[..]).unwrap();
                        if s == "PFF2" {
                            println!("Signiture OK!");
                        }
                    }
                },
                "NAME" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let s = CStr::from_ptr((ptr as usize + 8) as *const c_char).to_str().unwrap();
                        font_info.name = s.to_string();
                    }
                },
                "FAMI" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let s = CStr::from_ptr((ptr as usize + 8) as *const c_char).to_str().unwrap();
                        font_info.family = s.to_string();
                    }
                },
                "WEIG" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let s = CStr::from_ptr((ptr as usize + 8) as *const c_char).to_str().unwrap();
                        font_info.weight = s.to_string();
                    }
                },
                "SLAN" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let s = CStr::from_ptr((ptr as usize + 8) as *const c_char).to_str().unwrap();
                        font_info.slant = s.to_string();
                    }
                },
                "PTSZ" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());
                        let n = BigEndian::read_u16(&(*((ptr as usize + 8) as *const [u8; 2])));
                        font_info.point_size = n;
                    }
                },
                "MAXW" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let n = BigEndian::read_u16(&(*((ptr as usize + 8) as *const [u8; 2])));
                        font_info.max_width = n;
                    }
                },
                "MAXH" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let n = BigEndian::read_u16(&(*((ptr as usize + 8) as *const [u8; 2])));
                        font_info.max_height = n;
                    }
                },
                "ASCE" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let n = BigEndian::read_u16(&(*((ptr as usize + 8) as *const [u8; 2])));
                        font_info.ascent = n;
                    }
                },
                "DESC" => {
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let n = BigEndian::read_u16(&(*((ptr as usize + 8) as *const [u8; 2])));
                        font_info.descent = n;
                    }
                },
                "CHIX" => {
                    eof = true;
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let chix = *((ptr as usize + 8) as *const ChixEntry);
                        println!("{:#x}, {:#x}, {:#x}",
                                 chix.get_code_point(),
                                 chix.get_storage_flags(),
                                 chix.get_offset()
                                 );

                        let char_addr: usize = head_ptr as usize + chix.get_offset() as usize;
                        let char_data = *(char_addr as *const CharData);
                        println!("{:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:?}",
                                 char_data.get_width(),
                                 char_data.get_height(),
                                 char_data.get_x_offset(),
                                 char_data.get_y_offset(),
                                 char_data.get_dev_width(),
                                 char_addr,
                                 char_data.get_bitmap()
                                 );

                        let s = char_addr as *const u16;
                        println!("{:#x}", *s);

                        let bitmap = BitVec::from_bytes(char_data.get_bitmap());
                        println!("{:?}", bitmap);

                        for (i, d) in bitmap.iter().enumerate() {
                            if i != 0 && i % 16 == 0 {
                                println!();
                            }

                            print!("{}", match d {
                                true => "#",
                                false => " ",
                            });
                        }

                        //let n = BigEndian::read_u32(&(*((ptr as usize + 8) as *const [u8; 4]))[..]);
                        //println!("{:#x}", n);
                        //let n = *((ptr as usize + 12) as *const u8);
                        //println!("{:#x}", n);
                        //let n = BigEndian::read_u32(&(*((ptr as usize + 13) as *const [u8; 4]))[..]);
                        //println!("{:#x}", n);
                    }
                    break
                },
                _ => break,
            }

            let addr = ptr as usize + unsafe { (*ptr).get_length() } as usize + 8;
            ptr = addr as *mut _;
        }
        println!("{:?}", font_info);
        font_info
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
struct ChixEntry {
    code_point: [u8; 4],
    storage_flags: u8,
    offset: [u8; 4],
}

impl ChixEntry {
    fn get_code_point(&self) -> u32 {
        BigEndian::read_u32(&(self.code_point)[..])
    }

    fn get_storage_flags(&self) -> u8 {
        self.storage_flags
    }

    fn get_offset(&self) -> u32 {
        BigEndian::read_u32(&(self.offset)[..])
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CharData {
    width: [u8; 2],
    height: [u8; 2],
    x_offset: [u8; 2],
    y_offset: [u8; 2],
    dev_width: [u8; 2],
    bitmap_data: u8
}

impl CharData {
    fn get_width(&self) -> u16 {
        BigEndian::read_u16(&(self.width))
    }

    fn get_height(&self) -> u16 {
        BigEndian::read_u16(&(self.height))
    }

    fn get_x_offset(&self) -> u16 {
        BigEndian::read_u16(&(self.x_offset))
    }

    fn get_y_offset(&self) -> u16 {
        BigEndian::read_u16(&(self.y_offset))
    }

    fn get_dev_width(&self) -> u16 {
        BigEndian::read_u16(&(self.dev_width))
    }

    fn get_bitmap(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(&(self.bitmap_data) as *const u8,
                ((self.get_width() * self.get_height() + 7) / 8) as usize)
        }
    }
}
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

    let section = Font::new(file.as_ptr() as *mut Section);

    println!("{:?}", file);
}
