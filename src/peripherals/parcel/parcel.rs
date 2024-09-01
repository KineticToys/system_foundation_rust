use core::panic;
use std::{collections::LinkedList, iter::zip, mem::size_of, num::Wrapping};

use crate::enums::endian::Endian;

use super::parcel_error_type::ParcelErrorType;

pub struct Parcel<'a> {
    tx_buffer: Vec<u8>,
    rx_buffer: LinkedList<u8>,
    parcel_endian: Endian,

    tx_func: Box<dyn FnMut(&Vec<u8>) -> () + 'a>,
    rx_func: Box<dyn FnMut() -> Vec<u8> + 'a>,

    tx_buffer_phase: ParcelTxPhase,
}

impl<'a> Parcel<'a> {
    /// Create new Parcel instance.
    pub fn new<TxFuncT, RxFuncT>(parcel_endian: Endian, tx_func: TxFuncT, rx_func: RxFuncT) -> Self
    where
        TxFuncT: FnMut(&Vec<u8>) -> () + 'a,
        RxFuncT: FnMut() -> Vec<u8> + 'a,
    {
        return Self {
            tx_buffer: Vec::new(),
            rx_buffer: LinkedList::new(),
            parcel_endian: parcel_endian,
            tx_buffer_phase: ParcelTxPhase::Init,
            tx_func: Box::new(tx_func),
            rx_func: Box::new(rx_func),
        };
    }

    pub fn tx_clear(&mut self) {
        self.tx_buffer_phase = ParcelTxPhase::Init;
        self.tx_buffer.clear();
    }

    /// Write header to TX buffer.
    pub fn tx_write_header(&mut self) {
        if self.tx_buffer_phase != ParcelTxPhase::Init {
            panic!("TX buffer is not initialized.");
        }

        self.tx_buffer.push(0x55_u8);
        self.tx_buffer.push(0x55_u8);
        self.tx_buffer_phase = ParcelTxPhase::HeaderWritten;
    }

    /// Write topic to TX buffer.
    pub fn tx_write_topic(&mut self, topic: u16) {
        if self.tx_buffer_phase != ParcelTxPhase::HeaderWritten {
            panic!("Header is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => topic.to_be_bytes(),
            Endian::LittleEndian => topic.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);

        // Append two zero-bytes as placeholder for payload size.
        self.tx_write_bytes(&[0x00_u8, 0x00_u8]);

        self.tx_buffer_phase = ParcelTxPhase::TopicWritten;
    }

    /// Write i8 value to TX buffer.
    pub fn tx_write_i8(&mut self, value: i8) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write u8 value to TX buffer.
    pub fn tx_write_u8(&mut self, value: u8) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write i16 value to TX buffer.
    pub fn tx_write_i16(&mut self, value: i16) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write u16 value to TX buffer.
    pub fn tx_write_u16(&mut self, value: u16) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write i32 value to TX buffer.
    pub fn tx_write_i32(&mut self, value: i32) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write u32 value to TX buffer.
    pub fn tx_write_u32(&mut self, value: u32) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write i64 value to TX buffer.
    pub fn tx_write_i64(&mut self, value: i64) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write u64 value to TX buffer.
    pub fn tx_write_u64(&mut self, value: u64) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write f32 value to TX buffer.
    pub fn tx_write_f32(&mut self, value: f32) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write f64 value to TX buffer.
    pub fn tx_write_f64(&mut self, value: f64) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = match self.parcel_endian {
            Endian::BigEndian => value.to_be_bytes(),
            Endian::LittleEndian => value.to_le_bytes(),
        };
        self.tx_write_bytes(&bytes);
    }

