mod enchantments;

use enchantments::{EnchantTypes, Enchantment};
pub use enchantments::{Item, Enchant};

pub const AquaAffinity: Enchantment = Enchantment::new(EnchantTypes::AquaAffinity, 1, 4, 2);
pub const BlastProtection: Enchantment = Enchantment::new(EnchantTypes::BlastProtection, 4, 4, 2);
pub const CurseOfBinding: Enchantment = Enchantment::new(EnchantTypes::CurseOfBinding, 1, 8, 4);
pub const CurseOfVanishing: Enchantment = Enchantment::new(EnchantTypes::CurseOfVanishing, 1, 8, 4);
pub const DepthStrider: Enchantment = Enchantment::new(EnchantTypes::DepthStrider, 3, 4, 2);
pub const FeatherFalling: Enchantment = Enchantment::new(EnchantTypes::FeatherFalling, 4, 2, 1);
pub const FireProtection: Enchantment = Enchantment::new(EnchantTypes::FireProtection, 4, 2, 1);
pub const FrostWalker: Enchantment = Enchantment::new(EnchantTypes::FrostWalker, 2, 4, 2);
pub const Mending: Enchantment = Enchantment::new(EnchantTypes::Mending, 1, 4, 2);
pub const ProjectileProtection: Enchantment = Enchantment::new(EnchantTypes::ProjectileProtection, 4, 2, 1);
pub const Protection: Enchantment = Enchantment::new(EnchantTypes::Protection, 4, 1, 1);
pub const Respiration: Enchantment = Enchantment::new(EnchantTypes::Respiration, 3, 4, 2);
pub const SoulSpeed: Enchantment = Enchantment::new(EnchantTypes::SoulSpeed, 3, 8, 4);
pub const SwiftSneak: Enchantment = Enchantment::new(EnchantTypes::SwiftSneak, 3, 8, 4);
pub const Thorns: Enchantment = Enchantment::new(EnchantTypes::Thorns, 3, 8, 4);
pub const Unbreaking: Enchantment = Enchantment::new(EnchantTypes::Unbreaking, 3, 2, 1);