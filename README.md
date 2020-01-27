# Floaout

Floaout enables immersive sound.

### Floaout (.oao)
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Floaout           | `str` (3)          | “oao” means Floaout               |
| Version           | `u8` (1)           | Version of Floaout (0~255)        |
| Length            | `u8` (1)           | Length of Bubble field (2^n)      |
| Width             | `u8` (1)           | Width of Bubble field (2^n)       |
| Height            | `u8` (1)           | Height of Bubble field (2^n)      |
| Bubbles           | `u16` (2)          | Number of Bubbles (0~65535)       |
| Blocks            | `u64` (8)          | Number of Block                   |
| Sampling Rate     | `u32` (4)          | Sampling Rate                     |
| Bits Per Sample   | `u16` (2)          | Bits Per Sample                   |
| Title Size        | `u8` (1)           | Title Size                        |
| Title             | `String`           | Title (UTF-8)                     |
| Artist Size       | `u8` (1)           | Artist Size                       |
| Artist            | `String`           | Artist (UTF-8)                    |
| Lyrics Size       | `u16` (2)          | Lyrics Size                       |
| Lyrics            | `String`           | Lyrics (UTF-8)                    |
| CRC-32C           | `u32` (4)          | Everything until Begin            |
| Image Code        | `u8` (1)           | Image Code                        |
| Image Size        | `u32` (4)          | Image Size                        |
| Image             |                    | Image                             |
| CRC-32C           | `u32` (4)          | Everything until previous CRC     |
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

### BubblesInFloaout
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Name              | `String`           | Bubble Name                       |
| Red               | `u8` (1)           | Red                               |
| Green             | `u8` (1)           | Green                             |
| Blue              | `u8` (1)           | Blue                              |

## Soap

Soap makes extensions of Bubbles(.bub).

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

## Blower

Blower makes extensions of Floaout(.oao) and uses extensions of Blower(.blow).

### Blower (.blow)
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Blower            | `str` (4)          | “blow” means Blower               |
| Version           | `u8` (1)           | Version of Blower (0~255)         |
| Length            | `u8` (1)           | Length of Bubble field (2^n)      |
| Width             | `u8` (1)           | Width of Bubble field (2^n)       |
| Height            | `u8` (1)           | Height of Bubble field (2^n)      |
| Bubbles           | `u16` (2)          | Number of Bubbles (0~65535)       |
| Blocks            | `u64` (8)          | Number of Block                   |
| Sampling Rate     | `u32` (4)          | Sampling Rate                     |
| Bits Per Sample   | `u16` (2)          | Bits Per Sample                   |
| Title Size        | `u8` (1)           | Title Size                        |
| Title             | `String`           | Title (UTF-8)                     |
| Artist Size       | `u8` (1)           | Artist Size                       |
| Artist            | `String`           | Artist (UTF-8)                    |
| Lyrics Size       | `u16` (2)          | Lyrics Size                       |
| Lyrics            | `String`           | Lyrics (UTF-8)                    |
| Image Code        | `u8` (1)           | Image Code                        |
| Image Size        | `u32` (4)          | Image Size                        |
| Image             |                    | Image                             |


### BubblesInBlower
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Name Size         | `u8`               | Bubble Name Size                  |
| Name              | `String`           | Bubble Name                       |
| Times             | `u32`              | Bubble Time                       |
| Range             | `Vec<(u64, u64)>`  | (start, end)                      |


## Pop

Pop is player of Floaout(.oao).


## Developer

* **Kazuki Kurota** - [kurotakazuki](https://github.com/kurotakazuki)


## License

- Except modify or derive from these formats specification, anyone can use or create these formats the way each wants.
- these formats = { Blower, Bubble, Floaout }