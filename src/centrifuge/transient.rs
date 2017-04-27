use centrifuge::Store;
use centrifuge::Message;

pub struct TransientStore {
    sequence: usize,
    max_msg_size: usize,
}

#[derive(Debug)]
pub struct TransientMsg {
    sequence: usize,
    data: Vec<u8>,
}

impl TransientStore {
    pub fn new(max_size: usize) -> Self {
        TransientStore { 
            sequence: 0,
            max_msg_size: max_size,
        }
    }
}

impl Message for TransientMsg {
    fn get_sequence(&self) -> usize {
        self.sequence
    }

    fn get_data(&self) -> &[u8] {
        &self.data[..]
    }
}

impl <'a> Store<'a> for TransientStore {
    type Msg = TransientMsg;

    fn write<W>(&mut self, writer: W) -> TransientMsg
        where W: FnOnce(&mut [u8]) -> usize
    {
        // This isn't a good approach. Better to allocate one buffer for the Store and use the same
        // approach as PermStore, but overwrite the data each time.
        let mut data = vec![0; self.max_msg_size];
        writer(&mut data);
        data.shrink_to_fit();
        self.sequence += 1;
        TransientMsg { 
            sequence: self.sequence,
            data: data,
        }
    }

}




#[cfg(test)]
pub mod test {
    use centrifuge::*;

    #[test]
    pub fn test_perm() {
        use centrifuge::transient::*;
        
        let mut sequence_nums = Vec::<usize>::new();

        {
            let mut store = TransientStore::new(100);

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
    }
}