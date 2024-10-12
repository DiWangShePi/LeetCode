# 71. Simplify Path

### 题目描述

您将获得一个 Unix 风格文件系统的绝对路径，该路径始终以斜杠“/”开头。您的任务是将此绝对路径转换为其简化的规范路径。

Unix 风格文件系统的规则如下：

- 单个句点“.”表示当前目录。
- 双句点“..”表示上一个/父目录。
- 多个连续的斜杠（例如“//”和“///”）被视为单个斜杠“/”。

任何不符合上述规则的句点序列都应被视为有效的目录或文件名。例如，“...”和“....”是有效的目录或文件名。
简化的规范路径应遵循以下规则：

- 路径必须以单个斜杠“/”开头。
- 路径中的目录必须由一个斜杠“/”分隔。
- 路径不得以斜杠“/”结尾，除非它是根目录。
- 路径不得包含任何用于表示当前目录或父目录的单句点或双句点（“。”和“..”）。
- 返回简化的规范路径。

### 题目解析

维护当前经历过的字符，添加到一个字符串中。当遇到一个"/"时，处理当前记录下来的字符串：
- 若为完整的字符串，则将该字符串加入到结果列表中，指向结果列表的指针向后移动一格。
- 若为"."，则指向结果列表的指针保持不变。
- 若为".."，则指向结果列表的指针向前移动一格。
遍历结束后，将结果列表中的字符串依次取出，得到最终结果

### 代码实现

###### c++

```c++
class Solution {
public:
    string simplifyPath(string path) {
        vector<string> result;
        string current;
        for (char ch : path) {
            if (ch == '/') {
                if (current.empty()) continue;
                if (current == ".") {
                    current.clear();
                } else if (current == "..") {
                    if (!result.empty()) result.pop_back();
                    current.clear();
                } else {
                    result.push_back(current);
                    current.clear();
                }
            } else {
                current += ch;
            }
        }

        // 处理最后一段路径
        if (!current.empty()) {
            if (current == "..") {
                if (!result.empty()) result.pop_back();
            } else if (current != ".") {
                result.push_back(current);
            }
        }

        // 构造最终路径
        string simplifiedPath;
        for (const string& dir : result) {
            simplifiedPath += "/" + dir;
        }

        return simplifiedPath.empty() ? "/" : simplifiedPath;
    }
};
```
