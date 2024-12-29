use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct DiskMap(Vec<MapItem>);

impl FromStr for DiskMap {
    type Err = ParseDiskMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let size =
                    c.to_digit(10)
                        .ok_or_else(|| ParseDiskMapError::ParseIntError)? as u8;

                if i % 2 == 0 {
                    Ok(MapItem::File { size })
                } else {
                    Ok(MapItem::Free { size })
                }
            })
            .collect::<Result<Vec<MapItem>, ParseDiskMapError>>()?;

        Ok(DiskMap(items))
    }
}

#[derive(Debug)]
enum ParseDiskMapError {
    ParseIntError,
}

#[derive(Debug)]
enum MapItem {
    File { size: u8 },
    Free { size: u8 },
}

type FileID = u16;
const FREE_BLOCK: u16 = FileID::MAX;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let disk_map: DiskMap = input.parse().unwrap();
    log::debug!("{:?}", disk_map);

    let num_files = disk_map
        .0
        .iter()
        .filter(|item| match item {
            MapItem::File { size: _ } => true,
            MapItem::Free { size: _ } => false,
        })
        .count();

    assert!(num_files < FREE_BLOCK as usize);

    let mut disk = create_disk(disk_map);
    let mut disk_copy = disk.clone();
    compact_files(&mut disk);

    let filesystem_checksum: u64 = checksum(disk);

    println!("{}", filesystem_checksum);
}

fn checksum(disk: Vec<u16>) -> u64 {
    disk.into_iter()
        .enumerate()
        .map(|(block_id, file_id)| {
            if file_id == FREE_BLOCK {
                0
            } else {
                block_id as u64 * file_id as u64
            }
        })
        .sum()
}

fn compact_files(disk: &mut Vec<FileID>) {
    let mut head_index = 0;
    let mut tail_index = disk.len() - 1;

    while head_index < tail_index {
        if disk[head_index] != FREE_BLOCK {
            head_index += 1;
            continue;
        }

        if disk[tail_index] == FREE_BLOCK {
            tail_index -= 1;
            continue;
        }

        disk.swap(head_index, tail_index);
        head_index += 1;
        tail_index -= 1;
    }
}

fn create_disk(disk_map: DiskMap) -> Vec<FileID> {
    let disk_size: usize = disk_map
        .0
        .iter()
        .map(|item| match item {
            MapItem::File { size } => *size,
            MapItem::Free { size } => *size,
        } as usize)
        .sum();

    let mut disk: Vec<FileID> = vec![FREE_BLOCK; disk_size];
    let mut file_id = 0;
    let mut block_index = 0;
    for item in disk_map.0.iter() {
        match item {
            MapItem::File { size } => {
                for i in 0..*size as usize {
                    disk[block_index + i] = file_id
                }
                file_id += 1;
                block_index += *size as usize;
            }
            MapItem::Free { size } => {
                for i in 0..*size as usize {
                    disk[block_index + i] = FREE_BLOCK
                }
                block_index += *size as usize;
            }
        }
    }
    disk
}
