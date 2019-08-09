use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: rips <rom> <IPS patch>");
        return;
    }
    let rom_file = &args[1];
    let patch_file = &args[2];
    let mut rom = OpenOptions::new().write(true).open(rom_file)
        .expect("Failed to read rom file");
    let mut patch = File::open(patch_file).expect("Failed to read patch file");
    let mut buff: Vec<u8> = Vec::new();
    patch.read_to_end(&mut buff).unwrap();
    
    if &buff[..5] != "PATCH".as_bytes() {
        eprintln!("Patch is invalid format");
        return;
    }

    let mut buffer = &mut buff[5..];
    loop {
        if &buffer[..3] == "EOF".as_bytes() {
            break;
        }
        // get offset
        let offset = ((buffer[0] as u64) << 16) +
            ((buffer[1] as u64) <<8) +
            (buffer[2] as u64);

        buffer = &mut buffer[3..];
        // get size
        let size = ((buffer[0] as usize) << 8) + (buffer[1] as usize);
        buffer = &mut buffer[2..];

        // RLE encoded patch
        if size == 0 {
            let rle_size = ((buffer[0] as u16) << 8) + (buffer[1] as u16);
            buffer = &mut buffer[2..];

            let value = buffer[0];
            buffer = &mut buffer[1..];
            rom.seek(SeekFrom::Start(offset));
            let mut dat: Vec<u8> = Vec::new();
            for _i in 0..rle_size {
                dat.push(value);
            }
            rom.write_all(&dat).expect("Failed to write!");
        } else {
            // write chunk
            println!("offset: {} - size: {}", offset, size);
            rom.seek(SeekFrom::Start(offset));
            rom.write_all(&mut buffer[..size]).expect("Failed to write!");
            buffer = &mut buffer[size..];
        }
    }
    rom.flush();
}
