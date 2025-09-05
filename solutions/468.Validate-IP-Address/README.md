# 468. Validate IP Address

### Description

Given a string queryIP, return "IPv4" if IP is a valid IPv4 address, "IPv6" if IP is a valid IPv6 address or "Neither" if IP is not a correct IP of any type.

A valid IPv4 address is an IP in the form "x1.x2.x3.x4" where 0 <= xi <= 255 and xi cannot contain leading zeros. For example, "192.168.1.1" and "192.168.1.0" are valid IPv4 addresses while "192.168.01.1", "192.168.1.00", and "192.168@1.1" are invalid IPv4 addresses.

A valid IPv6 address is an IP in the form "x1:x2:x3:x4:x5:x6:x7:x8" where:

- 1 <= xi.length <= 4
- xi is a hexadecimal string which may contain digits, lowercase English letter ('a' to 'f') and upper-case English letters ('A' to 'F').
- Leading zeros are allowed in xi.
For example, "2001:0db8:85a3:0000:0000:8a2e:0370:7334" and "2001:db8:85a3:0:0:8A2E:0370:7334" are valid IPv6 addresses, while "2001:0db8:85a3::8A2E:037j:7334" and "02001:0db8:85a3:0000:0000:8a2e:0370:7334" are invalid IPv6 addresses.

### Example 

###### Example I

> Input: queryIP = "172.16.254.1"
> Output: "IPv4"
> Explanation: This is a valid IPv4 address, return "IPv4".

###### Example II

> Input: queryIP = "2001:0db8:85a3:0:0:8A2E:0370:7334"
> Output: "IPv6"
> Explanation: This is a valid IPv6 address, return "IPv6".

###### Example III

> Input: queryIP = "256.256.256.256"
> Output: "Neither"
> Explanation: This is neither a IPv4 address nor a IPv6 address.

### Solution

按要求实现即可

```c++
class Solution {
public:
    string validIPAddress(string queryIP) {
        vector<string> tryIPV4 = splitStringAdvanced(queryIP, '.');
        if (checkIPV4(tryIPV4)) return "IPv4";

        vector<string> tryIPV6 = splitStringAdvanced(queryIP, ':');
        if (checkIPV6(tryIPV6)) return "IPv6";

        return "Neither";
    }

private:
    vector<string> splitStringAdvanced(const string& input, char delimiter) {
        vector<string> tokens;
        stringstream ss(input);
        string token;
        
        while (getline(ss, token, delimiter)) {
            tokens.push_back(token);
        }
        if (!input.empty() && input.back() == delimiter) {
            tokens.push_back(""); 
        }
        return tokens;
    }

    bool checkIPV4(vector<string>& ips) {
        if (ips.size() != 4) return false;

        for (const string& s : ips) {
            if (s.size() != 1 && s[0] == '0') return false;
            if (s.size() >= 4 || s.size() < 1) return false;

            int num = 0;
            for (char c : s) {
                if (c < '0' || c > '9') return false;

                num += c - '0';
                num *= 10;
            }
            num /= 10;
            if (num >= 0 && num <= 255) continue;
            else return false;
        }
        return true;
    }

    bool checkIPV6(vector<string>& ips) {
        if (ips.size() != 8) return false;

        for (const string& s : ips) {
            if (s.size() > 4 || s.size() < 1) return false;

            for (char c : s) {
                if ((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')) continue;
                else return false;
            }
        }
        return true;
    }
};
```
