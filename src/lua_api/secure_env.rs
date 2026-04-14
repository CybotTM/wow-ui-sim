//! Secure environment helpers.

pub fn create_secure_environment<T>(_lua: &T) -> crate::Result<()> {
    Ok(())
}

pub fn apply_secure_env<L, F>(_lua: &L, _func: &F) -> crate::Result<()> {
    Ok(())
}

pub fn set_in_both_envs<L, V>(_lua: &L, _key: &str, _value: V) -> crate::Result<()> {
    Ok(())
}
