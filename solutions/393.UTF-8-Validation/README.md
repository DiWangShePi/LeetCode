# 393. UTF-8 Validation

### Description

Given an integer array data representing the data, return whether it is a valid UTF-8 encoding (i.e. it translates to a sequence of valid UTF-8 encoded characters).

A character in UTF8 can be from 1 to 4 bytes long, subjected to the following rules:

For a 1-byte character, the first bit is a 0, followed by its Unicode code.
For an n-bytes character, the first n bits are all one's, the n + 1 bit is 0, followed by n - 1 bytes with the most significant 2 bits being 10.
This is how the UTF-8 encoding would work:

```
Number of Bytes   |        UTF-8 Octet Sequence
                       |              (binary)
   --------------------+-----------------------------------------
            1          |   0xxxxxxx
            2          |   110xxxxx 10xxxxxx
            3          |   1110xxxx 10xxxxxx 10xxxxxx
            4          |   11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
```

x denotes a bit in the binary form of a byte that may be either 0 or 1.

Note: The input is an array of integers. Only the least significant 8 bits of each integer is used to store the data. This means each integer represents only 1 byte of data.

### Example

###### Example I

> Input: data = [197,130,1]
> Output: true
> Explanation: data represents the octet sequence: 11000101 10000010 00000001.
> It is a valid utf-8 encoding for a 2-bytes character followed by a 1-byte character.

###### Example II

> Input: data = [235,140,4]
> Output: false
> Explanation: data represented the octet sequence: 11101011 10001100 00000100.
> The first 3 bits are all one's and the 4th bit is 0 means it is a 3-bytes character.
> The next byte is a continuation byte which starts with 10 and that's correct.
> But the second continuation byte does not start with 10, so it is invalid.

### Solution

遍历，按照题目要求检查

```c++
class Solution {
public:
    bool validUtf8(vector<int>& data) {
        int p = 0;
        while (p < data.size()) {
            uint8_t current = data[p];

            int num_of_bytes = 0;
            while (num_of_bytes < 8 && (current & (1 << (7 - num_of_bytes))) != 0) num_of_bytes++;

            if (p + num_of_bytes > data.size() || num_of_bytes == 1 || num_of_bytes > 4) return false;
            if (num_of_bytes == 0) num_of_bytes = 1;

            for (int i = 1; i < num_of_bytes; i++) {
                uint8_t seq = data[p + i];  

                if ((seq & (1 << 7)) && !(seq & (1 << 6))) {
                    // satisfy the 10xxxxxx pattern
                } else return false;
            }
            if (num_of_bytes == 0) p++;
            else p += num_of_bytes;
        }
        return true;
    }
};
```

> 一个更简洁优雅的版本

```c++
class Solution {
public:
    bool validUtf8(vector<int>& data) {
        int i = 0;
        while (i < data.size()) {
            int count = count_leading_ones(data[i]);
            if (count == 0) {
                i++;
                continue;
            }
            if (count == 1 || count > 4 || i + count > data.size()) {
                return false;
            }
            for (int j = 1; j < count; j++) {
                if ((data[i + j] & 0b11000000) != 0b10000000) {
                    return false;
                }
            }
            i += count;
        }
        return true;
    }

private:
    int count_leading_ones(uint8_t byte) {
        int count = 0;
        while (count < 8 && (byte & (1 << (7 - count)))) {
            count++;
        }
        return count;
    }
};
```
