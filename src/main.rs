// //mod args;
mod chunk;
use chunk::Chunk;
mod chunk_type;
use chunk_type::*;
use std::str::FromStr;
// use chunk_type::*;
// //mod commands;
mod png;
use png::Png;

// pub type Error = Box<dyn std::error::Error>;
// pub type Result<T> = std::result::Result<T, Error>;

fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk, String> {

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let data: Vec<u8> = data.bytes().collect();

    Ok(Chunk::new(chunk_type, data))
}

fn main()  { 
    let vec_chunks = vec![
        chunk_from_strings("FrSt", "I am the first chunk").unwrap(),
        chunk_from_strings("miDl", "I am another chunk").unwrap(),
        chunk_from_strings("LASt", "I am the last chunk").unwrap(),
    ];

    // println!("{:?}", Png::from_chunks(vec_chunks));
    let chunk_bytes: Vec<u8> = vec_chunks.into_iter().flat_map(|chunk| chunk.as_bytes()).collect();
    let bytes: Vec<u8> = Png::STANDARD_HEADER.iter().chain(chunk_bytes.iter()).copied().collect();
    println!("{:?}", bytes);

    let png_instance = Png::try_from(bytes.as_ref());
    println!("{:#?}", png_instance);

    println!("{:?}", chunk_bytes);
}
