#[derive(Debug, Clone)]
pub struct Item {
    pub id: usize,
    pub recipe: String,
    pub item_type: ItemType,
    pub enchants: Vec<Enchant>,
    pub anvil_uses: usize,
    pub total_levels: usize,
    pub total_armor: usize,
    pub total_books: usize,
    pub total_lapis: usize,
}

impl Item {
    pub fn new(id: usize,
                recipe: String,
                item_type: ItemType,
                enchants: Vec<Enchant>,
                anvil_uses: usize,
                total_levels: usize,
                total_armor: usize,
                total_books: usize,
                total_lapis: usize)
    -> Self {
        Self {
            id,
            recipe,
            item_type,
            enchants,
            anvil_uses,
            total_levels,
            total_armor,
            total_books,
            total_lapis,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Book, 
    Armor
}

#[derive(Debug, Clone, Copy)]
pub struct Enchant {
    enchantment: Enchantment,
    level: usize,
} 

impl Enchant {
    pub fn new(enchantment: Enchantment, level: usize) -> Self {
        Self {
            enchantment,
            level,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Enchantment {
    name: EnchantTypes,
    max_level: usize,
    item_mult: usize,
    book_mult: usize
}

impl Enchantment {
    const EXCLUSIVE1: [EnchantTypes; 4] = [EnchantTypes::BlastProtection, EnchantTypes::FireProtection, EnchantTypes::ProjectileProtection, EnchantTypes::Protection];
    const EXCLUSIVE2: [EnchantTypes; 2] = [EnchantTypes::DepthStrider, EnchantTypes::FrostWalker];
    
    pub const fn new(name: EnchantTypes, max_level: usize, item_mult: usize, book_mult: usize) -> Self {
        Self {
            name,
            max_level,
            item_mult,
            book_mult
        }
    }

    pub fn is_compatible(&self, e2: Self) -> bool {
        if Self::EXCLUSIVE1.iter().any(|v| v == &self.name) && Self::EXCLUSIVE1.iter().any(|v| v == &e2.name) {
            true
        } else if Self::EXCLUSIVE2.iter().any(|v| v == &self.name) && Self::EXCLUSIVE2.iter().any(|v| v == &e2.name) {
            true
        } else {
            false
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnchantTypes {
    AquaAffinity,
    BlastProtection,
    CurseOfBinding,
    CurseOfVanishing,
    DepthStrider,
    FeatherFalling,
    FireProtection,
    FrostWalker,
    Mending,
    ProjectileProtection,
    Protection,
    Respiration,
    SoulSpeed,
    SwiftSneak,
    Thorns,
    Unbreaking,
}