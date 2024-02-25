use std::fs::File;

mod huffman;
mod compression;
mod file;
mod process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let option = &args[1];
    let file_path = args[2].as_str();

    if option == "-c" {
        let source = match std::fs::read(&args[2]) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error : {}", err);
                std::process::exit(1);
            }
        };

        let (prefix_code_table, compressed_data, total_bits) = process::compress(&source);
        let write_res = process::write_comp_file("output.bin", compressed_data, prefix_code_table, total_bits);

        if let Err(ref e) = write_res {
            eprintln!("Error while writing compressed file: {}", e);
        }

        let old_size: usize = source.len();
        let new_size: usize = std::fs::metadata("output.bin").unwrap().len() as usize;

        println!("Original Size: {} bytes", old_size);
        println!("Compressed Size: {} bytes", new_size);
    } else if option == "-d" {
        let file = File::open(file_path);
        if let Err(ref e) = file {
            eprintln!("Error while opening file: {}", e);
            std::process::exit(1);
        }

        let decomp_res = process::decompress(file.unwrap());

        if let Err(ref e) = decomp_res {
            eprintln!("Error while decompressing file: {}", e);
            std::process::exit(1);
        }

        let write_res = process::write_dec_file("output.txt", &decomp_res.unwrap());

        if let Err(ref e) = write_res {
            eprintln!("Error while writing decompressed file: {}", e);
            std::process::exit(1);
        }

        let old_size = std::fs::metadata(&args[2]).unwrap().len() as usize;
        let new_size = std::fs::metadata("output.txt").unwrap().len() as usize;

        println!("Original Size: {} bytes", old_size);
        println!("Decompressed Size: {} bytes", new_size);
    } else {
        eprintln!("Invalid option: {}", option);
        std::process::exit(1);
    }
}
