use std::num::NonZeroU8;

use arrayvec::ArrayVec;
use bevy::ecs::resource::Resource;

#[derive(Default, Resource)]
pub struct Board([[u8; 9]; 9]);

impl Board {
    pub fn new(value: [[u8; 9]; 9]) -> Result<Self, InvariantsError> {
        for (y, row) in value.iter().enumerate() {
            let mut visited_row_numbers = ArrayVec::<u8, 9>::new();
            for (x, &cell) in row.iter().enumerate() {
                if cell != 0 && !(1..=9).contains(&cell) {
                    return Err(InvariantsError::ValueOutOfRange(ValueOutOfRangeError {
                        x: x as u8,
                        y: y as u8,
                        value: cell,
                    }));
                }
                if cell != 0 {
                    if visited_row_numbers.contains(&cell) {
                        return Err(InvariantsError::DuplicateValue(DuplicateValueKind::Row {
                            row: x as u8,
                            value: cell,
                        }));
                    }
                    visited_row_numbers.push(cell);
                }
            }
        }

        for x in 0..9 {
            let mut visited_column_numbers = ArrayVec::<u8, 9>::new();
            for y in 0..9 {
                let cell = value[y][x];
                if cell != 0 {
                    if visited_column_numbers.contains(&cell) {
                        return Err(InvariantsError::DuplicateValue(
                            DuplicateValueKind::Column {
                                column: x as u8,
                                value: cell,
                            },
                        ));
                    }
                    visited_column_numbers.push(cell);
                }
            }
        }

        Ok(Self(value))
    }

    pub fn as_ref(&self) -> &[[u8; 9]; 9] {
        &self.0
    }

    pub fn set(
        &mut self,
        x: u8,
        y: u8,
        value: Option<NonZeroU8>,
    ) -> Result<(), ValueOutOfRangeError> {
        let Some(value) = value else {
            self.0[y as usize][x as usize] = 0;
            return Ok(());
        };
        let value = value.get();
        if !(1..=9).contains(&value) {
            return Err(ValueOutOfRangeError { x, y, value });
        }
        self.0[y as usize][x as usize] = value;
        Ok(())
    }

    pub fn validate_invariants(&self) -> Result<(), InvariantsError> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InvariantsError {
    #[error(transparent)]
    ValueOutOfRange(#[from] ValueOutOfRangeError),
    #[error("Duplicate value: {0}")]
    DuplicateValue(DuplicateValueKind),
}

#[derive(Debug, thiserror::Error)]
#[error("Value ({x}, {y}) is out of range: {value} (must be in `1..=9`)")]
pub struct ValueOutOfRangeError {
    pub x: u8,
    pub y: u8,
    pub value: u8,
}

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum DuplicateValueKind {
    #[display("Row {row} has duplicate value {value}")]
    Row { row: u8, value: u8 },
    #[display("Column {column} has duplicate value {value}")]
    Column { column: u8, value: u8 },
    #[display("Box ({box_row}, {box_column}) has duplicate value {value}")]
    Box {
        box_row: u8,
        box_column: u8,
        value: u8,
    },
}
