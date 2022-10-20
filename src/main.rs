use std::fs::File;
use std::io::{Seek, Read, Write};
use std::path::{Path, PathBuf};
use binrw::{binrw, BinReaderExt, NullString, BinWriterExt};
use binrw::io::SeekFrom;
use serde::{Serialize, Deserialize};



#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub card_pointer: u64,

    pub part: u32,
    pub unk1: u32,
    pub medal_type: u32,

    #[brw(pad_before = 4)]
    #[serde(skip)]
    pub letter_pointer: u64,

    pub unk2: i32,
    pub unk3: i32,
    pub unk4: i32,
    pub unk5: i32,

    #[serde(skip)]
    pub sfx1_pointer: u64,
    #[serde(skip)]
    pub sfx2_pointer: u64,
    #[serde(skip)]
    pub sfx3_pointer: u64,
    #[serde(skip)]
    pub sfx4_pointer: u64,
    
    #[brw(pad_before = 8)]
    #[serde(skip)]
    pub character_pointer: u64,

    #[brw(pad_before = 8)]
    pub unlock_condition: u32,
    pub unk6: u32,
    pub cost: u32,
    
    #[brw(pad_before = 12)]
    #[serde(skip)]
    pub card_detail_pointer: u64,

    #[brw(pad_after = 4)]
    pub index: u32,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub card_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub letter: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sfx1: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sfx2: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sfx3: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sfx4: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub character: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub card_detail: String
}   

#[binrw]
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomCard {
    pub unk1: u16,
    pub unk2: u16,
    pub version: u16,
    pub unk3: u16,
    pub entry_count: u16,
    pub unk4: u16,
    
    pub unk5: u16,
    pub unk6: u16,

    #[brw(pad_before = 4)]
    #[br(count = entry_count)]
    entries: Vec<Entry>
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = Path::new(&args[1]);

    let new_extension = match path.extension().unwrap().to_str().unwrap() {
        "binary" => "json",
        "json" => "binary",
        _ => "binary",
    };
    
    //let start = std::time::Instant::now();

    match new_extension {
        "json" => {
            let mut file = File::open(path).unwrap();

            let json_path = args
            .get(2)
            .map(PathBuf::from)
            .unwrap_or_else(|| path.with_extension(new_extension));

            serialize(&mut file, json_path.to_str().unwrap());
        }

        "binary" => {
            let args: Vec<String> = std::env::args().collect();
            let json_path = Path::new(&args[1]);
            let mut json_file = File::open(json_path).unwrap();

            let binary_path = args
            .get(2)
            .map(PathBuf::from)
            .unwrap_or_else(|| json_path.with_extension(new_extension));


            deserialize(&mut json_file, binary_path.to_str().unwrap());
        }
    
        _ => unreachable!(),
    }


    //println!("Completed operation in {:?}", start.elapsed());
    
}


fn serialize(file: &mut File, path: &str) {
    let mut custom_card = file.read_le::<CustomCard>().unwrap();

    for (pointer_offset, entry) in custom_card.entries.iter_mut().enumerate().map(|(i, e)| (((0x90 * i + 0x14) as u64, e))) {
        if entry.card_pointer != 0 {
            file.seek(SeekFrom::Start(pointer_offset as u64)).unwrap();
            file.seek(SeekFrom::Current(entry.card_pointer as i64)).unwrap();
            entry.card_id = file.read_be::<NullString>().unwrap().to_string();
        } else {
            entry.card_id = String::from("");
        }

        if entry.letter_pointer != 0 {
            file.seek(SeekFrom::Start((pointer_offset + 0x18) as u64)).unwrap();
            file.seek(SeekFrom::Current(entry.letter_pointer as i64)).unwrap();
            entry.letter = file.read_be::<NullString>().unwrap().to_string();
        } else {
            entry.letter = String::from("");
        }

        if entry.sfx1_pointer != 0 {
            file.seek(SeekFrom::Start((pointer_offset + 0x30) as u64)).unwrap();
            file.seek(SeekFrom::Current(entry.sfx1_pointer as i64)).unwrap();
            entry.sfx1 = file.read_be::<NullString>().unwrap().to_string();
        } else {
            entry.sfx1 = String::from("");
        }

        if entry.sfx2_pointer != 0 {
            file.seek(SeekFrom::Start((pointer_offset + 0x38) as u64)).unwrap();
            file.seek(SeekFrom::Current(entry.sfx2_pointer as i64)).unwrap();
            entry.sfx2 = file.read_be::<NullString>().unwrap().to_string();
        } else {
            entry.sfx2 = String::from("");
        }

        if entry.sfx3_pointer != 0 {
            file.seek(SeekFrom::Start((pointer_offset + 0x40) as u64)).unwrap();
            file.seek(SeekFrom::Current(entry.sfx3_pointer as i64)).unwrap();
            entry.sfx3 = file.read_be::<NullString>().unwrap().to_string();
        } else {
            entry.sfx3 = String::from("");
        }

        if entry.sfx4_pointer != 0 {
            file.seek(SeekFrom::Start((pointer_offset + 0x48) as u64)).unwrap();
            file.seek(SeekFrom::Current(entry.sfx4_pointer as i64)).unwrap();
            entry.sfx4 = file.read_be::<NullString>().unwrap().to_string();
        } else {
            entry.sfx4 = String::from("");
        }

        if entry.character_pointer != 0 {
            file.seek(SeekFrom::Start((pointer_offset + 0x58) as u64)).unwrap();
            file.seek(SeekFrom::Current(entry.character_pointer as i64)).unwrap();
            entry.character = file.read_be::<NullString>().unwrap().to_string();
        } else {
            entry.character = String::from("");
        }

        if entry.card_detail_pointer != 0 {
            file.seek(SeekFrom::Start((pointer_offset + 0x80) as u64)).unwrap();
            file.seek(SeekFrom::Current(entry.card_detail_pointer as i64)).unwrap();
            entry.card_detail = file.read_be::<NullString>().unwrap().to_string();
        } else {
            entry.card_detail = String::from("");
        }
    }

    let json = serde_json::to_string_pretty(&custom_card).unwrap();
    std::fs::write(path, json).unwrap();
}


