use super::point_group::identify_point_group;
use crate::base::error::MoyoError;
use crate::base::operation::Operations;
use crate::base::transformation::Transformation;
use crate::data::hall_symbol_database::{HallNumber, Number, HALL_SYMBOL_DATABASE};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Setting {
    HallNumber(HallNumber),
    /// The setting of the smallest Hall number
    Spglib,
    /// Unique axis b, cell choice 1 for monoclinic, hexagonal axes for rhombohedral, and origin choice 2 for centrosymmetric space groups
    Standard,
}

impl Setting {
    pub fn hall_numbers(&self) -> Vec<HallNumber> {
        match self {
            Setting::HallNumber(hall_number) => vec![*hall_number],
            Setting::Spglib => vec![
                1, 2, 3, 6, 9, 18, 21, 30, 39, 57, 60, 63, 72, 81, 90, 108, 109, 112, 115, 116,
                119, 122, 123, 124, 125, 128, 134, 137, 143, 149, 155, 161, 164, 170, 173, 176,
                182, 185, 191, 197, 203, 209, 212, 215, 218, 221, 227, 228, 230, 233, 239, 245,
                251, 257, 263, 266, 269, 275, 278, 284, 290, 292, 298, 304, 310, 313, 316, 322,
                334, 335, 337, 338, 341, 343, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358,
                359, 361, 363, 364, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377,
                378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393,
                394, 395, 396, 397, 398, 399, 400, 401, 402, 404, 406, 407, 408, 410, 412, 413,
                414, 416, 418, 419, 420, 422, 424, 425, 426, 428, 430, 431, 432, 433, 435, 436,
                438, 439, 440, 441, 442, 443, 444, 446, 447, 448, 449, 450, 452, 454, 455, 456,
                457, 458, 460, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474,
                475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490,
                491, 492, 493, 494, 495, 497, 498, 500, 501, 502, 503, 504, 505, 506, 507, 508,
                509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 520, 521, 523, 524, 525, 527,
                529, 530,
            ],
            Setting::Standard => vec![
                1, 2, 3, 6, 9, 18, 21, 30, 39, 57, 60, 63, 72, 81, 90, 108, 109, 112, 115, 116,
                119, 122, 123, 124, 125, 128, 134, 137, 143, 149, 155, 161, 164, 170, 173, 176,
                182, 185, 191, 197, 203, 209, 212, 215, 218, 221, 227, 229, 230, 234, 239, 245,
                251, 257, 263, 266, 269, 275, 279, 284, 290, 292, 298, 304, 310, 313, 316, 323,
                334, 336, 337, 338, 341, 343, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358,
                360, 362, 363, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377,
                378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393,
                394, 395, 396, 397, 398, 399, 400, 401, 403, 405, 406, 407, 409, 411, 412, 413,
                415, 417, 418, 419, 421, 423, 424, 425, 427, 429, 430, 431, 432, 433, 435, 436,
                438, 439, 440, 441, 442, 443, 444, 446, 447, 448, 449, 450, 452, 454, 455, 456,
                457, 458, 460, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474,
                475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490,
                491, 492, 493, 494, 496, 497, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508,
                509, 510, 511, 512, 513, 514, 515, 516, 517, 519, 520, 522, 523, 524, 526, 528,
                529, 530,
            ],
        }
    }
}

#[derive(Debug)]
pub struct SpaceGroup {
    pub prim_operations: Operations,
    pub number: Number,
    pub hall_number: HallNumber,
    /// Transformation to the representative for `hall_number`
    pub transformation: Transformation,
}

pub fn identify_space_group(
    prim_operations: Operations,
    setting: Setting,
) -> Result<SpaceGroup, MoyoError> {
    let point_group = identify_point_group(&prim_operations.rotations)?;
    let hall_number_candidates: Vec<HallNumber> = setting
        .hall_numbers()
        .iter()
        .filter_map(|hall_number| unimplemented!())
        .collect();

    unimplemented!()
}
