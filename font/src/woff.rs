extern crate miniz_oxide;

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
struct WOFFHeader {
    signature: Vec<u8>,
    flavor: Vec<u8>,
    length: Vec<u8>,
    num_tables: Vec<u8>,
    reserved: Vec<u8>,
    total_sfnt_size: Vec<u8>,
    major_version: Vec<u8>,
    minor_version: Vec<u8>,
    meta_offset: Vec<u8>,
    meta_length: Vec<u8>,
    meta_orig_length: Vec<u8>,
    priv_offset: Vec<u8>,
    priv_length: Vec<u8>,
}

impl WOFFHeader {
    pub fn new<'a>(rs: &'a mut ReadStream) -> Option<Self> {
        let signature = rs.read_vec(4)?;
        let flavor = rs.read_vec(4)?;
        let length = rs.read_vec(4)?;
        let num_tables = rs.read_vec(2)?;
        let reserved = rs.read_vec(2)?;
        let total_sfnt_size = rs.read_vec(4)?;
        let major_version = rs.read_vec(2)?;
        let minor_version = rs.read_vec(2)?;
        let meta_offset = rs.read_vec(4)?;
        let meta_length = rs.read_vec(4)?;
        let meta_orig_length = rs.read_vec(4)?;
        let priv_offset = rs.read_vec(4)?;
        let priv_length = rs.read_vec(4)?;
        Some(WOFFHeader {
            signature,
            flavor,
            length,
            num_tables,
            reserved,
            total_sfnt_size,
            major_version,
            minor_version,
            meta_offset,
            meta_length,
            meta_orig_length,
            priv_offset,
            priv_length,
        })
    }
}

#[derive(Debug, Clone)]
struct TableDirectory {
    tag: Vec<u8>,
    offset: Vec<u8>,
    comp_length: Vec<u8>,
    orig_length: Vec<u8>,
    orig_checksum: Vec<u8>,
}

impl TableDirectory {
    pub fn new<'a>(rs: &'a mut ReadStream) -> Option<Self> {
        let tag = rs.read_vec(4)?;
        let offset = rs.read_vec(4)?;
        let comp_length = rs.read_vec(4)?;
        let orig_length = rs.read_vec(4)?;
        let orig_checksum = rs.read_vec(4)?;
        Some(TableDirectory {
            tag,
            offset,
            comp_length,
            orig_length,
            orig_checksum,
        })
    }
}

struct ReadStream<'a> {
    offset: usize,
    data: &'a [u8],
}

impl<'a> ReadStream<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ReadStream {
            offset: 0,
            data,
        }
    }

    pub fn read(&mut self, step: usize) -> Option<&[u8]> {
        let start = self.offset;
        self.offset += step;
        let end = self.offset;

        self.data.get(start..end)
    }

    pub fn read_vec(&mut self, step: usize) -> Option<Vec<u8>> {
        self.read(step).and_then(|r| Some(r.to_vec()))
    }
}

struct WriteStream<'a> {
    offset: usize,
    data: &'a mut Vec<u8>,
}

impl<'a> WriteStream<'a> {
    pub fn new(data: &'a mut Vec<u8>) -> Self {
        WriteStream {
            offset: 0,
            data,
        }
    }
    pub fn write(&mut self, data: Vec<u8>) {
        for item in data.iter() {
            self.data.push(*item);
            self.offset += 1;
        }
    }
}

struct BufferOffset(pub usize);

