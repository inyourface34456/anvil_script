mod enchantments;
mod sequence;

fn main() {
    println!("Hello, world!");
}


// def combine(target: type[Enchantments.item], sacrifice: type[Enchantments.item]):
//     id = target.id
//     recipe = '(' + target.recipe + ') + (' + sacrifice.recipe + ')'
//     type = target.type
//     enchants, combine_cost = combine_enchants(target, sacrifice)

//     if combine_cost == 0:
//         return enchants
    
//     combine_cost += penalty[target.anvil_uses] + penalty[sacrifice.anvil_uses]
//     if combine_cost > 39:
//         return 'sequence too expensive'
    
//     anvil_uses = max(target.anvil_uses, sacrifice.anvil_uses) + 1
//     total_levels = target.total_levels + sacrifice.total_levels + combine_cost
//     total_armour = target.total_armour + sacrifice.total_armour
//     total_books = target.total_books + sacrifice.total_books
//     total_lapis = target.total_lapis + sacrifice.total_lapis

//     return Enchantments.item(id, recipe, type, enchants, anvil_uses, total_levels, total_armour, total_books, total_lapis)


// def combine_enchants(target: type[Enchantments.item], sacrifice: type[Enchantments.item]):
//     enchants_list = target.enchants
//     cost = 0

//     for e in sacrifice.enchants:
//         enchantment = e.enchantment
//         mult = enchantment.book_mult if sacrifice.type == 'Book' else enchantment.item_mult
//         counted = False

//         for f in range(0, len(enchants_list)):
//             if Enchantments.incompatible(enchantment, enchants_list[f].enchantment):
//                 return 'sequence contains incompatible enchantments', 0
//                 break
//             elif enchants_list[f].enchantment == enchantment:
//                 if e.level > enchants_list[f].level and e.level <= enchantment.max_level:
//                     enchants_list[f].level = e.level
//                     cost += enchants_list[f].level * mult
//                     counted = True
//                     break
//                 elif e.level == enchants_list[f].level and e.level < enchantment.max_level:
//                     enchants_list[f].level += 1
//                     cost += enchants_list[f].level * mult
//                     counted = True
//                     break
//                 else:
//                     return 'redundant sequence', 0
                     
//         if not counted:
//             enchants_list.append(e)
//             cost += e.level * mult

//     return enchants_list, cost