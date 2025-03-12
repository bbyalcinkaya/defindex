use soroban_sdk::{Address, Bytes, BytesN, Env, TryFromVal, TryIntoVal};


#[cfg(not(test))]
extern "C" {
    fn kasmer_create_contract(addr_val: u64, hash_val: u64) -> u64;
    fn kasmer_set_ledger_timestamp(x : u64);
    fn kasmer_address_from_bytes(addr_val: u64, is_contract: u64) -> u64;
}

#[cfg(not(test))]
pub fn create_contract<T, T2: >(env: &Env, addr: &T, hash: &T2) -> Address
    where
        T: TryIntoVal<Env, BytesN<32>>,
        T2: TryIntoVal<Env, BytesN<32>>
{
    use soroban_sdk::{FromVal, Val};

    let addr: BytesN<32> = addr.try_into_val(env).unwrap();
    let hash: BytesN<32> = hash.try_into_val(env).unwrap();

    unsafe {
        let res = kasmer_create_contract(addr.as_val().get_payload(), hash.as_val().get_payload());
        Address::from_val(env, &Val::from_payload(res))
    }
}

#[cfg(not(test))]
pub fn set_ledger_timestamp(env: &Env, x: u64) {
    use soroban_sdk::{Val, FromVal};

    unsafe {
        kasmer_set_ledger_timestamp(Val::from_val(env, &x).get_payload());
    }
}

#[cfg(not(test))]
pub fn address_from_bytes<T>(env: &Env, bs: &T, is_contract: bool) -> Address
    where Bytes: TryFromVal<Env, T>
{
    use soroban_sdk::{FromVal, Val};

    let bs: Bytes = Bytes::try_from_val(env, bs).unwrap();

    unsafe {
        let res = kasmer_address_from_bytes(
            Val::from_val(env, &bs).get_payload(),
            Val::from_val(env, &is_contract).get_payload()
        );
        Address::from_val(env, &Val::from_payload(res))
    }
}

#[cfg(test)]
pub fn create_contract<T, T2: >(env: &Env, addr: &T, hash: &T2) -> Address
    where
        T: TryIntoVal<Env, BytesN<32>>,
        T2: TryIntoVal<Env, BytesN<32>>
{

    let addr: BytesN<32> = addr.try_into_val(env).unwrap();
    let hash: BytesN<32> = hash.try_into_val(env).unwrap();
    env.deployer()
        .with_current_contract(addr)
        .deploy(hash)
}

#[cfg(test)]
pub fn set_ledger_timestamp(env: &Env, x: u64) {
    use soroban_sdk::testutils::Ledger;

    env.ledger().set_timestamp(x);
}

#[cfg(test)]
pub fn address_from_bytes<T>(env: &Env, _bs: &T, _is_contract: bool) -> Address
    where Bytes: TryFromVal<Env, T>
{
    use soroban_sdk::testutils::Address as _;

    Address::generate(env)
}

#[macro_export]
macro_rules! assume {
    ( $x:expr ) => {
        if ! $x {
            return true;
        }
    };
}