impl Deref for BufferOffset {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BufferOffset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BufferOffset {
    pub fn to_be(&self) -> Vec<u8> {
        convert_u32_to_u8s_be(self.0 as u32).to_vec()
    }
}

impl From<&BufferOffset> for Vec<u8> {
    fn from(offset: &BufferOffset) -> Self {
        convert_u32_to_u8s_be(offset.0 as u32).to_vec()
    }
}


fn convert_u16_to_u8s_be(x: u16) -> [u8; 2] {
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    [b3, b4]
}

fn convert_u32_to_u8s_be(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}

fn convert_be_to_u16(f: Vec<u8>) -> u16 {
    let n0 = f.get(0).unwrap_or(&0u8);
    let n1 = f.get(1).unwrap_or(&0u8);
    u16::from_be_bytes([*n0, *n1])
}

fn convert_be_to_u32(f: Vec<u8>) -> u32 {
    let n0 = f.get(0).unwrap_or(&0u8);
    let n1 = f.get(1).unwrap_or(&0u8);
    let n2 = f.get(2).unwrap_or(&0u8);
    let n3 = f.get(3).unwrap_or(&0u8);
    u32::from_be_bytes([*n0, *n1, *n2, *n3])
}

pub fn decompress_woff(input: &[u8]) -> Option<Vec<u8>> {
    let mut output = Vec::<u8>::new();
    let mut ws = WriteStream::new(&mut output);
    let mut rs = ReadStream::new(input);
    let header: WOFFHeader = WOFFHeader::new(&mut rs)?;

    ws.write(header.flavor);
    ws.write(header.num_tables.clone());

    let num_tables: u16 = convert_be_to_u16(header.num_tables);
    let mut temp = num_tables.clone();
    let mut entry_selector = 0u16;
    let mut search_range = 16u16;

    loop {
        if temp <= 1 { break; }
        temp = temp >> 1;
        entry_selector += 1;
        search_range = search_range << 1;
    }

    let range_shift = convert_u16_to_u8s_be(num_tables.clone() * 16 - search_range);
    let search_range = convert_u16_to_u8s_be(search_range);
    let entry_selector = convert_u16_to_u8s_be(entry_selector);

    ws.write(search_range.to_vec());
    ws.write(entry_selector.to_vec());
    ws.write(range_shift.to_vec());

    let mut table_directory_map = Vec::<TableDirectory>::new();
    let mut offset = ws.offset;
    let mut out_offset_map = Vec::<BufferOffset>::new();

    for _i in 0..num_tables.clone() {
        let table_directory = TableDirectory::new(&mut rs)?;
        table_directory_map.push(table_directory);
        offset += 4 * 4;
    }

    for table_directory in table_directory_map.iter() {
        let TableDirectory { tag, offset: _, comp_length: _, orig_length, orig_checksum } = table_directory;
        ws.write(tag.to_vec());
        ws.write(orig_checksum.to_vec());
        ws.write(BufferOffset(offset.clone()).to_be());
        ws.write(orig_length.to_vec());
        out_offset_map.push(BufferOffset(offset.clone()));
        let orig_length = convert_be_to_u32(table_directory.orig_length.clone()) as usize;
        offset += orig_length;
        if offset % 4 != 0 {
            offset += 4 - (offset % 4);
        }
    }

    out_offset_map.reserve(0);

    for table_directory in table_directory_map {
        let TableDirectory { tag: _, offset, comp_length, orig_length, orig_checksum: _ } = table_directory;
        let comp_length = convert_be_to_u32(comp_length.to_vec()) as usize;
        let orig_length = convert_be_to_u32(orig_length.to_vec()) as usize;
        rs.offset = convert_be_to_u32(offset.to_vec()) as usize;
        let mut data = rs.read_vec(comp_length)?;
        if comp_length != orig_length.clone() {
            data = {
                let result = miniz_oxide::inflate::decompress_to_vec_zlib(&data);
                if result.is_ok() {
                    Some(result.unwrap())
                } else {
                    None
                }
            }?;
        }
        let out_offset = out_offset_map.pop().unwrap();
        ws.offset = out_offset.0.clone();
        ws.write(data);
        let offset = out_offset.0 + orig_length;

        let mut padding = 0usize;

        if offset % 4 != 0 {
            padding = 4 - offset % 4
        }
        let mut padding_vec = Vec::<u8>::new();
        for _ in 0..padding {
            padding_vec.push(0u8);
        }

        ws.write(padding_vec);
    }

    Some(output)
}

#[cfg(test)]
mod test {
    use crate::woff::decompress_woff;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test() {
        let file = include_bytes!("../test/zc2016.woff") as &[u8];
        let result = decompress_woff(file).unwrap();
        // let result = file;
        let mut file = File::create("./out.ttf").unwrap();
        file.write(result.as_slice()).unwrap();
        println!("ok, {:?}", result.len());
    }
}