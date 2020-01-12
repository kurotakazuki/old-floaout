# Floaout

Floaout enables immersive sound.

### Floaout (.oao)
| Name              | Field             | `Type` (Length)    | Contents                          |
| ----------------- | ----------------- | ------------------ | --------------------------------- |
| Floaout           | floaout           | `String` (3)       | “oao” means Floaout               |
| Version           | version           | `u8` (1)           | Version of Floaout (0~255)        |
| Length            | length            | `u8` (1)           | Length of Bubble field (2^n)      |
| Width             | width             | `u8` (1)           | Width of Bubble field (2^n)       |
| Height            | height            | `u8` (1)           | Height of Bubble field (2^n)      |
| Bubbles           | bubbles           | `u16` (2)          | Number of Bubbles (0~65535)       |
| Blocks            | blocks            | `u64` (8)          | Number of Block                   |
| Sampling Rate     | sampling_rate	    | `u32` (4)          | Sampling Rate                     |
| Bits Per Sample   | bits_per_sample   | `u16` (2)          | Bits Per Sample                   |
| Title Size        | title_size        | `u8` (1)           | Title Size                        |
| Title             | title             | `String`           | Title (UTF-8)                     |
| Artist Size       | artist_size       | `u8` (1)           | Artist Size                       |
| Artist            | artist            | `String`           | Artist (UTF-8)                    |
| Lyrics Size       | lyrics_size       | `u16` (2)          | Lyrics Size                       |
| Lyrics            | lyrics            | `String`           | Lyrics (UTF-8)                    |
| CRC-32C           | crc_32_c          | `u32` (4)          | Everything until Begin            |
| Image Size        | image_size        | `u32` (4)          | Image Size                        |
| Image Code        | image_code        | `u32` (4)          | Image Code                        |
| Image             | image             |                    | Image                             |
| CRC-32C           | crc_32_c          | `u32` (4)          | Everything until previous CRC     |
#### Each Bubble
| Name              | Field             | `Type` (Length)    | Contents                          |
| ----------------- | ----------------- | ------------------ | --------------------------------- |
| Name Size         | name_size         | `u8` (1)           | Name Size                         |
| Name              | name              | `String`           | Name of bubble (UTF-8)            |
| Red               | red               | `u8` (1)           | Red                               |
| Green             | green             | `u8` (1)           | Green                             |
| Blue              | blue              | `u8` (1)           | Blue                              |
| CRC-32C           | crc_32_c          | `u32` (4)          | Everything until Name Size        |
#### Each Block
| Name              | Field             | `Type` (Length)    | Contents                          |
| ----------------- | ----------------- | ------------------ | --------------------------------- |
| Wave Data 1       | (wave_data_1)     | (Wave Data 1)      |                                   |
| Form 1            | (form_1)          | (Bubble Field*u8)  | Data of Bubble Field              |
| …                 | …                 | …                  | …                                 |
| Wave Data i       | (wave_data_i)     | (Wave Data i)      |                                   |
| Form i            | (form_i)          | (Bubble Field*u8)  | Data of Bubble Field              |
| CRC-32C           | crc_32_c          | `u32` (4)          | Everything until previous CRC     |

i = Number of Bubbles

## Soap

Soap makes extensions of Bubbles(.bub).

### Bubble (.bub)
| Name             | Field            | `Type` (Length)      | Contents                          |
| -----------------| ---------------- | -------------------- | --------------------------------- |
| Bubble           | bubble           | `String` (3)         | “bub” means Bubble               |
| Version          | version          | `u8` (1)             | Version of Bubble (0~255)         |
| Length           | length           | `u8` (1)             | Length of Bubble field (2^n)      |
| Width            | width            | `u8` (1)             | Width of Bubble field (2^n)       |
| Height           | height           | `u8` (1)             | Height of Bubble field (2^n)      |
| Red              | red              | `u8` (1)             | Red                               |
| Green            | green            | `u8` (1)             | Green                             |
| Blue             | blue             | `u8` (1)             | Blue                              |
| Blocks           | blocks           | `u64` (8)            | Number of Block                   |
| Sampling Rate    | sampling_rate    | `u32` (4)            | Sampling Rate                     |
| Bits Per Sample  | bits_per_sample  | `u16` (2)            | Bits Per Sample                   |
| Name Size        | name_size        | `u8` (1)             | Name Size                         |
| Name             | name             | `String`             | Name of bubble (UTF-8)            |
| Overall          | overall          | `Vec<Vec<Vec<u8>>>`  | Overall of Bubble Field           |
| Wave Data n      | (wave_data_n)    | (Wave Data n)        |                                   |
| Form n           | (form_n)         | (Bubble Field*u8)    | Data of Bubble Field              |
| …                | …                | …                    | …                                 |

n = Number of Block

## Blower

Blower makes extensions of Floaout(.oao) and uses extensions of Blower(.blow).

### Blower (.blow)
| Name              | Field             | `Type` (Length)    | Contents                          |
| ----------------- | ----------------- | ------------------ | --------------------------------- |
| Blower            | blower            | `String` (4)       | “blow” means Blower               |
| Version           | version           | `u8` (1)           | Version of Blower (0~255)         |
| Length            | length            | `u8` (1)           | Length of Bubble field (2^n)      |
| Width             | width             | `u8` (1)           | Width of Bubble field (2^n)       |
| Height            | height            | `u8` (1)           | Height of Bubble field (2^n)      |
| Bubbles           | bubbles           | `u16` (2)          | Number of Bubbles (0~65535)       |
| Blocks            | blocks            | `u64` (8)          | Number of Block                   |
| Sampling Rate     | sampling_rate	    | `u32` (4)          | Sampling Rate                     |
| Bits Per Sample   | bits_per_sample   | `u16` (2)          | Bits Per Sample                   |
| Title Size        | title_size        | `u8` (1)           | Title Size                        |
| Title             | title             | `String`           | Title (UTF-8)                     |
| Artist Size       | artist_size       | `u8` (1)           | Artist Size                       |
| Artist            | artist            | `String`           | Artist (UTF-8)                    |
| Bubbles Name Size | bubbles_name_size | `Vec<u8>`          | Bubbles Name Size                 |
| Bubbles Name      | bubbles_name      | `Vec<String>`      | Bubbles Name                      |
| Bubbles Time Size | bubbles_time_size | `Vec<u64>`         | Bubbles Time Size                 |
| Bubbles Time      | bubbles_time      | `Vec<(u64, u64)>`  | (start, end)                      |


### BubblesInBlower
| Name              | Field             | `Type` (Length)    | Contents                          |
| ----------------- | ----------------- | ------------------ | --------------------------------- |
| Name              | name              | `String`           | Bubble Name                       |
| Time              | time              | `Vec<(u64, u64)>`  | (start, end)                      |


## Pop

Pop is player of Floaout(.oao).


## Developer

* **Kazuki Kurota** - [kurotakazuki](https://github.com/kurotakazuki)


## License

- Except modify or derive from these formats specification, anyone can use or create these formats the way each wants.
- these formats = { Blower, Bubble, Floaout }