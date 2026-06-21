# Secure Env Probe

Install and enable all three addon folders:

- `SecureEnvProbeWriter`
- `SecureEnvProbeLoadInto`
- `SecureEnvProbeUseSecure`

The writer addon creates `SecureEnvProbeDB`. The other two addons test
`[LoadIntoEnvironment secure]` and `## UseSecureEnvironment: 1`.

Expected separate-secureenv result:

- `loadInto.envEqualsWriterEnv == false` and
  `useSecure.envEqualsWriterEnv == false`, proving the directive changed the
  file environment away from the normal writer env.
- `loadInto.primitiveSeen` and `useSecure.primitiveSeen` keep the original
  primitive value even after the writer rebinding, because the secure env has
  an own copied slot.
- `loadInto.beforeRead` and `useSecure.beforeRead` are nil/absent, because late
  addon globals were absent from the secure env copy and there is no `_G`
  fallback.
- `loadInto.afterSecureWrite == 7`, but
  `loadIntoAfter.insecureSees == 6`, because secure writes create/shadow an own
  secure env slot and do not write through to `_G`.
- `writer.afterAllAddons.useSecureLate == 16` after `useSecure` wrote `17`,
  for the same shadowing reason.
- `math` marker mutations cross both ways, because copied table globals share
  the same table reference.

Rejected models:

- `envEqualsWriterEnv == true` if the secure directives did not change the
  file environment.
- Secure files see the writer's primitive sentinel.
- Secure writes to `SecureEnvProbe_LoadIntoLate` / `SecureEnvProbe_UseSecureLate`
  update the value seen by insecure code.
- Late addon globals read through secure files would prove the old overlay
  model (`secureenv` with `__index = _G`), but the retail PrivateAurasUI
  cooldown wrapper probe did not observe that fallback.

Run `/seprobe` after login. Then `/reload` or logout to flush:

```text
WTF\Account\<ACCOUNT>\SavedVariables\SecureEnvProbeWriter.lua
```
