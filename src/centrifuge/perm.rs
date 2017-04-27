use centrifuge::{Store, Message};

pub struct PermStore<'a> {
    sequence: usize,
    position: usize,
    data: &'a mut [u8]
}

#[derive(Debug)]
pub struct PermMsg<'a> {
    sequence: usize,
    data: &'a [u8],
}

impl <'a> PermStore<'a> {
    pub fn new(data:  &'a mut [u8]) -> Self {
        PermStore { 
            sequence: 0,
            position: 0,
            data: data
        }
    }
    
    #[inline]
    fn get_slice(&mut self, from: usize, len: usize) -> &'a mut [u8] {
        use std::slice::from_raw_parts_mut;
        let ptr = self.data.as_mut_ptr();
        unsafe {
            from_raw_parts_mut(ptr.offset(from as isize), len)
        }
    }
    
    #[inline]
    fn get_slice_from_to(&mut self, from: usize, to: usize) -> &'a mut [u8] {
        self.get_slice(from, to - from)
    }

    #[inline]
    fn get_remaining_slice(&mut self) -> &'a mut [u8] {
        let start = self.position;
        let end = self.data.len();
        self.get_slice_from_to(start, end)
    }
}

impl <'a> Message for PermMsg<'a> {
    fn get_sequence(&self) -> usize {
        self.sequence
    }
    fn get_data(&self) -> &[u8] {
        self.data
    }
}

impl <'a> Store<'a> for PermStore<'a> {
    type Msg = PermMsg<'a>;
    
    fn write<W>(&mut self, writer: W) -> PermMsg<'a> 
        where W: FnOnce(&mut [u8]) -> usize
    {
        let start = self.position;
        self.position += writer(&mut self.get_remaining_slice());
        self.sequence += 1;
        let seq = self.sequence;
        let pos = self.position;
        PermMsg { 
            sequence: seq,
            data: self.get_slice_from_to(start, pos)
        }
    }
}




#[cfg(test)]
pub mod test {
    use centrifuge::*;

    #[test]
    pub fn test_perm() {
        use centrifuge::perm::*;
        
        let mut sequence_nums = Vec::<usize>::new();
        let mut log = vec![0; 16];

        {
            let mut store = PermStore::new(&mut log);

            let mut centrifuge = Centrifuge::new(&mut store, |msg| {
                println!("Message received: {:?}", msg);
                sequence_nums.push(msg.get_sequence());
            });

            let data = b"hello";
            centrifuge.receive_msg( &data[..] );

            let data = b"bye";
            centrifuge.receive_msg( &data[..] );
        }


        assert_eq!(&vec![1, 2], &sequence_nums);
        assert_eq!(&b"hellobye"[..], &log[0..8]);
    }
}