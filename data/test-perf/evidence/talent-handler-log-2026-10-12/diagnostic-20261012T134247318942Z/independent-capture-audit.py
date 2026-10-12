from pathlib import Path
from collections import Counter
from decimal import Decimal
import hashlib, json, re

E = Path('/home/osso/.local/state/wow-ui-sim/verification/talent-handler-log-current/diagnostic-20261012T134247318942Z')
R = Path('/home/osso/Projects/wow/wow-ui-sim')
raw = (E / 'runtime.stderr').read_text()
parsed = json.loads((E / 'parse.stdout').read_text())
counts = {'baseline':20, 'settle':10, 'steady':24}
expected = [(p, i, edge) for p,n in counts.items() for i in range(n) for edge in ('begin','end')]
markers, records, outside, ticks, unparsed = [], [], [], [], []
active = None
for number,line in enumerate(raw.splitlines(),1):
    if line.lstrip().startswith('[talent-tick]'):
        m = re.fullmatch(r'\[talent-tick\] phase=(\w+) tick=(\d+) (begin|end)',line)
        assert m, (number,line)
        marker = (m[1],int(m[2]),m[3]); markers.append(marker)
        if marker[2] == 'begin':
            assert active is None
            active = {'phase':marker[0], 'tick':marker[1], 'records':[], 'other_line_count':0}
            ticks.append(active)
        else:
            assert active and (active['phase'],active['tick']) == marker[:2]
            active = None
    elif line.lstrip().startswith('[handler]'):
        assert line.startswith('[handler] addon=')
        addon, tail = line[len('[handler] addon='):].split(' handler=',1)
        handler, tail = tail.split(' frame=',1)
        frame, tail = tail.split(' duration_ms=',1)
        duration, sep, source = tail.partition(' source=')
        assert re.fullmatch(r'[0-9]+\.[0-9]{3}',duration), (number,line)
        assert re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*',handler)
        record = (addon,handler,frame,source if sep else None,int(Decimal(duration)*1000))
        records.append(record)
        (active['records'] if active else outside).append(record)
    else:
        unparsed.append({'line':number,'text':line,'window':None if active is None else [active['phase'],active['tick']]})
        if active: active['other_line_count'] += 1
assert active is None and markers == expected

def stats(rows):
    return {'count':len(rows),'interval_total_ms':sum(x[4] for x in rows)/1000,'interval_max_ms':max((x[4] for x in rows),default=0)/1000}

def summary(rows):
    return {**stats(rows),'by_handler_kind':{k:stats([x for x in rows if (x[1] if x[1] in ('OnUpdate','OnPostUpdate') else 'other')==k]) for k in ('OnUpdate','OnPostUpdate','other')}}

def grouped(rows):
    accum = {}
    for row in rows: accum.setdefault(row[:4],[]).append(row)
    output = [{**dict(zip(('addon','handler','frame','source'),key)),**stats(values)} for key,values in accum.items()]
    return sorted(output,key=lambda x:(-x['interval_total_ms'],x['addon'],x['handler'],x['frame'],x['source'] or ''))

independent_ticks = [{k:v for k,v in t.items() if k!='records'} | summary(t['records']) for t in ticks]
phases = {}
for phase in counts:
    windows = [t for t in ticks if t['phase']==phase]
    rows = [x for t in windows for x in t['records']]
    phases[phase] = {'tick_count':len(windows),**summary(rows),'other_line_count':sum(t['other_line_count'] for t in windows),'grouped_handlers':grouped(rows)}
