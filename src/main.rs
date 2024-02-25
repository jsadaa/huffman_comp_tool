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

    // if the option is -c, compress the file
    if option == "-c" {
        let source = match std::fs::read_to_string(&args[2]) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error : {}", err);
                std::process::exit(1);
            }
        };

        let (huff_codes, compressed_data, total_bits) = process::compress(&source);
        let write_res = process::write_comp_file("output.bin", compressed_data, huff_codes, total_bits);

        if let Err(ref e) = write_res {
            eprintln!("Error while writing compressed file: {}", e);
        }

        let old_size = source.len();
        let new_size = std::fs::metadata("output.bin").unwrap().len() as usize;

        println!("Original Size: {} bytes", old_size);
        println!("Compressed Size: {} bytes", new_size);
    } else if option == "-d" {
        let decomp_res = process::decompress(file_path);

        if let Err(ref e) = decomp_res {
            eprintln!("Error while decompressing file: {}", e);
            std::process::exit(1);
        }

        let write_res = process::write_dec_file("output.pdf", &decomp_res.unwrap());

        if let Err(ref e) = write_res {
            eprintln!("Error while writing decompressed file: {}", e);
            std::process::exit(1);
        }

        let old_size = std::fs::metadata(&args[2]).unwrap().len() as usize;
        let new_size = std::fs::metadata("output.pdf").unwrap().len() as usize;

        println!("Original Size: {} bytes", old_size);
        println!("Decompressed Size: {} bytes", new_size);
    } else {
        eprintln!("Invalid option: {}", option);
        std::process::exit(1);
    }
}
