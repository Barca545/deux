use super::basicstat::BasicStat;
use crate::new_basic_stat;

// HP and Mana stats
new_basic_stat!(Health, u32,);
new_basic_stat!(SpellResource, u32,);

// Defensive stats
new_basic_stat!(Armor, i32,);
new_basic_stat!(MagicResist, i32,);

// Offensive stats
new_basic_stat!(PhysicalDamage, u32,);
new_basic_stat!(MagicDamage, u32,);
new_basic_stat!(CooldownReduction, u32,);
new_basic_stat!(AttackSpeed, u32,);

// Movement stats
new_basic_stat!(UnitSpeed, f32,);
new_basic_stat!(MissleSpeed, f32,);