fn deserialize(json_file: &mut File, output_path: &str) {
    let mut json = String::new();
    json_file.read_to_string(&mut json).unwrap();

    let mut custom_card: CustomCard = serde_json::from_str(&json).unwrap();
    
    custom_card.entry_count = custom_card.entries.len() as u16;

    let mut file = File::create(output_path).unwrap();
    file.write_le(&custom_card).unwrap();

    for (pointer_offset, entry) in custom_card.entries.iter().enumerate().map(|(i, e)| (((0x90 * i + 0x14) as u64, e))) {
        if entry.card_id != "" {
            file.seek(SeekFrom::End(0)).unwrap();
            let string_pos = tell(&mut file);
            file.write_be(&NullString::from(entry.card_id.clone())).unwrap();
            
            align(&mut file, 0x8, string_pos);
            
            let new_pointer = string_pos - pointer_offset as u64;
            file.seek(SeekFrom::Start(pointer_offset as u64)).unwrap();
            file.write_le(&new_pointer).unwrap();
        }

        if entry.letter != "" {
            file.seek(SeekFrom::End(0)).unwrap();
            let string_pos = tell(&mut file);
            file.write_be(&NullString::from(entry.letter.clone())).unwrap();
            
            align(&mut file, 0x8, string_pos);
        
            file.seek(SeekFrom::Start((pointer_offset + 0x18) as u64)).unwrap();
            file.write_le(&(string_pos - &pointer_offset - 0x18)).unwrap();
        }

        if entry.sfx1 != "" {
            file.seek(SeekFrom::End(0)).unwrap();
            let string_pos = tell(&mut file);
            file.write_be(&NullString::from(entry.sfx1.clone())).unwrap();
            
            align(&mut file, 0x8, string_pos);
        
            file.seek(SeekFrom::Start((pointer_offset + 0x30) as u64)).unwrap();
            file.write_le(&(string_pos - &pointer_offset - 0x30)).unwrap();
        }

        if entry.sfx2 != "" {
            file.seek(SeekFrom::End(0)).unwrap();
            let string_pos = tell(&mut file);
            file.write_be(&NullString::from(entry.sfx2.clone())).unwrap();
            
            align(&mut file, 0x8, string_pos);
        
            file.seek(SeekFrom::Start((pointer_offset + 0x38) as u64)).unwrap();
            file.write_le(&(string_pos - &pointer_offset - 0x38)).unwrap();
        }

        if entry.sfx3 != "" {
            file.seek(SeekFrom::End(0)).unwrap();
            let string_pos = tell(&mut file);
            file.write_be(&NullString::from(entry.sfx3.clone())).unwrap();
            
            align(&mut file, 0x8, string_pos);
        
            file.seek(SeekFrom::Start((pointer_offset + 0x40) as u64)).unwrap();
            file.write_le(&(string_pos - &pointer_offset - 0x40)).unwrap();
        }

        if entry.sfx4 != "" {
            file.seek(SeekFrom::End(0)).unwrap();
            let string_pos = tell(&mut file);
            file.write_be(&NullString::from(entry.sfx4.clone())).unwrap();
            
            align(&mut file, 0x8, string_pos);
        
            file.seek(SeekFrom::Start((pointer_offset + 0x48) as u64)).unwrap();
            file.write_le(&(string_pos - &pointer_offset - 0x48)).unwrap();
        }

        if entry.character != "" {
            file.seek(SeekFrom::End(0)).unwrap();
            let string_pos = tell(&mut file);
            file.write_be(&NullString::from(entry.character.clone())).unwrap();
            
            align(&mut file, 0x8, string_pos);
        
            file.seek(SeekFrom::Start((pointer_offset + 0x58) as u64)).unwrap();
            file.write_le(&(string_pos - &pointer_offset - 0x58)).unwrap();
        }

        if entry.card_detail != "" {
            file.seek(SeekFrom::End(0)).unwrap();
            let string_pos = tell(&mut file);
            file.write_be(&NullString::from(entry.card_detail.clone())).unwrap();
            
            align(&mut file, 0x8, string_pos);
        
            file.seek(SeekFrom::Start((pointer_offset + 0x80) as u64)).unwrap();
            file.write_le(&(string_pos - &pointer_offset - 0x80)).unwrap();
        }

    }
}


fn align(file: &mut File, align: u64, string_pos: u64) {
    let pos = file.seek(SeekFrom::Current(0)).unwrap() - string_pos;
    let padding = align - (pos % align);
    if padding != align {
        file.write(&vec![0; padding as usize]).unwrap();
    }
}

fn tell(file: &mut File) -> u64 {
    return file.seek(SeekFrom::Current(0)).unwrap();
}