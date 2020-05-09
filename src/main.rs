extern crate memmap;
extern crate byteorder;
extern crate bit_vec;
use byteorder::{ByteOrder, BigEndian};
//use std::io::Cursor;
use std::fs::File;
use std::env;
use std::os::raw::c_char;
use std::ffi::CStr;
//use core::marker::PhantomData;
use std::str::from_utf8;
use std::slice;
use bit_vec::BitVec;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct Section {
    data: [u8; 8]
}

impl Section {
    fn get_name(&self) -> &str {
        from_utf8(&(self.data[..4])).unwrap()
    }

    fn get_length(&self) -> usize {
        BigEndian::read_u32(&(self.data)[4..]) as usize
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

                    //println!("{:?}", chix.chix[0]);
                    unsafe {
                        println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length());

                        let chix = *((ptr as usize + 8 + (9 * 0x41)) as *const ChixEntry);
                        println!("code_point: {:#x}\noffset: {:#x}\nflags: {:#x}",
                                 chix.get_code_point(),
                                 chix.get_offset(),
                                 chix.get_storage_flags());
                        let data = (chix.get_offset() + head_ptr as usize) as *const CharData;
                        println!("width: {:#x}, height: {:#x}\nx_off: {}, y_off: {}\ndev_width: {}\nbitmap: [len: {}] {:?}",
                                 (*data).get_width(), (*data).get_height(),
                                 (*data).get_x_offset(), (*data).get_y_offset(),
                                 (*data).get_dev_width(), (*data).get_bitmap().len(),
                                 (*data).get_bitmap());
                        let bitmap = BitVec::from_bytes((*data).get_bitmap());

                        println!("\n");
                        for (i, b) in bitmap.iter().enumerate() {
                            print!("{}", match b {
                                true => "#",
                                false => " "
                            });
                            if (i + 1) % (*data).get_width() as usize == 0 {
                                println!()
                            }
                        }
                        println!("\n");
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

#[repr(packed, C)]
#[derive(Copy, Clone, Debug, Default)]
struct ChixEntry {
    entry: [u8; 4 + 1 + 4]
}

impl ChixEntry {
    fn get_code_point(&self) -> u32 {
        BigEndian::read_u32(&(self.entry)[..4])
    }

    fn get_storage_flags(&self) -> u8 {
        self.entry[4]
    }

    fn get_offset(&self) -> usize {
        BigEndian::read_u32(&(self.entry)[5..]) as usize
    }
}

#[repr(packed, C)]
#[derive(Debug, Clone, Copy)]
struct CharData {
    width: [u8; 2],
    height: [u8; 2],
    x_offset: [u8; 2],
    y_offset: [u8; 2],
    dev_width: [u8; 2],
}

impl CharData {
    fn get_width(&self) -> u16 {
        BigEndian::read_u16(&(self.width))
    }

    fn get_height(&self) -> u16 {
        BigEndian::read_u16(&(self.height))
    }

    fn get_x_offset(&self) -> i16 {
        BigEndian::read_i16(&(self.x_offset))
    }

    fn get_y_offset(&self) -> i16 {
        BigEndian::read_i16(&(self.y_offset))
    }

    fn get_dev_width(&self) -> i16 {
        BigEndian::read_i16(&(self.dev_width))
    }

    fn get_bitmap(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((self as *const Self as usize + 10) as *const u8,
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
    println!("{:#b}", 1 << 7);
}
