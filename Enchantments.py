class item:
    def __init__(self, id, recipe, type, enchants, anvil_uses, total_levels, total_armour, total_books, total_lapis):
        self.id = id
        self.recipe = recipe
        self.type = type
        self.enchants = enchants
        self.anvil_uses = anvil_uses
        self.total_levels = total_levels
        self.total_armour = total_armour
        self.total_books = total_books
        self.total_lapis = total_lapis

class enchantment:
    def __init__(self, name, max_level, item_mult, book_mult):
        self.name = name
        self.max_level = max_level
        self.item_mult = item_mult
        self.book_mult = book_mult

class enchant:
    def __init__(self, enchantment, level):
        self.enchantment = enchantment
        self.level = level

aqua_affinity = enchantment('Aqua Affinity', 1, 4, 2)
blast_protection = enchantment('Blast Protection', 4, 4, 2)
curse_of_binding = enchantment('Curse of Binding', 1, 8, 4)
curse_of_vanishing = enchantment('Curse of Vanishing', 1, 8, 4)
depth_strider = enchantment('Depth Strider', 3, 4, 2)
feather_falling = enchantment('Feather Falling', 4, 2, 1)
fire_protection = enchantment('Fire Protection', 4, 2, 1)
frost_walker = enchantment('Frost Walker', 2, 4, 2)
mending = enchantment('Mending', 1, 4, 2)
projectile_protection = enchantment('Projectile Protection', 4, 2, 1)
protection = enchantment('Protection', 4, 1, 1)
respiration = enchantment('Respiration', 3, 4, 2)
soul_speed = enchantment('Soul Speed', 3, 8, 4)
swift_sneak = enchantment('Swift Sneak', 3, 8, 4)
thorns = enchantment('Thorns', 3, 8, 4)
unbreaking = enchantment('Unbreaking', 3, 2, 1)

exclusive_1 = [blast_protection, fire_protection, projectile_protection, protection]
exclusive_2 = [depth_strider, frost_walker]

def incompatible(enchantment_1, enchantment_2):
    if enchantment_1 != enchantment_2:
        if enchantment_1 in exclusive_1 and enchantment_2 in exclusive_1:
            return True
        if enchantment_1 in exclusive_2 and enchantment_2 in exclusive_2:
            return True
    return False


items_breastplate = [
    item(1, '1', 'Armour', [
        enchant(blast_protection, 1)], 0, 1, 1, 0, 1),
    item(2, '2', 'Armour', [
        enchant(blast_protection, 1)], 0, 1, 1, 0, 1),
    item(3, '3', 'Armour', [
        enchant(blast_protection, 1)], 0, 1, 1, 0, 1),
    item(4, '4', 'Armour', [
        enchant(blast_protection, 1)], 0, 1, 1, 0, 1),
    item(5, '5', 'Armour', [
        enchant(blast_protection, 1)], 0, 1, 1, 0, 1),
    item(6, '6', 'Armour', [
        enchant(blast_protection, 1)], 0, 1, 1, 0, 1),
    item(7, '7', 'Armour', [
        enchant(blast_protection, 1)], 0, 1, 1, 0, 1),
    item(8, '8', 'Armour', [
        enchant(blast_protection, 1)], 0, 1, 1, 0, 1),
    item(9, '9', 'Armour', [
        enchant(thorns, 1)], 0, 1, 1, 0, 1),
    item(10, '10', 'Armour', [
        enchant(thorns, 1)], 0, 1, 1, 0, 1),
    item(11, '11', 'Armour', [
        enchant(thorns, 1)], 0, 1, 1, 0, 1),
    item(12, '12', 'Armour', [
        enchant(thorns, 1)], 0, 1, 1, 0, 1),
    item(13, '13', 'Armour', [
        enchant(unbreaking, 1)], 0, 1, 1, 0, 1),
    item(14, '14', 'Armour', [
        enchant(unbreaking, 1)], 0, 1, 1, 0, 1),
    item(15, '15', 'Armour', [
        enchant(unbreaking, 1)], 0, 1, 1, 0, 1),
    item(16, '16', 'Armour', [
        enchant(unbreaking, 1)], 0, 1, 1, 0, 1),
    item(17, '17', 'Book', [
        enchant(mending, 1),
        enchant(blast_protection, 1)], 0, 1, 0, 1, 1),
    item(18, '18', 'Book', [
        enchant(curse_of_binding, 1),
        enchant(blast_protection, 1)], 0, 1, 0, 1, 1),
    item(19, '19', 'Book', [
        enchant(curse_of_vanishing, 1),
        enchant(blast_protection, 1)], 0, 1, 0, 1, 1)]