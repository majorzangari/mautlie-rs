use std::{array, sync::OnceLock};

use crate::state::game_constants;

const KNIGHT_MOVES: [u64; 64] = generate_knight_moves();
const KING_MOVES: [u64; 64] = generate_king_moves();

#[rustfmt::skip]
const ROOK_MAGICS: [u64; 64] = [
    0x8180024000805020, 0x40100440022004, 0x802a1000200080, 0x8180080010000480,
    0x1200100884204a00, 0x200100200090408, 0x400024c09009008, 0x80108000402300,
    0x800440026288, 0x220500400c210080, 0x110802000841002, 0x4003005000082100,
    0xd004808008005400, 0x8822001014080200, 0x4002000822000495, 0x8002c0802100,
    0x41010020c88000, 0x301010040008020, 0x2130020004100, 0x1010018201002,
    0x1820808004001800, 0x2010080110044060, 0x48208400b0010208, 0x20009108044,
    0x842180004001, 0x203200040100240, 0xe00080300080, 0x100080080081,
    0x45110100142800, 0x1002000a00283004, 0x430424400100908, 0x2611c200040081,
    0x24224003800080, 0x780c10e002204, 0x1812044052002080, 0x408080080803000,
    0x204000482800800, 0xa0a2c0080800600, 0x221003411000a00, 0xb0881008200005c,
    0x3015401024808000, 0x4033020140820020, 0x11002005130040, 0x10000804004040,
    0x4201000408010012, 0x210024001b0008, 0x220850440009, 0x100044008a0001,
    0x2c00800025400080, 0x850421084210200, 0x41000908a200180, 0x1238021000088180,
    0x24008000c008080, 0x102000910240200, 0x1e00810608100400, 0x100142a1040200,
    0x221001200c482, 0x2a2011109614082, 0x4200148704501, 0x136100e8900005,
    0x1200e008100402, 0x401000204000801, 0x4058a0300903824, 0x1100040060804d02,
];

#[rustfmt::skip]
const BISHOP_MAGICS: [u64; 64] = [
    0x2086a1800440088, 0x61480a0404003010, 0x892080200220002, 0x1028208020252000,
    0x202021004800400, 0x82084404000080, 0x220c04461a900080, 0x812020200820800,
    0x29041004110408, 0x8202820408120040, 0x83a0110404004000, 0x401440404824200,
    0x40140414600a0020, 0x19040201102a8000, 0x2880412382104100, 0x1408004108011184,
    0x120100c20020a40, 0x4484c4202041420, 0x4481008420040, 0x800280080a004000,
    0x808400a05000, 0x408080280900800, 0x1040880882012, 0x1d000088809010,
    0x18600440140110, 0x10082000082a2080, 0x2030008080020, 0x4004000a01a018,
    0x2209001201004001, 0x2210040804828400, 0x208220089030102, 0x10b2504004820800,
    0x900820400802a0, 0xc088014801100208, 0x1092240100100904, 0x20040101500900,
    0x40044100211100, 0xa09004201510100, 0x4001810500040c00, 0x8802008040a0a2,
    0x88111110c120500c, 0x412ac1008000208, 0x2041402000c00, 0xb028002011080800,
    0x8204a4000200, 0x801200881000184, 0x208380080820410, 0x48883100214040,
    0x29889010118000, 0x402a0841c200002, 0x1066420506882520, 0x4a40026084240274,
    0x40805012020006, 0x130d240d88120000, 0x4c081001121000, 0x8201820a0808244,
    0x200104410088800, 0xa000310308822004, 0x2021802252009000, 0x804822400208800,
    0x81000012220200, 0x82020061492, 0x10469a501b280101, 0x448100400440120,
];

#[derive(Default, Clone)]
struct MagicInfo {
    magic: u64,
    shift: u8,
    mask: u64,
    attacks: Vec<u64>,
}

static ROOK_MAGIC_INFO: OnceLock<[MagicInfo; 64]> = OnceLock::new();
static BISHOP_MAGIC_INFO: OnceLock<[MagicInfo; 64]> = OnceLock::new();

