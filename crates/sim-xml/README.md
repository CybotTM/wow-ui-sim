# sim-xml

XML schema, parser preprocessing, animation templates and template registries.
The root passes `TemplatePolicy` for Wrath declaration precedence and registers
Mists templates after the shared intrinsic templates. Cached-UI and selected-policy
tests stay at the root boundary; other original unit tests move with this package.

The only feature is `aura-xml-widgets`, forwarded by the root capability of the same
name. Its cfg-gated serde variants and tag mappings preserve the accepted XML.
This creates two schema builds, not seven client-selected builds. Main approved
this exception during phase 2. No whole client-profile bundle enters this crate.
