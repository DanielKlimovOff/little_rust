// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    fn list_to_vector(list: Option<Box<ListNode>>) -> Vec<i32>{
        let mut result = Vec::new();
        if list == None {
            return result;
        }
        let mut node = &list.unwrap();
        result.push(node.val);

        while let Some(elem) = &node.next {
            result.push(elem.val);
            node = elem;
        }

        result
    }
    
    pub fn merge_two_lists(list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_vec = Solution::list_to_vector(list1);
        println!("{:?}", l1_vec);
        let mut l2_vec = Solution::list_to_vector(list2);
        println!("{:?}", l2_vec);
        l1_vec
            .append(&mut l2_vec)
            .sort()
            .reverce();
    

        if l1_vec.len() == 0 {
            return None;
        }

        let mut result = Box::new(ListNode::new(l1_vec.pop()));
        let mut iter = &mut result;
        while l1_vec.len() > 0 {
            iter.next = Box::new(ListNode::new(l1_vec.pop()));
            iter = iter.next;
        }
        
        result
    }
}