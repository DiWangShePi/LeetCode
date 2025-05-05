# 315. Count of Smaller Numbers After Self

### Description

Given an integer array `nums`, return an integer array counts where `counts[i]` is the number of smaller elements to the right of `nums[i]`.

### Example 

###### Example I

```
Input: nums = [5,2,6,1]
Output: [2,1,1,0]
Explanation:
To the right of 5 there are 2 smaller elements (2 and 1).
To the right of 2 there is only 1 smaller element (1).
To the right of 6 there is 1 smaller element (1).
To the right of 1 there is 0 smaller element.
```

###### Example II

```
Input: nums = [-1]
Output: [0]
```

###### Example III

```
Input: nums = [-1,-1]
Output: [0,0]
```

### Solution

当然，我们总是可以尝试用暴力的方式解决问题

> 超时归超时，你就说对不对吧

```c++
class Solution {
public:
    vector<int> countSmaller(vector<int>& nums) {
        vector<int> an;
        int n = nums.size();
        for (int i = 0; i < n; i++) {
            int count  = 0;
            for (int j = i + 1; j < n; j++) {
                if (nums[j] < nums[i]) count++;
            }
            an.push_back(count);
        }
        return an;
    }
};
```

我们尝试考虑动态规划的算法，从右到左的考虑这个问题。
定义`dp[i]`代表位置i右边比`nums[i]`小的数字个数，显然，当`nums[i]`大于`nums[j]`时，有`dp[i]`等于`dp[j] + 1`。
但当数列是递增的时候，这一算法会退化到暴力的情况。

我们需要一种优雅的找到右边第一个比`nums[i]`小的`nums[j]`的方式。我思考过如何快速的获取到一个指定数列中，比目标值小的第一个数，这和二分查找十分相似。但用vector每次构造新的适用于二分查找的数列似乎不太优雅。可能我确实需要一个新的数据结构。平衡树听起来不错，我可以将数值作为节点的键值，再存储一个count代表比这个值小的数字的个数。

> 但他还是超时了

```c++
class BST {
public:
    int val;
    int left_count; 
    int count;       
    BST* left;
    BST* right;

    BST(int v) : val(v), left_count(0), count(1), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    vector<int> countSmaller(vector<int>& nums) {
        int n = nums.size();
        vector<int> res(n);
        BST* root = nullptr;

        for (int i = n - 1; i >= 0; --i) {
            res[i] = insert(root, nums[i]);
        }

        return res;
    }

    int insert(BST*& node, int val) {
        if (!node) {
            node = new BST(val);
            return 0;
        }

        if (val == node->val) {
            node->count++;
            return node->left_count;
        } else if (val < node->val) {
            node->left_count++;
            return insert(node->left, val);
        } else {
            int smaller = node->left_count + node->count;
            return smaller + insert(node->right, val);
        }
    }
};
```

于是我们转而采用归并排序。对于排好序的`left`和`right`两个子数组，如果`left[i]`大于`right[j]`，就代表在`right`数组中有`j`个值比此时的`left[i]`小。
由此自底向上，即可逐步计算出目标值。

```c++
class Solution {
public:
    vector<int> countSmaller(vector<int>& nums) {
        int n = nums.size();
        vector<int> indices(n), result(n, 0), temp_indices(n);
        for (int i = 0; i < n; ++i) indices[i] = i;

        mergeSort(nums, indices, result, 0, n - 1, temp_indices);
        return result;
    }

private:
    void mergeSort(const vector<int>& nums, vector<int>& indices, vector<int>& result,
                   int left, int right, vector<int>& temp) {
        if (left >= right) return;
        int mid = (left + right) / 2;
        mergeSort(nums, indices, result, left, mid, temp);
        mergeSort(nums, indices, result, mid + 1, right, temp);
        merge(nums, indices, result, left, mid, right, temp);
    }

    void merge(const vector<int>& nums, vector<int>& indices, vector<int>& result,
               int left, int mid, int right, vector<int>& temp) {
        int i = left;     
        int j = mid + 1;  
        int k = left;     
        int rightCount = 0;

        while (i <= mid && j <= right) {
            if (nums[indices[j]] < nums[indices[i]]) {
                temp[k++] = indices[j++];
                rightCount++;
            } else {
                result[indices[i]] += rightCount;
                temp[k++] = indices[i++];
            }
        }

        while (i <= mid) {
            result[indices[i]] += rightCount;
            temp[k++] = indices[i++];
        }

        while (j <= right) {
            temp[k++] = indices[j++];
        }

        for (int p = left; p <= right; ++p) {
            indices[p] = temp[p];
        }
    }
};

```


