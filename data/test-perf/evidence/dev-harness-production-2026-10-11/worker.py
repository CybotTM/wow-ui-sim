from pathlib import Path
import datetime,hashlib,json,time
repo=Path('/home/osso/Projects/wow/wow-ui-sim')
root=Path('/home/osso/.local/state/wow-ui-sim/verification/dev-harness-production-build-current')
epoch=root/datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
epoch.mkdir()
memory={line.split(':')[0]:int(line.split(':')[1].split()[0]) for line in Path('/proc/meminfo').read_text().splitlines()}
load=float(Path('/proc/loadavg').read_text().split()[0])
active=[]
for p in Path('/proc').iterdir():
    if p.name.isdigit():
        try:
            if (p/'comm').read_text().strip()=='cargo' and (p/'cwd').resolve() in [repo,repo.parent/'full-suite-checkout']:
                active.append(p.name)
        except OSError:
            continue
admission={'available_gib':memory['MemAvailable']/1048576,'load1':load,'relevant_cargo':active}
(epoch/'admission.json').write_text(json.dumps(admission,indent=2)+'\n')
if admission['available_gib']<6 or load>=24 or active:
    raise RuntimeError('Production build admission blocked; no build started')
inputs=['Cargo.toml','Cargo.lock','build.rs','crates/test-harness/Cargo.toml','crates/test-harness/build.rs','crates/test-harness/src/lib.rs','.cargo/config.toml']
before={name:hashlib.sha256((repo/name).read_bytes()).hexdigest() for name in inputs}
revision=cli.git('rev-parse','HEAD').cwd(str(repo)).capture().run()
argv=['/usr/bin/cargo','build','--offline','--locked','--bin','wow-sim','--message-format=json']
(epoch/'submission.json').write_text(json.dumps({'argv':argv,'revision':revision.stdout.strip(),'input_hashes_before':before,'environment_overrides':{'CARGO_BUILD_JOBS':'12','CARGO_NET_OFFLINE':'true'},'scope':'Ordinarydefaultproductionbinary selection, nottest/workspacebuild; no timingacceptance/nativehost/container/deployment'},indent=2)+'\n')
result=cli.command(*argv).cwd(str(repo)).env('CARGO_BUILD_JOBS','12').env('CARGO_NET_OFFLINE','true').capture().run()
(epoch/'build.stdout').write_text(result.stdout)
(epoch/'build.stderr').write_text(result.stderr)
rows=[json.loads(line) for line in result.stdout.splitlines() if line.startswith('{')]
helper=[row for row in rows if 'test-harness' in row.get('package_id','')]
artifacts=[row for row in rows if row.get('reason')=='compiler-artifact']
after={name:hashlib.sha256((repo/name).read_bytes()).hexdigest() for name in inputs}
(epoch/'outcome.json').write_text(json.dumps({'exit':result.exit_code,'helper_records':helper,'artifact_records':len(artifacts),'fresh':sum(r['fresh'] for r in artifacts),'nonfresh':sum(not r['fresh'] for r in artifacts),'input_hashes_after':after,'inputs_unchanged':before==after,'scope':'Ordinaryrootproductionbuild only; devcrate metadata maystillberequired; notalltargets/profiles/nativeacceptance'},indent=2)+'\n')
print(str(epoch))
