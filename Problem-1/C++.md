**C++中存在 unsorted_map 和 map 两种哈希表，导入的方式分别为** `#include <unsorted_map>`和 `#include <map>`。

**unsorted_map 是一个无序关联容器，它使用哈希函数来存储和检索键值对，这意味着它存储的键值对没有特定的顺序。平均情况下，插入和查找的时间复杂度为 O(1)，不支持范围查询。**

**map 是一个有序关联容器，它根据键的比较函数对键进行排序，由于有序性质，map 支持范围查询，插入和查找操作的平均时间复杂度为 O(log n)。这是因为 map 是基于红黑树实现的。**

###### 声明一个 map 有以下几种方式

```
#include <map>

std::map<int, std::string> myMap;
```

**这将创建一个空的** `std::map`对象。

```
#include <map>

std::map<int, std::string> myMap = {{1, "One"}, {2, "Two"}, {3, "Three"}};
```

**使用初始化列表可以在创建** `std::map`的时候插入一些键值对。

```
#include <map>

std::map<int, std::string> originalMap({{1, "One"}, {2, "Two"}, {3, "Three"}});
std::map<int, std::string> myMap(originalMap.begin(), originalMap.end());
```

**在 C++11 中引入的，使用花括号进行范围构造，随后使用迭代器复制另一个** `std::map`的内容。

###### 增删改查四种操作

**将新的键值对插入到对象中**

```
std::map<int, std::string> myMap;

myMap.insert(std::make_pair(1, "One"));
myMap[2] = "Two";

// 使用emplace进行插入，效率更高
myMap.emplace(3, "Three");
```

**从原有对象中删除键值对**

```
myMap.erase(2);

// 通过myMap.begin()可以访问到map的第一个元素
myMap.erase(myMap.begin());
```

**修改对象中的元素，所用到的操作和插入很像**

```
myMap[1] = "Uno";

myMap.insert(std::make_pair(2, "Deux"));
```

**查找给定的值**

```
std::map<int, std::string>::iterator it = myMap.find(1);

hashMap.count(currentNum) > 0
```

**使用 find 函数查找时，如果键不存在，会返回** `end()`迭代器，因此需要检查是否等于 `end()`来确定是否找到了键。

**除此之外，还可以使用** `count()`函数，查看指定的键个数是否大于 0，若是，即为存在。在 `std::map`中，键是唯一的，每个键只能对应一个值。
