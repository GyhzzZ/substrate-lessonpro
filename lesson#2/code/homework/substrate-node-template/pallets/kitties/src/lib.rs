#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    sp_runtime::traits::AtLeast32BitUnsigned,
    traits::{Currency, Get, LockIdentifier, LockableCurrency, Randomness, WithdrawReasons},
    Parameter, StorageMap,
};
use frame_system::ensure_signed;
use sp_io::hashing::blake2_128;
use sp_runtime::{traits::Bounded, DispatchError};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests_create;

#[cfg(test)]
mod tests_transfer;

#[cfg(test)]
mod tests_breed;

const BASE_KITTY_ID: LockIdentifier = [66, 0, 0, 0, 0, 0, 0, 0];

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

#[derive(Default, Encode, Decode, Clone)]
pub struct KittyRelation<KittyIndex> {
    pub father: KittyIndex,
    pub mother: KittyIndex,
    pub children: Vec<KittyBreed<KittyIndex>>,
}

#[derive(Default, Encode, Decode, Clone)]
pub struct KittyBreed<KittyIndex> {
    pub breed: KittyIndex,
    pub children: KittyIndex,
}

pub type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
    type KittyIndex: Parameter + AtLeast32BitUnsigned + Bounded + Default + Copy;
    type Currency: LockableCurrency<Self::AccountId>;
    type KittyLockToken: Get<BalanceOf<Self>>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
        pub KittiesCount get(fn kitties_count): T::KittyIndex;
        pub KittyOwners get(fn kitty_owners): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;
        pub UserKitties get(fn user_kitties): map hasher(blake2_128_concat) T::AccountId => Vec<T::KittyIndex>;
        pub KittyRelations get(fn kitty_relations): map hasher(blake2_128_concat) T::KittyIndex => KittyRelation<T::KittyIndex>;
        pub LockIndexd get(fn lock_index): u32;
        pub LockId get(fn lock_id): map hasher(blake2_128_concat) T::KittyIndex => u32;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        KittyIndex = <T as Trait>::KittyIndex,
    {
        Created(AccountId, KittyIndex), // 创建Kitty事件，owner--kitty下标
        Transferred(AccountId, AccountId, KittyIndex), //发送Kitty事件，from--to--kitty下标
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        //Kitty数量超出最大限制--错误
        KittiesCountOverflow,
        //非法的KittyId--错误
        InvaildKittyId,
        //父母Id不可相同--错误
        RequireDifferentParent,
        //不是kitty的拥有者--错误
        NotKittyOwner,
        //余额不足质押--错误
        FreeNotEnough,
        //LockIndex数量超出最大限制--错误
        LockIndexOverflow,
    }
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    (selector & dna1) | (!selector & dna2)
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        const KittyLockToken: BalanceOf<T> = T::KittyLockToken::get();

        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 0]
        pub fn create(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;
            let lock_id = Self::next_lock_index()?;
            //检查余额
            Self::check_balance(&sender)?;
            //质押token
            Self::lock_token(&sender,kitty_id,lock_id);
            let dna = Self::random_value(&sender);
            let kitty = Kitty(dna);
            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::set_lock_index(lock_id + 1);
            Self::deposit_event(RawEvent::Created(sender,kitty_id));
            Ok(())
        }

        #[weight = 0]
        pub fn transfer(origin, to: T::AccountId, kitty_index: T::KittyIndex) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::check_balance(&to)?;
            Self::transfer_kitty(&sender,&to,kitty_index)?;
            Self::transfer_lock(&sender,&to,kitty_index);
            Self::deposit_event(RawEvent::Transferred(sender, to, kitty_index));
            Ok(())
        }

        #[weight = 0]
        pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let lock_id: u32 = Self::next_lock_index()?;
            // ensure!(1==2,Error::<T>::FreeNotEnough);
            //检查余额
            Self::check_balance(&sender)?;

            //繁殖
            let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;
            //质押token
            Self::lock_token(&sender,new_kitty_id,lock_id);
            Self::set_lock_index(lock_id + 1);
            Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    //插入kitty
    fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
        <Kitties<T>>::insert(kitty_id, kitty);
        <KittiesCount<T>>::put(kitty_id + 1.into());
        <KittyOwners<T>>::insert(kitty_id, owner);

        //记录用户拥有的kitty
        let mut my_kitties = Self::user_kitties(owner);
        my_kitties.push(kitty_id);
        <UserKitties<T>>::insert(owner, my_kitties);
    }

    // 转移kitty--实现
    fn transfer_kitty(
        from: &T::AccountId,
        to: &T::AccountId,
        kitty_id: T::KittyIndex,
    ) -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        //检查owner
        let owner = Self::kitty_owners(kitty_id).ok_or(Error::<T>::InvaildKittyId)?;
        ensure!(from.clone() == owner, Error::<T>::NotKittyOwner);

        //移除owner的kitty
        let mut from_kitties = Self::user_kitties(from.clone());
        for i in 0..from_kitties.len() {
            if from_kitties[i].clone() == kitty_id {
                from_kitties.remove(i);
                break;
            }
        }
        <UserKitties<T>>::insert(from.clone(), from_kitties);

        //修改kitty的owner
        <KittyOwners<T>>::insert(kitty_id, to.clone());

        //增加to的kitty
        let mut to_kitties = Self::user_kitties(to.clone());
        to_kitties.push(kitty_id);
        <UserKitties<T>>::insert(to, to_kitties);

        Ok(kitty_id)
    }

    //获取下一个kitty的id
    fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        let kitty_id = Self::kitties_count();

        if kitty_id == T::KittyIndex::max_value() {
            return Err(Error::<T>::KittiesCountOverflow.into());
        }
        Ok(kitty_id)
    }

    //获取下一个质押Id
    fn next_lock_index() -> sp_std::result::Result<u32, DispatchError> {
        let lock_id: u32 = <LockIndexd>::get();
        if lock_id >= 9999999 {
            return Err(Error::<T>::LockIndexOverflow.into());
        }
        Ok(lock_id)
    }

    //检查余额
    fn check_balance(sender: &T::AccountId) -> sp_std::result::Result<(), DispatchError> {
        if T::Currency::free_balance(sender) < T::KittyLockToken::get() {
            return Err(Error::<T>::FreeNotEnough.into());
        }
        Ok(())
    }

    //获取一个随机dna
    fn random_value(sender: &T::AccountId) -> [u8; 16] {
        let payload = (
            T::Randomness::random_seed(),
            &sender,
            <frame_system::Module<T>>::extrinsic_index(),
        );

        payload.using_encoded(blake2_128)
    }

    //繁殖kitty
    fn do_breed(
        sender: &T::AccountId,
        kitty_id_1: T::KittyIndex,
        kitty_id_2: T::KittyIndex,
    ) -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvaildKittyId)?;
        let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvaildKittyId)?;

        ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

        let kitty_id = Self::next_kitty_id()?;

        let kitty1_dna = kitty1.0;
        let kitty2_dna = kitty2.0;
        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i])
        }

        Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

        //更新kitty的关系
        Self::new_kitty_relation(kitty_id, kitty_id_1, kitty_id_2);

        Ok(kitty_id)
    }

    //更新kitty关系
    fn new_kitty_relation(kitty_id: T::KittyIndex, father: T::KittyIndex, mother: T::KittyIndex) {
        //建立默认的relation
        Self::_build_default_relation(kitty_id, father, mother);
        //为父/母增加孩子
        Self::_add_children(kitty_id, father, mother);
    }

    //建立默认的relation
    fn _build_default_relation(
        kitty_id: T::KittyIndex,
        father: T::KittyIndex,
        mother: T::KittyIndex,
    ) {
        let new_relation = KittyRelation {
            father: father.clone(),
            mother: mother.clone(),
            ..Default::default()
        };
        <KittyRelations<T>>::insert(kitty_id, new_relation);
    }

    //为父/母增加孩子
    fn _add_children(kitty_id: T::KittyIndex, father_id: T::KittyIndex, mother_id: T::KittyIndex) {
        //修改父增加孩子
        let father_kitty_breed = KittyBreed {
            breed: mother_id.clone(),
            children: kitty_id.clone(),
        };
        <KittyRelations<T>>::mutate(&father_id, |relation| {
            relation.children.push(father_kitty_breed.clone())
        });

        //修改母增加孩子
        let mother_kitty_breed = KittyBreed {
            breed: father_id.clone(),
            children: kitty_id.clone(),
        };
        <KittyRelations<T>>::mutate(&mother_id, |relation| {
            relation.children.push(mother_kitty_breed.clone())
        });
    }

    //获取kitty的brothers
    fn get_brothers(kitty_id: T::KittyIndex) -> Vec<T::KittyIndex> {
        let my_relation = Self::kitty_relations(kitty_id);
        let father_relation = Self::kitty_relations(my_relation.father);
        let mut my_brothers = Vec::new();

        for i in father_relation.children.clone() {
            if i.breed == my_relation.mother {
                my_brothers.push(i.children);
            }
        }
        my_brothers
    }

    //质押
    fn lock_token(sender: &T::AccountId, kitty_id: T::KittyIndex, lock_id: u32) {
        let _lock_id = Self::_decode_kitty_lock_id(lock_id);
        T::Currency::set_lock(
            _lock_id,
            &sender,
            T::KittyLockToken::get(),
            WithdrawReasons::all(),
        );
        //绑定kitty-质押id
        <LockId<T>>::insert(kitty_id, lock_id);
    }

    //转移质押
    fn transfer_lock(sender: &T::AccountId, to: &T::AccountId, kitty_id: T::KittyIndex) {
        let lock_id = Self::lock_id(kitty_id);
        let _lock_id = Self::_decode_kitty_lock_id(lock_id);
        T::Currency::remove_lock(_lock_id, sender);
        Self::lock_token(to, kitty_id, lock_id);
    }

    //获取质押id
    fn _decode_kitty_lock_id(kitty_id: u32) -> LockIdentifier {
        let mut lock_id = BASE_KITTY_ID.clone();
        // let kitty_id = kitty_id as u32;
        lock_id[7] = (kitty_id / 1 % 10) as u8;
        lock_id[6] = (kitty_id / 10 % 10) as u8;
        lock_id[5] = (kitty_id / 100 % 10) as u8;
        lock_id[4] = (kitty_id / 1000 % 10) as u8;
        lock_id[3] = (kitty_id / 10000 % 10) as u8;
        lock_id[2] = (kitty_id / 100000 % 10) as u8;
        lock_id[1] = (kitty_id / 1000000 % 10) as u8;
        lock_id
    }

    fn set_kitties_count(value: T::KittyIndex) {
        <KittiesCount<T>>::put(value);
    }

    fn set_lock_index(value: u32) {
        LockIndexd::put(value);
    }
}
