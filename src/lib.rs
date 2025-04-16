#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_int, c_void};

#[link(name = "lzav", kind = "static")]
unsafe extern "C"
{
  /// Function returns buffer size required for LZAV compression.
  ///
  /// #### Parameters
  ///
  /// - `uncompressed_length`: The length of the source data to be compressed.
  ///
  /// #### Returns
  ///
  /// The required allocation size for destination compression buffer.
  /// Always a positive value.
  pub fn compressBound(uncompressed_length: i32) -> i32;

  /// Function returns buffer size required for the higher-ratio LZAV compression.
  ///
  /// #### Parameters
  ///
  /// - `uncompressed_length`: The length of the source data to be compressed.
  ///
  /// #### Returns
  ///
  /// The required allocation size for destination compression buffer.
  /// Always a positive value.
  pub fn compressHighBound(uncompressed_length: i32) -> i32;

  /// LZAV compression function, with external buffer option.
  ///
  /// Function performs in-memory data compression using the LZAV compression
  /// algorithm and stream format. The function produces a "raw" compressed data,
  /// without a header containing data length nor identifier nor checksum.
  ///
  /// Note that compression algorithm and its output on the same source data may
  /// differ between LZAV versions, and may differ between big- and little-endian
  /// systems. However, the decompression of a compressed data produced by any
  /// prior compressor version will remain possible.
  ///
  /// #### Parameters
  ///
  /// - `source`: (Input) data pointer, can be 0 if `sourceLength` equals 0.
  ///   Address alignment is unimportant.
  /// - `destination`: (Output) buffer pointer where the compressed data will be written.
  ///   The allocated size should be at least `compressBound()` bytes large.
  ///   Should be different from `source`.
  /// - `source_length`: Source data length, in bytes.
  /// - `destination_length`: Destination buffer's capacity, in bytes.
  /// - `external_buffer`: External buffer to use for hash-table. Set to 0 for the function
  ///   to manage memory itself (via standard `malloc`). Supplying a pre-allocated buffer
  ///   is useful if compression is performed frequently: this reduces memory allocation
  ///   overhead and fragmentation. Note that the access to the supplied buffer is not
  ///   implicitly thread-safe. Buffer's address must be aligned to 32 bits.
  /// - `external_buffer_length`: The capacity of the `external_buffer`, in bytes, should be
  ///   a power-of-2 value. Set to 0 if `external_buffer` is 0. The capacity should not be
  ///   lesser than 4 x `sourceLength`, and not greater than 1 MiB. Same `externalBufferLength`
  ///   value can be used for any smaller source data. This value does not affect the compression
  ///   algorithm: the `external_buffer` simply will not be used if it is too small.
  ///
  /// #### Returns
  ///
  /// The length of compressed data, in bytes.
  /// Returns 0 if `sourceLength` is less or equal to 0, or if `destinationLength` is too small,
  /// or if not enough memory.
  pub fn compress(
    source: *const c_void,
    source_length: i32,
    destination: *mut c_void,
    destination_length: i32,
    external_buffer: *mut c_void,
    external_buffer_length: i32,
  ) -> i32;

  /// Default LZAV compression function.
  ///
  /// Function performs in-memory data compression using the LZAV compression
  /// algorithm, with the default settings.
  ///
  /// See the `compress()` function for a more detailed description.
  ///
  /// #### Parameters
  ///
  /// - `source`: (Input) data pointer.
  /// - `destination`: (Output) buffer pointer for the compressed data.
  ///   The allocated size should be at least `compressBound()` bytes large.
  /// - `source_length`: Source data length, in bytes.
  /// - `destination_length`: Destination buffer's capacity, in bytes.
  ///
  /// #### Returns
  ///
  /// The length of compressed data, in bytes.
  /// Returns 0 if `sourceLength` is less or equal to 0, or if `destinationLength` is too small,
  /// or if not enough memory.
  pub fn compressDefault(
    source: *const c_void,
    source_length: i32,
    destination: *mut c_void,
    destination_length: i32,
  ) -> i32;

  /// Higher-ratio LZAV compression function (much slower).
  ///
  /// Function performs in-memory data compression using the higher-ratio LZAV
  /// compression algorithm.
  ///
  /// #### Parameters
  ///
  /// - `source`: (Input) data pointer.
  /// - `destination`: (Output) buffer pointer for the compressed data.
  ///   The allocated size should be at least `compressBoundHigh()` bytes large.
  /// - `source_length`: Source data length, in bytes.
  /// - `destination_length`: Destination buffer's capacity, in bytes.
  ///
  /// #### Returns
  ///
  /// The length of compressed data, in bytes.
  /// Returns 0 if `sourceLength` is less or equal to 0, or if `destinationLength` is too small,
  /// or if not enough memory.
  pub fn compressHigh(
    source: *const c_void,
    source_length: i32,
    destination: *mut c_void,
    destination_length: i32,
  ) -> i32;

  /// LZAV decompression function.
  ///
  /// Function decompresses "raw" data previously compressed into the LZAV stream format.
  ///
  /// Note that while the function performs checks to avoid OOB memory accesses and to verify
  /// decompressed data length, these checks are not a strict guarantee of correct decompression.
  /// In cases when the compressed data is stored without embedded data integrity mechanisms,
  /// a checksum of the original data should be stored and verified after decompression.
  ///
  /// #### Parameters
  ///
  /// - `source`: (Input) Pointer to compressed data, can be 0 if `sourceLength` is 0.
  ///   Address alignment is unimportant.
  /// - `destination`: (Output) Pointer to the destination buffer for decompressed data.
  ///   Address alignment is unimportant. Should be different from `source`.
  /// - `source_length`: Source data length, in bytes.
  /// - `destination_length`: Expected destination data length, in bytes.
  ///   Note: this is not necessarily the size of the destination buffer.
  ///
  /// #### Returns
  ///
  /// The length of decompressed data, in bytes, or a negative value if an error occurred.
  /// Always returns a negative value if the actual decompressed data length differs from `destinationLength`.
  pub fn decompress(
    source: *const c_void,
    source_length: i32,
    destination: *mut c_void,
    destination_length: i32,
  ) -> i32;

  /// LZAV decompression function (partial).
  ///
  /// Function decompresses "raw" data previously compressed into the LZAV stream format,
  /// for partial or recovery decompression. For example, this function can decompress only
  /// the initial segment of a larger data block.
  ///
  /// #### Parameters
  ///
  /// - `source`: (Input) Pointer to compressed data, can be 0 if `sourceLength` is 0.
  ///   Address alignment is unimportant.
  /// - `destination`: (Output) Pointer to the destination buffer for decompressed data.
  ///   Address alignment is unimportant. Should be different from `source`.
  /// - `source_length`: Source data length, in bytes.
  /// - `destination_length`: Destination buffer length, in bytes.
  ///
  /// #### Returns
  ///
  /// The length of decompressed data, in bytes.
  /// Always a non-negative value (error codes are not returned).
  pub fn decompressPartial(
    source: *const c_void,
    source_length: i32,
    destination: *mut c_void,
    destination_length: i32,
  ) -> i32;
}