/// computes and returns the blocker mask for a rook on the given square
fn compute_rook_mask(square: usize) -> u64 {
    debug_assert!(square < 64);
    let rank = (square / 8);
    let file = (square % 8);
    let rank_bb = game_constants::RANK_1 << (rank * 8);
    let file_bb = game_constants::FILE_A << file;

    let mut mask = rank_bb | file_bb;

    if file != 7 {
        mask &= !game_constants::FILE_H;
    }
    if file != 0 {
        mask &= !game_constants::FILE_A;
    }
    if rank != 7 {
        mask &= !game_constants::RANK_8;
    }
    if rank != 0 {
        mask &= !game_constants::RANK_1;
    }
    mask &= !(1u64 << square);
    mask
}

/// computes and returns the blocker mask for a bishop on the given square
fn compute_bishop_mask(square: usize) -> u64 {
    debug_assert!(square < 64);
    let rank = (square / 8);
    let file = (square % 8);
    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut mask = 0;
    for (dr, df) in directions {
        let mut r = rank as isize + dr;
        let mut f = file as isize + df;
        while r > 0 && r < 7 && f > 0 && f < 7 {
            mask |= 1u64 << (r * 8 + f);
            r += dr;
            f += df;
        }
    }
    mask
}

/// computes the rook attacks for a given square and blocker configuration
fn compute_rook_attacks(square: usize, blockers: u64) -> u64 {
    debug_assert!(square < 64);
    let mut attacks = 0;
    let rank = square / 8;
    let file = square % 8;

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (dr, df) in directions {
        let mut r = rank as isize + dr;
        let mut f = file as isize + df;
        while (0..8).contains(&r) && (0..8).contains(&f) {
            let sq = (r as usize) * 8 + (f as usize);
            attacks |= 1u64 << sq;
            if (blockers & (1u64 << sq)) != 0 {
                break;
            }
            r += dr;
            f += df;
        }
    }

    attacks
}

/// computes the bishop attacks for a given square and blocker configuration
fn compute_bishop_attacks(square: usize, blockers: u64) -> u64 {
    debug_assert!(square < 64);
    let mut attacks = 0;
    let rank = square / 8;
    let file = square % 8;

    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    for (dr, df) in directions {
        let mut r = rank as isize + dr;
        let mut f = file as isize + df;
        while (0..8).contains(&r) && (0..8).contains(&f) {
            let sq = (r as usize) * 8 + (f as usize);
            attacks |= 1u64 << sq;
            if (blockers & (1u64 << sq)) != 0 {
                break;
            }
            r += dr;
            f += df;
        }
    }

    attacks
}

pub fn init_magic_info() {
    let mut rook_info: [MagicInfo; 64] = array::from_fn(|square| {
        let square_rook_magic = ROOK_MAGICS[square];
        let square_rook_mask = compute_rook_mask(square);
        let square_rook_shift = 64 - square_rook_mask.count_ones();
        let square_rook_attack_count = 1 << (64 - square_rook_shift);
        MagicInfo {
            magic: square_rook_magic,
            shift: square_rook_shift as u8,
            mask: square_rook_mask,
            attacks: vec![0; square_rook_attack_count],
        }
    });
    rook_info.iter_mut().enumerate().for_each(|(square, info)| {
        debug_assert!(info.attacks.len() == (1 << (64 - info.shift)));
        let mut mask = info.mask;
        let mut blockers: u64 = 0;
        loop {
            let index = blockers.wrapping_mul(info.magic) >> info.shift;
            info.attacks[index as usize] = compute_rook_attacks(square, blockers);
            blockers = blockers.wrapping_sub(mask) & mask;
            if blockers != 0 {
                break;
            }
        }
    });

    match ROOK_MAGIC_INFO.set(rook_info) {
        Ok(_) => {}
        Err(_) => panic!("Rook magic info already initialized"),
    }

    let mut bishop_info: [MagicInfo; 64] = array::from_fn(|square| {
        let square_bishop_magic = BISHOP_MAGICS[square];
        let square_bishop_mask = compute_bishop_mask(square);
        let square_bishop_shift = 64 - square_bishop_mask.count_ones();
        let square_bishop_attack_count = 1 << (64 - square_bishop_shift);
        MagicInfo {
            magic: square_bishop_magic,
            shift: square_bishop_shift as u8,
            mask: square_bishop_mask,
            attacks: vec![0; square_bishop_attack_count],
        }
    });
    bishop_info
        .iter_mut()
        .enumerate()
        .for_each(|(square, info)| {
            let mut mask = info.mask;
            let mut blockers: u64 = 0;
            loop {
                let index = blockers.wrapping_mul(info.magic) >> info.shift;
                info.attacks[index as usize] = compute_bishop_attacks(square, blockers);
                blockers = blockers.wrapping_sub(mask) & mask;
                if blockers != 0 {
                    break;
                }
            }
        });

    match BISHOP_MAGIC_INFO.set(bishop_info) {
        Ok(_) => {}
        Err(_) => panic!("Bishop magic info already initialized"),
    }
}

