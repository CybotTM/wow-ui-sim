# Screenshot encoding and inspection fidelity

Verified: 2026-10-15. External format/API guidance, not new simulator behavior or runtime acceptance. The [screenshot contract](../../specs/screenshot-output-format.md) owns encoding flags, actual defaults and scoped test receipts.

## Encoding

- W3C defines PNG as a lossless format supporting truecolor and alpha. Use the existing opt-in PNG path when comparing decoded pixel bytes. Lossless storage does not establish correct rendering, color management or native WoW parity.
- WebP supports separate lossy and lossless encoders. Google's lossy quality parameter controls the compression/quality tradeoff; quality 100 is not the lossless encoder. Retain the simulator's existing WebP defaults rather than silently changing their quality or format.
- A future lossless-WebP option would need its own byte-preservation tests: Google's simple lossless encoder disables `exact` by default and can change RGB beneath fully transparent pixels. The bitstream can preserve those values, but the encoder setting matters. No lossless-WebP feature is proposed here.

## Resolution and model inspection

Keep an original-resolution capture as evidence. For a small UI defect, provide a separate native-pixel crop plus the full frame for context; this is an inspection recommendation, not a change to capture defaults or layout.

OpenAI's current vision guidance recommends `detail: original` for supported models when fine details or coordinates matter. Image preprocessing still depends on model-specific dimension/patch limits; changing PNG versus WebP alone does not bypass resizing. Small text can require a larger text view. No single universal screenshot resolution follows from this guidance, and this research does not prove that Pi's attachment/viewer path delivers original pixels to a model.

Do not upscale and treat the result as new detail, or infer absent hidden text from failed OCR. Decoded image pixels, model interpretation and arbitrary steganographic absence are separate claims.

## Sources

- [W3C PNG specification, scope and image types](https://www.w3.org/TR/png-3/) — lossless storage, RGB/alpha and color interpretation.
- [Google WebP encoding API](https://developers.google.com/speed/webp/docs/api#simple_encoding_api) — separate encoders, quality semantics and transparent-RGB `exact` caveat.
- [Google lossless WebP bitstream](https://developers.google.com/speed/webp/docs/webp_lossless_bitstream_specification) — exact ARGB representation supported by the format.
- [OpenAI images and vision](https://developers.openai.com/api/docs/guides/images-vision) — model/detail-specific resizing, fine-detail guidance and small-text limits; checked on the verification date, not a permanent model-limit contract.

## See Also

- [Screenshot CLI](cli-commands.md#screenshot)
- [Rendering pipeline](../../rendering-pipeline.md)
- [Screenshot output contract](../../specs/screenshot-output-format.md)
