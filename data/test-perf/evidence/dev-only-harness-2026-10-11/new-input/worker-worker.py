from pathlib import Path
import datetime,hashlib,json,os,random,time
repo=Path('/home/osso/Projects/wow/wow-ui-sim')
root=Path('/home/osso/.local/state/wow-ui-sim/verification/dev-harness-new-input-current')
epoch=root/datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
epoch.mkdir()
def save(name,value):
    (epoch/name).write_text(json.dumps(value,indent=2,sort_keys=True)+'\n')
def admit(stage):
    observations = []
    for attempt in range(5):
        memory = {line.split(':')[0]: int(line.split(':')[1].split()[0]) for line in Path('/proc/meminfo').read_text().splitlines()}
        load = float(Path('/proc/loadavg').read_text().split()[0])
        active = []
        for p in Path('/proc').iterdir():
            if p.name.isdigit():
                try:
                    if (p / 'comm').read_text().strip() == 'cargo' and (p / 'cwd').resolve() in [repo, repo.parent / 'full-suite-checkout']:
                        active.append(p.name)
                except OSError:
                    continue
        admitted = memory['MemAvailable'] / 1048576 >= 6 and load < 24 and not active
        observations.append({'attempt': attempt + 1, 'available_gib': memory['MemAvailable'] / 1048576, 'load1': load, 'relevant_cargo': active, 'admitted': admitted})
        save(stage + '-admission.json', observations)
        if admitted:
            return True
        if attempt < 4:
            time.sleep(min(10 * 2 ** attempt, 60) + random.uniform(0, 5))
    return False

probe=repo/'tests/dev_harness_discovery_probe.rs'
selector='dev_harness_discovery_probe::new_marker_271828'
if probe.exists():
    raise RuntimeError('Temporary probe path already exists; no overwrite')
reserved=['PREFORK_CONFORMANCE_SUITE','PREFORK_CONFORMANCE_DRIVER','PREFORK_CONFORMANCE_TREE_CHILD','PREFORK_EXACT_FIXTURE_GROUP']
if any(k in os.environ for k in reserved):
    raise RuntimeError('Reserved prefork environment already set')
inputs=['Cargo.toml','Cargo.lock','build.rs','crates/test-harness/Cargo.toml','crates/test-harness/build.rs','crates/test-harness/src/lib.rs','tests/integration.rs','tests/prefork_full_ui.rs']
def seals():
    return {name:hashlib.sha256((repo/name).read_bytes()).hexdigest() for name in inputs}
before=seals()
fixture=(repo/'tests/prefork_full_ui_nested/fixture.rs').read_text().replace('preloaded_parent_has_normal_game_startup','new_marker_271828')
assert fixture.count('new_marker_271828')==1
fixture_hash=hashlib.sha256(fixture.encode()).hexdigest()
directory_stat=(repo/'tests').stat()
revision=cli.git('rev-parse','HEAD').cwd(str(repo)).capture().run()
if revision.exit_code!=0:
    raise RuntimeError('Cannot record revision')
argv=['/usr/bin/cargo','test','--offline','--locked','--test','integration','--test','prefork_full_ui','--no-run','--message-format=json']
save('submission.json',{'revision':revision.stdout.strip(),'argv':argv,'environment_overrides':{'CARGO_BUILD_JOBS':'12','CARGO_NET_OFFLINE':'true'},'inputs_before':before,'temporary_path':str(probe),'fixture_sha256':fixture_hash,'original_fixture':'tests/prefork_full_ui_nested/fixture.rs','change':'Only marker function name renamed; original fixture untouched','scope':'Controlled newtopmodule andmarker discovery; originalcaseassertions preserved; no production behavior change'})
old_epoch=Path('/home/osso/.local/state/wow-ui-sim/verification/dev-harness-green-current/20261011T051913Z/retail')
def names(text):
    return {line[:-6] for line in text.splitlines() if line.endswith(': test')}
