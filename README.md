# Floaout
A Rust library for Floaout.

Floaout is the forefront audio format that enables immersive sound which takes advantage of both channel-based and object-based system.

Note: Floaout can build only `nightly` at this moment.

## Floaout (.oao)
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Floaout           | `str` (3)          | “oao” means Floaout               |
| Version           | `u8` (1)           | Version of Floaout (0~255)        |
| Song ID           | `u64` (8)          | Song ID of the file               |
| Length            | `u8` (1)           | Length of Bubble field (2^n)      |
| Width             | `u8` (1)           | Width of Bubble field (2^n)       |
| Height            | `u8` (1)           | Height of Bubble field (2^n)      |
| Bubbles           | `u16` (2)          | Number of Bubbles (0~65535)       |
| Blocks            | `u64` (8)          | Number of Block                   |
| Sampling Rate     | `u32` (4)          | Sampling Rate                     |
| Bits Per Sample   | `u16` (2)          | Bits Per Sample                   |
| CRC-32C           | `u32` (4)          | Everything until Begin            |
#### Each Bubble
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Name Size         | `u8` (1)           | Name Size                         |
| Name              | `String`           | Name of bubble (UTF-8)            |
| Red               | `u8` (1)           | Red                               |
| Green             | `u8` (1)           | Green                             |
| Blue              | `u8` (1)           | Blue                              |
| CRC-32C           | `u32` (4)          | Everything until Name Size        |
#### Each Block
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Wave Data 1       | (Wave Data 1)      |                                   |
| Form 1            | (Bubble Field*u8)  | Data of Bubble Field              |
| …                 | …                  | …                                 |
| Wave Data i       | (Wave Data i)      |                                   |
| Form i            | (Bubble Field*u8)  | Data of Bubble Field              |
| CRC-32C           | `u32` (4)          | Everything until previous CRC     |

i = Number of Bubbles

### What's Song ID?

Song ID is 8 bytes data. It links to title, artist, lyrics, image, etc.
This ID will save the file size and enhance versatility.
Developers also have the merit that they don't have to process title string and so on in the file and allow concentrate on audio processing.
If Song ID is "0x0000000000000000", this means no link.

#### BubbleInFloaout
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Name Size         | `u8` (1)           | Name Size                         |
| Name              | `String`           | Bubble Name                       |
| Red               | `u8` (1)           | Red                               |
| Green             | `u8` (1)           | Green                             |
| Blue              | `u8` (1)           | Blue                              |


### Bubble (.bub)
| Name             | `Type` (Bytes)       | Contents                          |
| -----------------| -------------------- | --------------------------------- |
| Bubble           | `str` (3)            | “bub” means Bubble                |
| Version          | `u8` (1)             | Version of Bubble (0~255)         |
| Length           | `u8` (1)             | Length of Bubble field (2^n)      |
| Width            | `u8` (1)             | Width of Bubble field (2^n)       |
| Height           | `u8` (1)             | Height of Bubble field (2^n)      |
| Red              | `u8` (1)             | Red                               |
| Green            | `u8` (1)             | Green                             |
| Blue             | `u8` (1)             | Blue                              |
| Blocks           | `u64` (8)            | Number of Block                   |
| Sampling Rate    | `u32` (4)            | Sampling Rate                     |
| Bits Per Sample  | `u16` (2)            | Bits Per Sample                   |
| Name Size        | `u8` (1)             | Name Size                         |
| Name             | `String`             | Name of bubble (UTF-8)            |
| Overall          | `Vec<Vec<Vec<u8>>>`  | Overall of Bubble Field           |
| Wave Data n      | (Wave Data n)        |                                   |
| Form n           | (Bubble Field*u8)    | Data of Bubble Field              |
| …                | …                    | …                                 |

n = Number of Block


## Developer

* **Kazuki Kurota** - [kurotakazuki](https://github.com/kurotakazuki)

## Library License

- [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)

## Format License

- Except modify or derive from these formats specification, anyone can use or create these formats the way each wants.
- these formats = { Bubble, Floaout }