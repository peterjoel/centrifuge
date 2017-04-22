use centrifuge::Store;
use centrifuge::Message;
use centrifuge::RawMsg;

pub struct PermStore<'a> {
    sequence: usize,
    position: usize,
    data: &'a mut [u8]
}

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
    fn get_slice(&mut self, from: usize, len: usize) -> &'a [u8] {
        use std::slice::from_raw_parts_mut;
        let ptr = self.data.as_mut_ptr();
        unsafe {
            from_raw_parts_mut(ptr.offset(from as isize), len)
        }
    }
    
    #[inline]
    fn get_slice_from_to(&mut self, from: usize, to: usize) -> &'a [u8] {
        self.get_slice(from, to - from)
    }

}

impl <'a> Message<'a> for PermMsg<'a> {
    fn get_sequence(&self) -> usize {
        self.sequence
    }
    fn get_data(&self) -> &'a [u8] {
        self.data
    }
}

impl <'a> Store<'a> for PermStore<'a> {
    type Msg = PermMsg<'a>;
    fn append(&mut self, raw: RawMsg) -> PermMsg<'a> {
        let start = self.position;
        let end = start + raw.data.len(); 

        self.data[start .. end].clone_from_slice(&raw.data);
        self.position = end;
        self.sequence += 1;

        PermMsg { 
            sequence: self.sequence,
            data: self.get_slice_from_to(start, end)
        }
    }
}