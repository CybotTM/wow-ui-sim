# sim-render

Profile-neutral markup stripping, font measurement arithmetic, quad batching and
canvas texture drawing. Only `gui` is forwarded for optional iced/wgpu/bytemuck.
Shared logging remains in sim-texture.

GPU atlas, pipeline, primitive/program, upload values and shader tests remain in
root. Their shared BC support atomic has an exactly preserved `#[cfg(test)]`
setter and re-export. Moving that coupled code would make the setter unavailable
to existing root tests or require changing conditional availability. This cut
does neither. Root re-exports the shared quad types at the original paths.

Font construction and shaping remain in root because they select CASC/profile
data. No new loader interface or runtime behavior is introduced.
