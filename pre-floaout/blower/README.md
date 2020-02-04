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


### BubbleInBlower
| Name              | `Type` (Bytes)     | Contents                          |
| ----------------- | ------------------ | --------------------------------- |
| Name Size         | `u8` (1)           | Name Size                         |
| Name              | `String`           | Bubble Name                       |
| Times             | `u32`              | Number of ranges in Bubble        |
| Ranges            | `Vec<(u64, u64)>`  | (start, end)                      |

