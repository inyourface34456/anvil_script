import Enchantments
import random

class sequence:
    max_levels = 0
    num = 0

    def __init__(self, id, item_list, current_index=0):
        self.id = id
        self.item_list = item_list
        self.current_index = current_index

    def next_step(self):
        num_items = len(self.item_list)
        if num_items > 1:
            sacrifice = self.item_list[round(random.uniform(0, num_items - 1))]
            self.item_list.remove(sacrifice)
            self.current_index = round(random.uniform(0, num_items - 2))
            target = self.item_list[self.current_index]
            return (False, target, sacrifice)
        print("strange line")
        self.finish(self.item_list[0])
        return (True, self.item_list[0])

    def update_item_list(self, target, sacrifice, result):
        self.item_list[self.current_index] = result

    def finish(self, result=0, report='No new maximum.'):
        # report_num = random.uniform(1, 20)
        # if report_num <= 1:
        #     report = 'redundant sequence'
        # elif report_num <= 5:
        #     report = 'sequence too expensive'
        # else:
        #     report = 'no new maximum'

        if self.id%100000==0:
            print('Sequence #', self.id, ' completed: ', report, sep='')

def next_sequence():
    sequence.num += 1
    return sequence(sequence.num, Enchantments.items_breastplate.copy())