> 一个有趣的线段树实现

```c++
#pragma GCC optimize("Ofast")
#pragma GCC target("sse,sse2,sse3,ssse3,sse4,popcnt,abm,mmx,avx,avx2,fma")
#pragma GCC optimize("unroll-loops")
static const int _ = []() { std::ios::sync_with_stdio(false); std::cin.tie(nullptr); std::cout.tie(nullptr); return 0; }();
const auto __ = []() { struct ___ { static void _() { std::ofstream("display_runtime.txt") << 0 << '\n'; } }; std::atexit(&___::_); return 0; }();
class Solution {
public:
vector<int> st[400025],ans;
int a[100005];
void build(int i,int l,int r,const int a[]){
  if (l==r){
    st[i].push_back(a[l]);
    return;
  }
  int m=(l+r)>>1;
  build(i<<1,l,m,a);
  build(i<<1|1,m+1,r,a);
  for (int x :st[i<<1]) st[i].push_back(x);
  for (int x:st[i<<1|1]) st[i].push_back(x);
  sort(st[i].begin(),st[i].end());
}
int get(int i,int l,int r,int u,int v,int val){
    if (l>v||r<u||l>r||u>v) return 0;
    if (l>=u&&r<=v){
        int pos=lower_bound(st[i].begin(),st[i].end(),val)-st[i].begin();
        return pos;
    }
    int m=(l+r)>>1;
    return (get(i<<1,l,m,u,v,val)+get(i<<1|1,m+1,r,u,v,val));
}
    vector<int> countSmaller(vector<int>& nums) {
        __;
        _;
        int d=1;
        for (int i=0;i<nums.size();i++) {a[d]=nums[i]; d++;}
        int n=d-1; 
        build(1,1,n,a);
        for (int i=1;i<=n;i++)
        ans.push_back(get(1,1,n,i+1,n,a[i]));
        return ans;
    }
};
```

> 一个PBDS实现

> chatgpt-o4
> 这是 C++ 中一个非常强大的非标准扩展，属于 GNU 的扩展 STL。你可以使用一种叫做 ordered_set 的结构，它支持两个关键操作：
> - order_of_key(x)：返回集合中严格小于 x 的元素个数（正是你需要的！）
> - find_by_order(k)：返回第 k 小的元素

```c++
#include <vector>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>

using namespace std;
using namespace __gnu_pbds;

// 定义 PBDS 的 Ordered Set 类型（支持重复元素需要用 pair）
typedef tree<
    pair<int, int>,           // 存储 pair<数值, 插入序号>，用于处理重复值
    null_type,
    less<pair<int, int>>,     // 使用 less 保证有序性
    rb_tree_tag,
    tree_order_statistics_node_update>
    ordered_set;

class Solution {
public:
    vector<int> countSmaller(vector<int>& nums) {
        int n = nums.size();
        ordered_set os;
        vector<int> res(n);

        for (int i = n - 1; i >= 0; --i) {
            // 查询当前有多少个数 < nums[i]
            res[i] = os.order_of_key({nums[i], 0});
            // 插入当前值，i 用来区分重复元素
            os.insert({nums[i], i});
        }

        return res;
    }
};
```