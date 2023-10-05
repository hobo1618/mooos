use super::align_up;
use core::mem;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}


impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    /// Adds the given memory region to the front of the list.
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // ensure that the freed region is capable of holding the ListNode?
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        // create new node in freed region.
        // the node has a known size but its
        // `next` field is set to None.
        let mut node = ListNode::new(size);
        
        // Now its `next` field is set to the
        // value of the value of Some(head.next), 
        // which initially is None, so this is confusing.
        // After the first freed region is added to the linked
        // list, however, the value of head is the 
        // address of that node. 
        node.next = self.head.next.take();

        // Now we take the addr and coerce it
        // into a [raw pointer](https://doc.rust-lang.org/std/primitive.pointer.html#common-ways-to-create-raw-pointers)
        let node_ptr = addr as *mut ListNode;
        // then we invoke the write method, which 
        // overwrites the memory at location addr (https://doc.rust-lang.org/std/primitive.pointer.html#method.write)
        // and stores the node data there.
        node_ptr.write(node);

        // then we set the head.next value to the
        // the value of the raw pointer of the new node
        self.head.next = Some(&mut *node_ptr)
    }

    fn find_region(&mut self, size: usize, align: usize) 
        -> Option<(&'static mut ListNode, usize)>
    {
        // reference to current list node, starting at head, and updated for each iteration
        let mut current = &mut self.head;
        // look for a large enough memory region in linked list
        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                // region suitable for allocation -> remove node from list
                // the take() method on an option removes the Some value out
                // of the Option and leaves None in its place. 
                // the pickpocket method.
                
                // next gets its next pointer robbed
                let next = region.next.take();

                // current gets robbed
                let ret = Some((current.next.take().unwrap(), alloc_start));
                // but then gets region.next's pointer instead.
                current.next = next;

                // the return value is a pointer to the region start.
                return ret;
            } else {
                // region not suitable -> continue with next region
                current = current.next.as_mut().unwrap();
            }
        }

        // no suitable region found
        None
    }
}
