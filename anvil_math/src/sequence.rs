use rand::Rng;

use crate::enchantments::Item;

pub struct Sequence {
    id: usize,
    item_list: Vec<Item>,
    current_index: usize,
}

impl Sequence {
    //     def __init__(self, id, item_list, current_index=0):
    //         self.id = id
    //         self.item_list = item_list
    //         self.current_index = current_index
    pub fn new(id: usize, item_list: Vec<Item>, current_index: Option<usize>) -> Self {
        Self {
            id,
            item_list,
            current_index: current_index.unwrap_or(0)
        }
    }

    // def next_sequence():
    //     sequence.num += 1
    //     return sequence(sequence.num, Enchantments.items_breastplate.copy())
   
    
    //     def next_step(self):
    //         num_items = len(self.item_list)
    //         if num_items > 1:
    //             sacrifice = self.item_list[round(random.uniform(0, num_items - 1))]
    //             self.item_list.remove(sacrifice)
    //             self.current_index = round(random.uniform(0, num_items - 2))
    //             target = self.item_list[self.current_index]
    //             return (False, target, sacrifice)
    //         self.finish(self.item_list[0])
    //         return (True, self.item_list[0])
    
    pub fn next_step(&mut self) -> (bool, Item, Option<Item>) {
        let mut rng = rand::thread_rng();
        if self.item_list.len() > 1 {
            let sacrfice = self.item_list.remove(rng.gen_range(0..self.item_list.len()-1));
            let target = self.item_list[rng.gen_range(0..self.item_list.len()-1)].clone();
            return (false, target, Some(sacrfice));
        }
        self.finish(format!("{:?}", self.item_list[0]));
        (true, self.item_list[0].clone(), None)
    }
    
    //     def update_item_list(self, target, sacrifice, result):
    //         self.item_list[self.current_index] = result
    
    pub fn update_item_list(&mut self, _target: Item, _sacfrfice: Item, result: Item) {
        self.item_list[self.current_index] = result;
    }
    
    //     def finish(self, result=0, report='No new maximum.'):
    //         report_num = random.uniform(1, 20)
    //         if report_num <= 1:
    //             report = 'redundant sequence'
    //         elif report_num <= 5:
    //             report = 'sequence too expensive'
    //         else:
    //             report = 'no new maximum'
    
    //         if self.id%100000 == 0:
    //             print('Sequence #', self.id, ' completed: ', report, sep='')

    fn finish(&self, report: String) {
        if self.id %100_000 == 0 {
            println!("{report}");
        }
    }
}