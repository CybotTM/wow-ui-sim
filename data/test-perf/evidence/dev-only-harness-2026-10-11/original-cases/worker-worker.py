from pathlib import Path
import datetime, hashlib, json, os, random, time
repo=Path('/home/osso/Projects/wow/wow-ui-sim')
root=Path('/home/osso/.local/state/wow-ui-sim/verification/dev-harness-original-run-current')
epoch=root/datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
epoch.mkdir()
def save(path,value):
    path.write_text(json.dumps(value,indent=2)+'\n')
def admit(path):
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
        save(path, observations)
        if admitted:
            return True
        if attempt < 4:
            time.sleep(min(10 * 2 ** attempt, 60) + random.uniform(0, 5))
    return False

if any(name in os.environ for name in ['PREFORK_CONFORMANCE_SUITE','PREFORK_CONFORMANCE_DRIVER','PREFORK_CONFORMANCE_TREE_CHILD','PREFORK_EXACT_FIXTURE_GROUP']):
    raise RuntimeError('Reserved prefork environment already set')
if not admit(epoch/'admission.json'):
    raise RuntimeError('Original case admission blocked')
binary=Path('/home/osso/.local/state/wow-ui-sim/verification/dev-harness-green-current/20261011T051913Z/retail/new-prefork')
selectors=['prefork_full_ui_nested::fixture::preloaded_parent_has_normal_game_startup','test_keybindings_panels_detail::world_map_floor_dropdown_hidden_without_subzone_or_map_group']
results=[]
for index,selector in enumerate(selectors):
    argv=['/usr/bin/timeout','90',str(binary),selector,'--exact','--nocapture','--test-threads=1']
    save(epoch/(str(index)+'-submission.json'),{'argv':argv,'artifact_receipt':'../dev-harness-green-current/20261011T051913Z/retail/artifacts.json','scope':'Already sealed compiled artifact256e54ef1; no conformance shortcut environment, no fixture changes'})
    result=cli.command(*argv).cwd(str(repo)).env('WOW_SIM_CASC','0').capture().run()
    (epoch/(str(index)+'.stdout')).write_text(result.stdout)
    (epoch/(str(index)+'.stderr')).write_text(result.stderr)
    results.append({'argv':argv,'exit':result.exit_code,'named_case_passed':'test '+selector+' ... ok' in result.stdout})
    save(epoch/'results.json',results)
save(epoch/'outcome.json',{'results':results,'scope':'Two exact defaultRetail originalbody controls; notallmarkers/profiles/nativeacceptance'})
print(str(epoch))