    /// Write string value to TX buffer.
    pub fn tx_write_string(&mut self, value: String) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let bytes = value.as_bytes();
        self.tx_write_bytes(bytes);
    }

    /// Finalize the TX buffer.
    /// Writes the payload size at byte 4 and 5, and the checksum at the end.
    pub fn tx_finalize(&mut self) {
        if self.tx_buffer_phase != ParcelTxPhase::TopicWritten {
            panic!("Topic is not written to TX buffer yet.");
        }

        let payload_size = (self.tx_buffer.len() - 6) as u16;
        let checksum = Parcel::compute_checksum(&self.tx_buffer[6..]);
        let payload_size_bytes = match self.parcel_endian {
            Endian::BigEndian => payload_size.to_be_bytes(),
            Endian::LittleEndian => payload_size.to_le_bytes(),
        };

        self.tx_buffer[4] = payload_size_bytes[0];
        self.tx_buffer[5] = payload_size_bytes[1];
        self.tx_buffer.push(checksum);
        self.tx_buffer_phase = ParcelTxPhase::Finalized;
    }

    pub fn tx_send(&mut self) {
        if self.tx_buffer_phase != ParcelTxPhase::Finalized {
            panic!("TX buffer is not finalized.");
        }

        (self.tx_func)(&self.tx_buffer);
    }

    /// Write bytes to TX buffer.
    fn tx_write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.tx_buffer.push(*byte);
        }
    }

    /// Receive data via RX function.
    pub fn rx_receive(&mut self) {
        let rx_data = (self.rx_func)();
        for byte in rx_data {
            self.rx_buffer.push_back(byte);
        }
    }

    pub fn rx_read_frame(&mut self) -> Option<(u16, Vec<u8>)> {
        let mut frame_size: usize = 0;
        let mut frame_bytes: Vec<u8> = Vec::new();
        let mut payload_size_bytes: [u8; 2] = [0; 2];

        while self.rx_buffer.len() >= 7 {
            let mut index: usize = 0;
            let mut out_of_order: bool = false;

            // Pop bytes until correctly-formed metadata is found.
            for byte in self.rx_buffer.iter() {
                if index == 0 || index == 1 {
                    if *byte != 0x55_u8 {
                        out_of_order = true;
                        break;
                    }
                } else if index == 4 || index == 5 {
                    payload_size_bytes[index - 4] = *byte;
                } else if index == 6 {
                    break;
                }

                index += 1;
            }

            if out_of_order {
                self.rx_buffer.pop_front();
                continue;
            }

            let payload_size: u16 = match self.parcel_endian {
                Endian::BigEndian => u16::from_be_bytes(payload_size_bytes),
                Endian::LittleEndian => u16::from_le_bytes(payload_size_bytes),
            };

            frame_size = 6 + payload_size as usize + 1;

            // If there are less bytes than frame size, terminate the loop.
            if self.rx_buffer.len() < frame_size as usize {
                return None;
            }

            frame_bytes.resize(frame_size, 0);
            for (index, byte) in zip(0..frame_size, self.rx_buffer.iter()) {
                *frame_bytes.get_mut(index).unwrap() = *byte;
            }

            let received_checksum = *frame_bytes.last().unwrap();
            let computed_checksum =
                Parcel::compute_checksum(&frame_bytes[6..frame_bytes.len() - 1]);
            if received_checksum != computed_checksum {
                self.rx_buffer.pop_front();
                continue;
            }

            let topic_bytes: [u8; 2] = [*frame_bytes.get(2).unwrap(), *frame_bytes.get(3).unwrap()];

            let topic = match self.parcel_endian {
                Endian::BigEndian => u16::from_be_bytes(topic_bytes),
                Endian::LittleEndian => u16::from_le_bytes(topic_bytes),
            };

            let payload = Vec::from(&frame_bytes[6..frame_bytes.len() - 1]);
            for _ in 0..frame_size {
                self.rx_buffer.pop_front();
            }
            return Some((topic, payload));
        }

        return None;
    }

    pub fn rx_read_i8(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<i8, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<1>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => i8::from_be_bytes(bytes),
            Endian::LittleEndian => i8::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_u8(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<u8, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<1>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => u8::from_be_bytes(bytes),
            Endian::LittleEndian => u8::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_i16(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<i16, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<2>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => i16::from_be_bytes(bytes),
            Endian::LittleEndian => i16::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_u16(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<u16, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<2>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => u16::from_be_bytes(bytes),
            Endian::LittleEndian => u16::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_i32(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<i32, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<4>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => i32::from_be_bytes(bytes),
            Endian::LittleEndian => i32::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_u32(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<u32, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<4>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => u32::from_be_bytes(bytes),
            Endian::LittleEndian => u32::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_i64(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<i64, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<8>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => i64::from_be_bytes(bytes),
            Endian::LittleEndian => i64::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_u64(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<u64, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<8>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => u64::from_be_bytes(bytes),
            Endian::LittleEndian => u64::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_f32(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<f32, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<4>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => f32::from_be_bytes(bytes),
            Endian::LittleEndian => f32::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_f64(
        payload: &Vec<u8>,
        offset: usize,
        as_endian: Endian,
    ) -> Result<f64, ParcelErrorType> {
        let mut bytes = Parcel::rx_copy_bytes::<8>(payload, offset)?;
        return Ok(match as_endian {
            Endian::BigEndian => f64::from_be_bytes(bytes),
            Endian::LittleEndian => f64::from_le_bytes(bytes),
        });
    }

    pub fn rx_read_string(
        payload: &Vec<u8>,
        offset: usize,
        length: usize,
    ) -> Result<String, ParcelErrorType> {
        if offset + length > payload.len() {
            return Err(ParcelErrorType::OutOfBounds);
        }

        let string_bytes = Vec::from(&payload[offset..(offset + length)]);
        match String::from_utf8(string_bytes) {
            Ok(string) => return Ok(string),
            Err(_) => return Err(ParcelErrorType::InvalidData),
        };
    }

    fn rx_copy_bytes<const SIZE: usize>(
        payload: &Vec<u8>,
        offset: usize,
    ) -> Result<[u8; SIZE], ParcelErrorType> {
        if offset + SIZE > payload.len() {
            return Err(ParcelErrorType::OutOfBounds);
        }

        let mut bytes = [0_u8; SIZE];
        for i in 0..SIZE {
            bytes[i] = payload[offset + i];
        }
        return Ok(bytes);
    }

    /// Compute checksum of given bytes.
    fn compute_checksum(bytes: &[u8]) -> u8 {
        let mut checksum = Wrapping(0xFF_u8);
        for byte in bytes {
            let b = Wrapping(*byte);
            checksum = checksum ^ b;
        }

        return checksum.0;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ParcelTxPhase {
    Init,
    HeaderWritten,
    TopicWritten,
    Finalized,
}
