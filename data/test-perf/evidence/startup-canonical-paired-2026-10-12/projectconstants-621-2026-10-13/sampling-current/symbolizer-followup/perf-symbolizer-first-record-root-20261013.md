# Perf symbolizer first-record root — 2026-10-13

## Scope and safety

Read-only offline investigation of source `621649e56d8418d21c4539b26d787ad40c73431f`. No runtime execution, new sampling, build, executable/library/source hash or authentication, tracked writes, install/settings/kernel changes, or delegation. The sealed executable was inspected only for ELF/debug metadata and symbolization. Its recorded size is 408,320,640 bytes.

## Finding

The repeated diagnostic is not explained by a missing `addr2line`, absent debug sections, or a generally incompatible ELF/DWARF format:

- `PATH` resolves `/usr/bin/addr2line` (GNU Binutils 2.47), `/usr/bin/eu-addr2line` (elfutils 0.196), `/usr/bin/perf` 7.2.10-1, and `/usr/bin/readelf`.
- Sealed `wow-sim` is x86-64 ELF64 PIE with `.debug_info`, `.debug_line`, `.debug_abbrev`, `.debug_str`, `.debug_rnglists`, `.debug_loclists`, `.debug_macro`, `.debug_ranges`, and `.symtab`; `file` reports debug info and not stripped.
- Direct GNU `addr2line -f -C -e <sealed-wow-sim> 0x528d260` resolves `rilua::vm::execute::execute_inner::<false>` to `execute.rs:992`. `0x52cb440` resolves `rilua::compiler::codegen::compile_expr` to `codegen.rs:2616`. `eu-addr2line` is installed too.
- Conversely, passing recorded runtime PIE IP `0x55ba20970481` directly returns `??:0`; that address must first have the PIE load bias removed. From the recorded frame `compile_expr+0x41`, whose ELF symbol address is `0x52cb440`, the observed mapping bias is `0x55ba1b6a5000`. Then `0x55ba20970481 - 0x55ba1b6a5000 = 0x52cb481`, which correctly resolves to `compile_expr` at `codegen.rs:2619`. This establishes the proper address normalization and that the exact sealed image can symbolize.

## Root cause status

The artifact stderr alone does **not** establish whether perf's `cmd__addr2line: could not read first record` arises from its addr2line subprocess line protocol, request/response association, or threaded symbolizer behavior. No evidence supports blaming missing tools, absent debug metadata, or wholly unreadable DWARF. The warning is emitted for the executable path despite successful direct symbolization. Therefore classify the specific perf-internal first-record failure as **not conclusively localized**; likely boundary is perf's subprocess/protocol/thread integration, but that remains a hypothesis, not a proven root.

Supporting limited test from the saved `perf-script.stdout`: four distinct unknown wow-sim runtime IPs were each normalized using the bias derived above. Three resolve to ELF symbols (`Table::raw_set_impl`, `LuaApiMut::set_global_val`, `str_lower`); one (`0x63cb35f`) remains `??:0`, consistent with an address lacking corresponding usable symbol/debug mapping rather than proof of a tool-wide failure. This limited set does not characterize all unknown frames. A separate `perf script -F ip,dso,dsoff,sym,srcline --max-stack 1` probe unexpectedly auto-backgrounded; it was immediately terminated by exact child PID, and process listing confirmed its runner/perf child had exited. Its saved detached output was read, not rerun. Output shows loader frames with normalized DSO offsets, some successfully expanded to symbol names; stderr still contains the same diagnostics. No additional perf replay is justified.

## Source 621 evidence retained

- Existing `perf-report.stderr`, `perf-report-verbose.stderr`, `perf-script.stderr`: each 4,422 bytes with repeated first-record warning; preserved unchanged.
- Existing proof says 6,715 samples, zero lost; its report estimates unknown frames/rows, but reported JSON unknown-leaf proportion 92.45% and 5,032 empty chains invalidate the earlier VM-vs-cache interpretation. Do not make phase/dominance claims from those summaries.
- The saved report may contain some correctly symbolized self symbols alongside unreliable call chains. Treat it as incomplete attribution, not valid evidence for relative phase dominance.
- Perf config checks (`perf config -l`, user/system) returned no configured entries; relevant environment variables `PERF_EXEC_PATH`, `PERF_BUILDID_DIR`, and `DEBUGINFOD_URLS` were unset. These facts do not identify the failing mechanism.

## Conclusion

Concrete result: tools and usable DWARF are present; direct addr2line works when given an ELF virtual address, and fails to resolve an unnormalized runtime PIE address as expected. The original perf warnings therefore point to a perf symbolizer invocation/protocol/path rather than simple absence of the tool or debug info, but available evidence cannot distinguish malformed request, response/protocol, or threaded failure. Keep original diagnostics intact; no warning suppression or fix proposed. Do not use the source-621 report for phase/dominance claims.
