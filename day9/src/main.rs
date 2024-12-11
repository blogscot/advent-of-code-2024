use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Block {
    id: Option<u32>,
    size: u32,
}

fn allocate(index: u32, size: u32) -> Block {
    assert!(index % 2 == 0);
    Block {
        id: Some(index / 2),
        size,
    }
}
fn reserve(size: u32) -> Block {
    Block { id: None, size }
}

fn is_file(block: &Block) -> bool {
    block.id.is_some()
}

fn _print_diskmap(diskmap: &VecDeque<Block>) {
    let mut s: String;
    for block in diskmap {
        if block.id.is_none() {
            s = ".".repeat(block.size as usize);
        } else {
            s = (block.id.unwrap().to_string()).repeat(block.size as usize);
        }
        print!("{}", s);
    }
    println!()
}

// Pop the next allocated block from the diskmap, discarding free blocks
fn pop_back(diskmap: &mut VecDeque<Block>) -> Option<Block> {
    if let Some(block) = diskmap.pop_back() {
        if block.id.is_none() {
            return pop_back(diskmap);
        } else {
            return Some(block);
        }
    }
    None
}

fn find_free_block_position(diskmap: &VecDeque<Block>, size: u32, limit: usize) -> Option<usize> {
    for (i, block) in diskmap.iter().enumerate() {
        if i >= limit {
            return None;
        }
        if block.id.is_none() && block.size >= size {
            return Some(i);
        }
    }
    None
}

fn remaining_free_blocks(diskmap: &VecDeque<Block>, before: usize) -> bool {
    (0..before).any(|i| diskmap[i].id.is_none())
}

fn checksum(compact: &VecDeque<Block>) -> u64 {
    compact
        .iter()
        .flat_map(|b| {
            std::iter::repeat(b.id.unwrap_or_default())
                .take(b.size as usize)
                .collect::<Vec<u32>>()
        })
        .enumerate()
        .map(|(i, d)| (i as u32 * d) as u64)
        .sum()
}

fn solve_part1(diskmap: &mut VecDeque<Block>) -> u64 {
    let mut compact: VecDeque<Block> = VecDeque::default();

    while !diskmap.is_empty() {
        let block = diskmap.pop_front().unwrap();
        if is_file(&block) {
            compact.push_back(block);
        } else {
            let mut free_block = block;
            let available_space = free_block.size;
            if let Some(mut last) = pop_back(diskmap) {
                match available_space.cmp(&last.size) {
                    std::cmp::Ordering::Equal => {
                        // free space matches allocated block size
                        // so no adjustments required
                        compact.push_back(last);
                    }
                    std::cmp::Ordering::Less => {
                        // free space is less than allocated block size
                        // so fill the free space with the allocated block
                        // and return the reduced block to the diskmap
                        free_block.id = last.id;
                        last.size -= available_space;
                        compact.push_back(free_block);
                        diskmap.push_back(last);
                    }
                    std::cmp::Ordering::Greater => {
                        // free space is greater than allocated block size
                        // so add allocated block size and reduce the free space
                        // block on the diskmap
                        free_block.id = last.id;
                        free_block.size = last.size;
                        compact.push_back(free_block);
                        diskmap.push_front(reserve(available_space - last.size));
                    }
                }
            }
        }
    }

    checksum(&compact)
}

fn solve_part2(diskmap: &mut VecDeque<Block>) -> u64 {
    let mut current_block_index = diskmap.len() - 1;
    while remaining_free_blocks(diskmap, current_block_index) {
        while diskmap[current_block_index].id.is_none() {
            current_block_index -= 1;
        }
        let allocated_block = diskmap[current_block_index];
        if let Some(free_index) =
            find_free_block_position(diskmap, allocated_block.size, current_block_index)
        {
            let free_block = diskmap[free_index];
            match free_block.size.cmp(&allocated_block.size) {
                std::cmp::Ordering::Equal => {
                    diskmap[free_index].id = allocated_block.id;
                    diskmap[current_block_index].id = None;
                }
                std::cmp::Ordering::Less => {
                    panic!("free block is smaller than allocated block");
                }
                std::cmp::Ordering::Greater => {
                    // move allocated block to free block
                    // mark allocated block as free
                    // create new free block with remaining free space
                    let extra_space = free_block.size - allocated_block.size;
                    diskmap[free_index].id = allocated_block.id;
                    diskmap[free_index].size = allocated_block.size;
                    diskmap[current_block_index].id = None;
                    diskmap.insert(free_index + 1, reserve(extra_space));
                }
            }
        }
        current_block_index -= 1;
    }
    checksum(diskmap)
}

fn main() {
    let digits: Vec<u32> = include_str!("puzzle.txt")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut diskmap: VecDeque<Block> = VecDeque::default();

    for (i, digit) in digits.iter().enumerate() {
        if i % 2 == 0 {
            diskmap.push_back(allocate(i as u32, *digit));
        } else {
            diskmap.push_back(reserve(*digit));
        }
    }

    println!("Part 1: {:?}", solve_part1(&mut diskmap.clone()));
    println!("Part 2: {:?}", solve_part2(&mut diskmap));
}
