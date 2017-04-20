
pub struct Msg<'a> {
    pub sequence: usize,
    pub data: &'a [u8],
}

pub struct RawMsg<'a> {
    pub data: &'a [u8],
}

pub trait History<'a> {
    fn append(&mut self, raw: RawMsg) -> Msg<'a>;
}

pub struct DefaultHistory<'a> {
    sequence: usize,
    position: usize,
    data: &'a mut [u8]
}

impl <'a> DefaultHistory<'a> {
    pub fn new(data:  &'a mut [u8]) -> Self {
        DefaultHistory { 
            sequence: 0,
            position: 0,
            data: data
        }
    }
}

impl <'a> History<'a> for DefaultHistory<'a> {
    fn append(&mut self, raw: RawMsg) -> Msg<'a> {


        let start = self.position;
        let end = start + raw.data.len(); 


        self.data[start .. end].clone_from_slice(&raw.data);

        self.position = end;
        self.sequence += 1;

        let ptr = self.data.as_mut_ptr();

        Msg { 
            sequence: self.sequence,
            data: unsafe {
                use std::slice::from_raw_parts_mut;
                from_raw_parts_mut(ptr.offset(start as isize), end - start)
            }
        }
    }
}

pub trait Processor {
    fn process<'a>(&mut self, msg: Msg<'a>);
}

pub struct Centrifuge<'a, T: 'a, P: 'a> {
    history: &'a mut T,
    processor: &'a mut P,
}

impl <'a, T, P> Centrifuge<'a, T, P> 
    where T: History<'a>,
          P: Processor + 'a
{ 
    pub fn new(history: &'a mut T, processor: &'a mut P) -> Self {
        Centrifuge { 
            history: history,
            processor: processor
        }
    }

    pub fn receive_msg(&mut self, data: &[u8]) {
        let raw = RawMsg { data: data };
        let msg = self.history.append(raw);
        // TODO handle panics. We should be able to pinpoint the exact message that caused a panics
        // and then take appropriate action and possibly recover.
        self.processor.process(msg);
    }
}



