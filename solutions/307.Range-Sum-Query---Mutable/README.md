# 307. Range Sum Query - Mutable

### Description

Given an integer array nums, handle multiple queries of the following types:

1. Update the value of an element in nums.
2. Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.
Implement the NumArray class:

- NumArray(int[] nums) Initializes the object with the integer array nums.
- void update(int index, int val) Updates the value of nums[index] to be val.
- int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).

### Example 

###### Example I

```
Input
["NumArray", "sumRange", "update", "sumRange"]
[[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
Output
[null, 9, null, 8]

Explanation
NumArray numArray = new NumArray([1, 3, 5]);
numArray.sumRange(0, 2); // return 1 + 3 + 5 = 9
numArray.update(1, 2);   // nums = [1, 2, 5]
numArray.sumRange(0, 2); // return 1 + 2 + 5 = 8
```

### Solution

采用和前面类似的方式，加一个O(n)的更新

```c++
class NumArray {
    vector<int> sums;
    vector<int> copy;
public:
    NumArray(vector<int>& nums) {
        int result = nums[0];
        sums.push_back(result);
        for (int i = 1; i < nums.size(); i++) {
            result += nums[i];
            sums.push_back(result);
        }
        copy = nums;
    }
    
    void update(int index, int val) {
        int diff = copy[index] - val;
        copy[index] = val;

        for (int i = index; i < sums.size(); i++) sums[i] -= diff;
    }
    
    int sumRange(int left, int right) {
        if (left == 0) return sums[right];
        else return sums[right] - sums[left - 1];
    }
};

/**
 * Your NumArray object will be instantiated and called as such:
 * NumArray* obj = new NumArray(nums);
 * obj->update(index,val);
 * int param_2 = obj->sumRange(left,right);
 */
```

用线段树会更快一些

```c++
class NumArray {
private:
    vector<int> tree; 
    int n;             

    void build(vector<int>& nums, int node, int l, int r) {
        if (l == r) {
            tree[node] = nums[l];
            return;
        }
        int mid = (l + r) / 2;
        build(nums, 2 * node, l, mid);
        build(nums, 2 * node + 1, mid + 1, r);
        tree[node] = tree[2 * node] + tree[2 * node + 1];
    }

    void update(int node, int l, int r, int index, int val) {
        if (l == r) {
            tree[node] = val;
            return;
        }
        int mid = (l + r) / 2;
        if (index <= mid) update(2 * node, l, mid, index, val);
        else update(2 * node + 1, mid + 1, r, index, val);
        tree[node] = tree[2 * node] + tree[2 * node + 1];
    }

    int query(int node, int l, int r, int ql, int qr) {
        if (ql <= l && r <= qr) return tree[node];
        if (qr < l || r < ql) return 0;
        int mid = (l + r) / 2;
        return query(2 * node, l, mid, ql, qr) + query(2 * node + 1, mid + 1, r, ql, qr);
    }

public:
    NumArray(vector<int>& nums) {
        n = nums.size();
        tree.resize(4 * n); 
        build(nums, 1, 0, n - 1);
    }
    
    void update(int index, int val) {
        update(1, 0, n - 1, index, val);
    }
    
    int sumRange(int left, int right) {
        return query(1, 0, n - 1, left, right);
    }
};

```

一个树状数组的实现：

```
class BIT{
    public:
    vector<int>vec;
    BIT()
    {

    }
    void update(int idx, int val)
    {
        while(idx < vec.size())
        {
            vec[idx]+=val;
            idx += (idx & -idx);
        }
    }
    int sum(int idx)
    {
        int ans=0;
        while(idx > 0)
        {
            ans += vec[idx];
            idx -= (idx & -idx);
        }
        return ans;
    }
    int read_single(int idx)
    {
        int sum = vec[idx];
        
        if(idx > 0) //idx should never be zero
        {
            int par = idx - (idx & -idx);
            idx--;
            while(idx != par)
            {
                sum -= vec[idx];
                idx -= (idx & -idx);
            }
        }
        return sum;
    }
};
class NumArray {
    BIT bit_tree;
public:
    NumArray(vector<int>& nums) {
        bit_tree.vec.resize(nums.size()+1,0);
        for(int i=0;i<nums.size();i++)
        bit_tree.update(i+1,nums[i]);
    }
    
    void update(int index, int val) {
        // int orig =  bit_tree.sum(index+1) - bit_tree.sum(index);
        int orig = bit_tree.read_single(index+1);
        bit_tree.update(index+1,val-orig);
    }
    
    int sumRange(int left, int right) {
        return bit_tree.sum(right+1) - bit_tree.sum(left);
    }
};

/**
 * Your NumArray object will be instantiated and called as such:
 * NumArray* obj = new NumArray(nums);
 * obj->update(index,val);
 * int param_2 = obj->sumRange(left,right);
 */
```