# Secure Env Probe

Install and enable all three addon folders:

- `SecureEnvProbeWriter`
- `SecureEnvProbeLoadInto`
- `SecureEnvProbeUseSecure`

The writer addon creates `SecureEnvProbeDB`. The other two addons test
`[LoadIntoEnvironment secure]` and `## UseSecureEnvironment: 1`.

Expected overlay result:

- `loadInto.envEqualsWriterEnv == false` and
  `useSecure.envEqualsWriterEnv == false`, proving the directive changed the
  file environment away from the normal writer env.
- `loadInto.primitiveSeen` and `useSecure.primitiveSeen` keep the original
  primitive value even after the writer rebinding, because the secure env has
  an own copied slot.
- `loadInto.beforeRead == 6` and `useSecure.beforeRead == 16`, because late
  addon globals were absent from the secure env copy and fall through to `_G`.
- `loadInto.afterSecureWrite == 7`, but
  `loadIntoAfter.insecureSees == 6`, because secure writes create/shadow an own
  secure env slot and do not write through to `_G`.
- `writer.afterAllAddons.useSecureLate == 16` after `useSecure` wrote `17`,
  for the same shadowing reason.
- `math` marker mutations cross both ways, because copied table globals share
  the same table reference.

Shared `_G` + taint-only result:

- `envEqualsWriterEnv == true` if the secure directives did not change the
  file environment.
- Secure files see the writer's primitive sentinel.
- Secure writes to `SecureEnvProbe_LoadIntoLate` / `SecureEnvProbe_UseSecureLate`
  update the value seen by insecure code.

Run `/seprobe` after login. Then `/reload` or logout to flush:

```text
WTF\Account\<ACCOUNT>\SavedVariables\SecureEnvProbeWriter.lua
```
