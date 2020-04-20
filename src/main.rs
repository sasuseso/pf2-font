extern crate memmap;
extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::fs::File;
use std::env;
use std::os::raw::c_char;
use std::ffi::CStr;
use core::marker::PhantomData;
use std::str::from_utf8;

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
        Cursor::new(self.length).read_u32::<BigEndian>().unwrap() as usize
    }
}

#[derive(Clone, Debug, Default)]
struct Font {
    name: String,
    family: String,
    weight: String,
    point_size: u16,
    max_width: u16,
    max_height: u16,
    ascent: u16,
    descent: u16,
}

impl Font {
    fn new(head_ptr: *mut Section) -> Font {
        let font_info = Default::default();
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
                "NAME" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "FAMI" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "WEIG" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "SLAN" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "PTSZ" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "MAXW" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "MAXH" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "ASCE" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "DESC" => { unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); } },
                "CHIX" => {
                    eof = true;
                    unsafe { println!("{}, {:#x}", (*ptr).get_name(), (*ptr).get_length()); }
                    break
                },
                _ => break,
            }

            let addr = ptr as usize + unsafe { (*ptr).get_length() } as usize + 8;
            ptr = addr as *mut _;
        }
        font_info
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
