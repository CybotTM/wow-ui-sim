# sim-widget

Profile-neutral anchor, frame value/enumeration, widget identity, SimpleHTML style,
atlas geometry, BlendMode and LayoutRect types. Root re-exports preserve identity
and original paths. No profile or capability features are forwarded.

Frame, EditBox presentation, registry and layout remain in root. The original
EditBox constructors inspect the active client profile. A move would require
threading a new boolean or changing the type/method ownership. The strict
move-only cut instead retains that code and its dependent Frame/registry/layout
ownership in root. EditBox and its caller are byte-identical to phase 2.
The cooldown impl retains its exact gui/retail-12-0-5 cfg in root. Widget identity
and value-type fixtures move with their code; Frame/layout fixtures stay in root.
