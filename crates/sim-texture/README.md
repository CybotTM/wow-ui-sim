# sim-texture

Profile-neutral texture values, BLP/RGBA decoding, dimensions, crop/cache helpers,
directory observations and shared process logging. No capability features.

TextureManager stays in root: its inherent resolve/preload methods depend on
profile-selected CASC configuration, atlas data and spellbook state. Moving the
manager would require a resolver/preload interface or wrapper. This cut does not
introduce one. Renderer-crossing and profile/preload tests stay in composition,
above both leaves. Existing root paths re-export all public texture values.
