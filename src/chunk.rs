enum OpCode {
    OpReturn,
}

struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    fn initChunk() -> Chunk {
        return Chunk(
            code: vec![],
        )
    }
}