const fn generate_knight_moves() -> [u64; 64] {
    let mut out = [0; 64];

    let directions = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    let mut rank = 0;
    while rank < 8 {
        let mut file = 0;
        while file < 8 {
            let mut dir_index = 0;
            let mut attacks: u64 = 0;
            while dir_index < directions.len() {
                let (dr, df) = directions[dir_index];
                let new_rank = rank + dr;
                let new_file = file + df;

                if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                    let new_square_index = new_rank * 8 + new_file;
                    attacks |= 1 << new_square_index;
                }

                dir_index += 1;
            }
            out[(rank * 8 + file) as usize] = attacks;
            file += 1;
        }
        rank += 1;
    }

    out
}

const fn generate_king_moves() -> [u64; 64] {
    let mut out = [0; 64];

    let directions = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut rank = 0;
    while rank < 8 {
        let mut file = 0;
        while file < 8 {
            let mut dir_index = 0;
            let mut attacks: u64 = 0;
            while dir_index < directions.len() {
                let (dr, df) = directions[dir_index];
                let new_rank = rank + dr;
                let new_file = file + df;

                if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                    let new_square_index = new_rank * 8 + new_file;
                    attacks |= 1 << new_square_index;
                }

                dir_index += 1;
            }
            out[(rank * 8 + file) as usize] = attacks;
            file += 1;
        }
        rank += 1;
    }

    out
}

pub fn get_knight_moves_bb(square: u8, friendly_occ: u64) -> u64 {
    debug_assert!(square < 64);
    KNIGHT_MOVES[square as usize] & !friendly_occ
}

pub fn get_king_moves_bb(square: u8, friendly_occ: u64) -> u64 {
    debug_assert!(square < 64);
    KING_MOVES[square as usize] & !friendly_occ
}

pub fn get_bishop_moves_bb(square: u8, friendly_occ: u64, enemy_occ: u64) -> u64 {
    debug_assert!(square < 64);
    let info = &BISHOP_MAGIC_INFO
        .get()
        .expect("Bishop magic info not initialized")[square as usize];

    let blockers = (friendly_occ | enemy_occ) & info.mask;
    let index = blockers.wrapping_mul(info.magic) >> info.shift;

    debug_assert!(index < info.attacks.len() as u64);
    info.attacks[index as usize]
}

pub fn get_rook_moves_bb(square: u8, friendly_occ: u64, enemy_occ: u64) -> u64 {
    debug_assert!(square < 64);
    let info = &ROOK_MAGIC_INFO
        .get()
        .expect("Rook magic info not initialized")[square as usize];

    let blockers = (friendly_occ | enemy_occ) & info.mask;
    let index = blockers.wrapping_mul(info.magic) >> info.shift;

    debug_assert!(index < info.attacks.len() as u64);
    info.attacks[index as usize]
}