baseline={kind:names((old_epoch/('new-'+kind+'.stdout')).read_text()) for kind in ['integration','prefork']}
def stage(stage):
    if not admit(stage):
        raise RuntimeError(stage+' resource admission blocked')
    built=cli.command(*argv).cwd(str(repo)).env('CARGO_BUILD_JOBS','12').env('CARGO_NET_OFFLINE','true').capture().run()
    (epoch/(stage+'-compile.stdout')).write_text(built.stdout)
    (epoch/(stage+'-compile.stderr')).write_text(built.stderr)
    save(stage+'-compile-result.json',{'exit':built.exit_code})
    if built.exit_code!=0:
        return {'compile_exit':built.exit_code}
    rows=[json.loads(line) for line in built.stdout.splitlines() if line.startswith('{')]
    result={'compile_exit':0,'targets':[{'name':r['target']['name'],'kind':r['target']['kind'],'fresh':r['fresh'],'features':r['features']} for r in rows if r.get('reason')=='compiler-artifact' and r['package_id'].startswith('path+file://'+str(repo)+'#') and r['target']['kind']!=['custom-build']]}
    for kind,target in [('integration','integration'),('prefork','prefork_full_ui')]:
        found=[r for r in rows if r.get('reason')=='compiler-artifact' and r['target']['name']==target and r.get('executable')]
        if len(found)!=1:
            raise RuntimeError('Ambiguous '+kind+' artifact')
        binary=found[0]['executable']
        listed=cli.command('/usr/bin/timeout','90',binary,'--list').cwd(str(repo)).env('WOW_SIM_CASC','0').capture().run()
        (epoch/(stage+'-'+kind+'.stdout')).write_text(listed.stdout)
        (epoch/(stage+'-'+kind+'.stderr')).write_text(listed.stderr)
        actual=names(listed.stdout)
        result[kind]={'executable':binary,'listing_exit':listed.exit_code,'count':len(actual),'added':sorted(actual-baseline[kind]),'removed':sorted(baseline[kind]-actual),'probe_occurrences':sum(line==selector+': test' for line in listed.stdout.splitlines())}
        if kind=='prefork' and stage=='probe' and listed.exit_code==0:
            run_argv=['/usr/bin/timeout','90',binary,selector,'--exact','--nocapture','--test-threads=1']
            executed=cli.command(*run_argv).cwd(str(repo)).env('WOW_SIM_CASC','0').capture().run()
            (epoch/'probe-run.stdout').write_text(executed.stdout)
            (epoch/'probe-run.stderr').write_text(executed.stderr)
            result['execution']={'argv':run_argv,'exit':executed.exit_code,'named_pass':'test '+selector+' ... ok' in executed.stdout}
    save(stage+'-result.json',result)
    return result
if not admit('initial'):
    raise RuntimeError('Initial admission blocked; no fixture created')
probe_result=None
try:
    with probe.open('x') as f:
        f.write(fixture)
    probe_result=stage('probe')
finally:
    if probe.exists():
        if hashlib.sha256(probe.read_bytes()).hexdigest()!=fixture_hash:
            raise RuntimeError('Probe changed externally; preserve file and stop')
        probe.unlink()
    os.utime(repo/'tests',ns=(directory_stat.st_atime_ns,directory_stat.st_mtime_ns))
    save('source-cleanup.json',{'probe_absent':not probe.exists(),'inputs_after':seals(),'inputs_unchanged':before==seals(),'tests_directory_mtime_restored':(repo/'tests').stat().st_mtime_ns==directory_stat.st_mtime_ns})
cleanup_result=stage('cleanup')
save('outcome.json',{'probe':probe_result,'cleanup':cleanup_result,'probe_absent':not probe.exists(),'inputs_unchanged':before==seals(),'scope':'One newtemporary topmodule/marker, then normal rebuilt registriesafterremoval; no permanenttest ornative/allprofileacceptance'})
print(str(epoch))
