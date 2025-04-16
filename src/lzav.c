#include "lzav.h"

int compressBound(const int uncompressedLength)
{
  return lzav_compress_bound(uncompressedLength);
}

int compressHighBound(const int uncompressedLength)
{
  return lzav_compress_bound_hi(uncompressedLength);
}

int compress(
  const void* const source, const int sourceLength,
  void* const destination, const int destinationLength,
  void* const externalBuffer, const int externalBufferLength
)
{
  return lzav_compress(
    source,
    destination,
    sourceLength,
    destinationLength,
    externalBuffer,
    externalBufferLength
  );
}

int compressDefault(
  const void* const source,
  const int sourceLength,
  void* const destination,
  const int destinationLength
)
{
  return lzav_compress_default(
    source,
    destination,
    sourceLength,
    destinationLength
   );
}

int compressHigh(
  const void* const source,
  const int sourceLength,
  void* const destination,
  const int destinationLength
)
{
  return lzav_compress_hi(
    source,
    destination,
    sourceLength,
    destinationLength
  );
}

int decompress(
  const void* const source,
  const int sourceLength,
  void* const destination,
  const int destinationLength
)
{
  return lzav_decompress(
    source,
    destination,
    sourceLength,
    destinationLength
  );
}

int decompressPartial(
  const void* const source,
  const int sourceLength,
  void* const destination,
  const int destinationLength
)
{
  return lzav_decompress_partial(
    source,
    destination,
    sourceLength,
    destinationLength
  );
}