outside_summary = {**summary(outside),'other_line_count':sum(x['window'] is None for x in unparsed),'grouped_handlers':grouped(outside)}
checks = {'expected_counts':parsed['expected_counts']==counts,'ticks':parsed['ticks']==independent_ticks,'phases':parsed['phases']==phases,'all_logged_dispatches':parsed['all_logged_dispatches']==summary(records),'grouped_handlers':parsed['grouped_handlers']==grouped(records),'outside_markers':parsed['outside_markers']==outside_summary,'other_line_count':parsed['other_line_count']==len(unparsed)}
assert all(checks.values()), checks
identity_counts = Counter(x[:4] for x in records)
old = json.loads((E.parent/'proof.json').read_text())
source_hashes = {p:hashlib.sha256((R/p).read_bytes()).hexdigest() for p in old['source_sha256']}
source_matches = {p:source_hashes[p]==h for p,h in old['source_sha256'].items()}
assert all(source_matches.values())
source_snapshots = [json.loads((E/n).read_text()) for n in ('source-before.json','source-after-compile.json','source-after-runtime.json')]
assert source_snapshots[0]==source_snapshots[1]==source_snapshots[2]
assert all(hashlib.sha256((R/p).read_bytes()).hexdigest()==h for p,h in source_snapshots[0].items())
meta = {p.name:json.loads(p.read_text()) for p in E.glob('*.json') if p.name not in ('capture-independent-proof.json',)}
compile_rows = [json.loads(x) for x in (E/'compile.stdout').read_text().splitlines() if x.strip()]
selected = [x for x in compile_rows if x.get('reason')=='compiler-artifact' and x.get('target',{}).get('name')=='integration' and x.get('executable')]
assert selected==[meta['artifact.json']['cargo_record']]
assert {'reason':'build-finished','success':True} in compile_rows
proof = {
    'scope':'New epoch bounded real diagnostic capture; source-only proof reused only for unchanged source/parser development',
    'status':'PASS_BOUNDED_CAPTURE_COHERENCE',
    'epoch':str(E), 'repo':str(R), 'source_revision':'1d2b0ea30afa2c76128bc1356b2125615c6e1748',
    'authentication_receipt':meta['independent-auth-integration.json'],
    'source_only_proof':str(E.parent/'proof.json'),'source_hashes':source_hashes,'source_only_reuse_matches':source_matches,
    'source_snapshots_equal_current':True,
    'compile':{'submission':meta['compile-submission.json'],'result':meta['compile-result.json'],'cargo_record_matches':True,'integration_fresh':selected[0]['fresh'],'library_fresh':next(x['fresh'] for x in compile_rows if x.get('target',{}).get('name')=='wow_ui_sim'),'build_finished_success':True,'compiler_message_count':sum(x.get('reason')=='compiler-message' for x in compile_rows),'manifest_warning_count':6},
    'toolchain':{'cargo':(E/'cargo-version.stdout').read_text(),'rustc':(E/'rustc-version.stdout').read_text()},
    'runtime_original':{'status':'PASS','result':meta['runtime-result.json'],'submission':meta['runtime-submission.json'],'credited_body_executions':1,'stdout':(E/'runtime.stdout').read_text(),'tick_latency_line':next(x['text'] for x in unparsed if x['text'].startswith('talent panel OnUpdate')),'performance_fix':False},
    'parser':{'saved_result':meta['parse-result.json'],'independent_comparisons':checks,'ordered_windows':54,'markers':108,'phase_counts':counts,'all_logged_dispatches':summary(records),'phases':phases,'outside_markers':outside_summary,'ticks':independent_ticks,'grouped_handlers':grouped(records),'handler_name_counts':dict(Counter(x[1] for x in records)),'unique_identity_count':len(identity_counts),'repeated_identity_count':sum(v>1 for v in identity_counts.values()),'repeat_records_beyond_first':sum(v-1 for v in identity_counts.values()),'zero_interval_records':sum(x[4]==0 for x in records),'other_line_count':len(unparsed),'unparsed_lines':unparsed},
    'input_context':meta['input-context.json'],'cache_context':meta['cache-context.json'],
    'formatting':{'status':'PASS' if meta.get('fmt-result.json',{}).get('exit')==0 else 'PENDING_OR_FAIL','result':meta.get('fmt-result.json'),'submission':meta.get('fmt-submission.json'),'stdout':(E/'fmt.stdout').read_text() if (E/'fmt.stdout').exists() else None,'stderr':(E/'fmt.stderr').read_text() if (E/'fmt.stderr').exists() else None},
    'default_check_reuse':{'epoch':'/home/osso/.local/state/wow-ui-sim/verification/friend-info-exact-current/check-20261012T124825577210Z','condition':'Only unchanged src/Cargo/library; new integration test separately compiled','git_diff_from_6ec0c1cb8_src_Cargo_crates_build_rs':'','observed_exit':0,'old_binary_seals_rehashed':False},
    'limits':['No frozen BA or native or full-suite acceptance','Logging perturbs runtime; observed PASS is not performance repair or historical FAIL reversal','Process environment is reconstructed from saved launcher and unmodified child source, not independently recorded child /proc environment','Handshake file is ephemeral; successful parent protocol and single 54-window stream support one body execution, not two PASS credits','No archive/decode attestation or full cache content hash; current metadata only','Prototype source:line_defined is declaration label, not unique invocation or hot inner line','Rounded inclusive dispatch intervals include helper/error/nested work; attempted calls need not execute Lua body; missing/filtered/unsupported dispatch paths cannot be inferred from absence'],
    'commands_not_run':['Cargo','tests','simulator runtime','delegation'],'repo_edits':False
}
for name in ('runtime.stderr','runtime.stdout','parse.stdout','compile.stdout','compile.stderr','artifact.json'):
    proof.setdefault('capture_file_sha256',{})[name]=hashlib.sha256((E/name).read_bytes()).hexdigest()
(E/'capture-independent-proof.json').write_text(json.dumps(proof,indent=2)+'\n')
print(json.dumps({'checks':checks,'counts':proof['parser']['handler_name_counts'],'unique':len(identity_counts),'repeated':proof['parser']['repeated_identity_count'],'repeat_extra':proof['parser']['repeat_records_beyond_first'],'zeros':proof['parser']['zero_interval_records'],'top_groups':grouped(records)[:5]},indent=2))