/// Safe LZAV compression function.
///
/// This function wraps the `compressDefault` method, ensuring safe compression
/// of the input data. It handles edge cases (e.g., empty data or invalid buffer sizes)
/// and returns a compressed `Vec<u8>` if the compression is successful.
///
/// #### Parameters
/// - `input`: The data to be compressed. Should be a non-empty byte slice.
///
/// #### Returns
/// - `Option< Vec<u8> >`: A `Some` containing the compressed data if the compression was
///   successful, or `None` if an error occurred (e.g., empty input, invalid compression size).
pub fn safeCompress(input: &[u8]) -> Option <Vec<u8> >
{
  let inputLen: c_int = input.len() as c_int;
  match inputLen == 0 || input.is_empty()
  { // Handle edge cases, empty or zero-length data
    false => {}
    true => return None
  }

  let bound: c_int = unsafe { compressBound(inputLen) };
  match bound <= 0
  {
    false => {}
    true => return None // If the compression size is smaller or equal to zero
  }

  let mut compressed: Vec<u8> = vec![0u8; bound as usize];

  let compressedLen: c_int = unsafe
    {
      compressDefault(
        input.as_ptr() as *const c_void,
        inputLen,
        compressed.as_mut_ptr() as *mut c_void,
        bound,
      )
    };

  match compressedLen <= 0
  {
    false => {}
    true => return None // If you could not squeeze the data
  }

  compressed.truncate(compressedLen as usize);
  Some(compressed)
}

/// Safe LZAV decompression function.
///
/// This function handles the decompression of LZAV-compressed data, automatically
/// adjusting the buffer size as needed for safe decompression. It processes the data
/// in chunks, ensuring that the decompressed output fits within the allocated buffers.
/// It returns the decompressed data as a `Vec<u8>` if successful, or `None` if an error occurs.
///
/// #### Parameters
/// - `inputData`: The compressed data to be decompressed.
///
/// #### Returns
/// - `Option< Vec<u8> >`: A `Some` containing the decompressed data if the decompression
///   was successful, or `None` if an error occurred (e.g., invalid data or decompression failure).
pub fn safeDecompress(inputData: &[u8]) -> Option< Vec<u8> >
{
  match inputData.is_empty()
  {
    false => {}
    true => return None
  }

  let mut decompressedData: Vec<u8> = Vec::new();
  let mut offsetInput: usize = 0;

  const blockSize: usize = 65536;

  //
  let mut currentChunkSize: usize;
  let mut inputChunk: &[u8];

  let mut bufferSize: usize;

  let mut decompressedLengthUsize: usize;
  let mut decompressedLength: c_int;
  let mut outputBuffer: Vec<u8>;

  while offsetInput < inputData.len()
  {
    currentChunkSize = std::cmp::min(blockSize, inputData.len() -offsetInput);
    inputChunk = &inputData[offsetInput..offsetInput +currentChunkSize];

    // Initial assessment of the size of the output buffer
    bufferSize = currentChunkSize * 10;

    //
    loop
    {
      outputBuffer = vec![0u8; bufferSize];
      decompressedLength = unsafe
        {
          decompressPartial(
            inputChunk.as_ptr() as *const c_void,
            inputChunk.len() as c_int,
            outputBuffer.as_mut_ptr() as *mut c_void,
            outputBuffer.len() as c_int,
          )
        };

      match decompressedLength <= 0
      {
        false => {}
        true => return None
      }
      decompressedLengthUsize = decompressedLength as usize;

      // If decompression has met the current buffer, adjust the length and add the data
      match decompressedLengthUsize < bufferSize
      {
        true =>
          {
            unsafe { outputBuffer.set_len(decompressedLengthUsize); }
            decompressedData.extend_from_slice(&outputBuffer);
            break;
          }
        // If the result filled the buffer, increase the size and try again
        false => bufferSize *= 2
      }
    }
    offsetInput += currentChunkSize;
  }

  Some(decompressedData)
}