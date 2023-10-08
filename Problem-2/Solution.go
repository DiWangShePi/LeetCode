package main

// Definition for singly-linked list.
 type ListNode struct {
     Val int
     Next *ListNode
}

func addTwo(l1, l2 *ListNode, carry int) *ListNode {
    if l1 == nil && l2 == nil { 
        if carry != 0 {
            return &ListNode{Val: carry} 
        }
        return nil
    }
    if l1 == nil { 
        l1, l2 = l2, l1
    }
    carry += l1.Val 
    if l2 != nil {
        carry += l2.Val 
        l2 = l2.Next
    }
    l1.Val = carry % 10 
    l1.Next = addTwo(l1.Next, l2, carry/10)
    return l1
}

func addTwoNumbers(l1, l2 *ListNode) *ListNode {
    return addTwo(l1, l2, 0)
}