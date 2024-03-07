use super::basicstat::{BasicGrowableStat, BasicStat};

//HP and Mana stats
///Component containing the health of an entity.
pub type Health = BasicGrowableStat<BaseHealth, i32>;
///Component containing the spell resource of an entity.
pub type SpellResource = BasicGrowableStat<BaseSpellResource, i32>;

//Defensive stats
///Component containing the armor of an entity.
pub type Armor = BasicStat<BaseArmor, i32>;
///Component containing the magic resist of an entity.
pub type MagicResist = BasicStat<BaseMagicResist, i32>;

//Offensive stats
///Component containing the attack damage of an entity.
pub type AttackDamage = BasicStat<BaseAttackDamage, i32>;
///Component containing the magic damage of an entity.
pub type MagicDamage = BasicStat<BaseMagicDamage, i32>;
///Component containing the cooldown reduction of an entity.
pub type CooldownReduction = BasicStat<BaseCooldownReduction, i32>;
///Component containing the attack speed of an entity.
pub type AttackSpeed = BasicStat<BaseAttackSpeed, i32>;

//Movement stats
///Component holding a unit entity's speed.
pub type UnitSpeed = BasicStat<BaseUnitSpeed, f32>;
///Component containing the speed of a missle entity.
pub type MissleSpeed = BasicStat<BaseMissleSpeed, f32>;

#[derive(Debug, Clone, Copy)]
pub struct BaseHealth;

#[derive(Debug, Clone, Copy)]
pub struct BaseSpellResource;

#[derive(Debug, Clone, Copy)]
pub struct BaseArmor;

#[derive(Debug, Clone, Copy)]
pub struct BaseMagicResist;

#[derive(Debug, Clone, Copy)]
pub struct BaseAttackDamage;

#[derive(Debug, Clone, Copy)]
pub struct BaseMagicDamage;

#[derive(Debug, Clone, Copy)]
pub struct BaseCooldownReduction;

#[derive(Debug, Clone, Copy)]
pub struct BaseAttackSpeed;

#[derive(Debug, Clone, Copy)]
pub struct BaseUnitSpeed;

#[derive(Debug, Clone, Copy)]
pub struct BaseMissleSpeed;
