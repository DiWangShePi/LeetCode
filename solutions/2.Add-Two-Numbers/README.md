# 2. Add Two Numbers

### 题目描述

给出两个 **非空** 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 **逆序** 的方式存储的，并且它们的每个节点只能存储 **一位** 数字。

如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

**示例:**

```
输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
原因：342 + 465 = 807
```

### 题目解析

保存两个指针，分别指向链表一和链表二。同时遍历两条链表，并保存临时变量`carry`（初始化为0）作为进位。

在每一个元素的遍历中，计算链表一、链表二和`carry`三个元素的和，获取链表三新元素的值和新的进位。

链表三的长度不小于 max(链表一，链表二)，在遍历结束后，依据进位是否为0判断是否需要创建新的元素。

### 代码实现

###### c++

``` c++
class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode dummy; 
        ListNode* pointer = &dummy;
        int carry = 0;

        while (l1 != nullptr || l2 != nullptr || carry != 0) {
            int sum = carry;
            if (l1 != nullptr) {
                sum += l1->val;
                l1 = l1->next;
            }
            if (l2 != nullptr) {
                sum += l2->val;
                l2 = l2->next;
            }
            carry = sum / 10;
            pointer->next = new ListNode(sum % 10);
            pointer = pointer->next;
        }

        return dummy.next;
    }
};
```

###### Rust
``` rust
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);

            carry = sum / 10;
            let new_node = ListNode::new(sum % 10);
            current.next = Some(Box::new(new_node));
            current = current.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy_head.next
    }
}
```