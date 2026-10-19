use super::{AddonContext, LoadTiming, WowLuaEnv, load_xml_file};
use std::path::Path;
use std::rc::Rc;

const FIXTURE: &str = r#"<Ui>
    <Script>
        function FactoryXMLRecord(self, label, event, ...)
            assert(event == 'PLAYER_ENTERING_WORLD')
            assert(select('#', ...) == 2)
            local payload, missing = ...
            assert(payload == self.expected and missing == nil)
            self.private.calls = self.private.calls + 1
            self.order[#self.order + 1] = label
            return label, nil, self.private.calls
        end
        FactoryXMLBaseMixin = {
            BaseLoad = function(self)
                self.order = self.order or {}
                self.order[#self.order + 1] = 'base-load'
                self.private = { calls = 0 }
            end,
            BaseEvent = function(self, ...)
                return FactoryXMLRecord(self, 'base-event', ...)
            end,
        }
        FactoryXMLDerivedMixin = {
            DerivedLoad = function(self)
                self.order[#self.order + 1] = 'derived-load'
            end,
            DerivedEvent = function(self, ...)
                return FactoryXMLRecord(self, 'derived-event', ...)
            end,
        }
        FactoryXMLInstanceMixin = {
            InstanceEvent = function(self, ...)
                return FactoryXMLRecord(self, 'instance-event', ...)
            end,
        }
    </Script>
    <Frame name="FactoryXMLBase" virtual="true" mixin="FactoryXMLBaseMixin">
        <Scripts>
            <OnLoad method="BaseLoad"/>
            <OnEvent method="BaseEvent"/>
        </Scripts>
    </Frame>
    <Frame name="FactoryXMLDerived" virtual="true" inherits="FactoryXMLBase" mixin="FactoryXMLDerivedMixin">
        <Scripts>
            <OnLoad method="DerivedLoad" inherit="prepend"/>
            <OnEvent method="DerivedEvent" inherit="append"/>
        </Scripts>
    </Frame>
    <Frame name="FactoryXMLInstance" virtual="true" inherits="FactoryXMLDerived" mixin="FactoryXMLInstanceMixin">
        <Scripts>
            <OnLoad inherit="append">
                self.order = { 'instance-load' }
            </OnLoad>
            <OnEvent method="InstanceEvent" inherit="prepend"/>
        </Scripts>
    </Frame>
    <Frame name="FactoryXMLFirst" parent="UIParent" inherits="FactoryXMLInstance"/>
    <Frame name="FactoryXMLSecond" parent="UIParent" inherits="FactoryXMLInstance"/>
    <Frame name="FactoryXMLBare" parent="UIParent" inherits="FactoryXMLBase"/>
</Ui>"#;

fn load_fixture(env: &WowLuaEnv, root: &Path) -> crate::Result<()> {
    let addon_table = env.create_addon_table()?;
    let context = AddonContext::new(
        env.lua(),
        "FactoryXMLProbe",
        addon_table,
        root,
        false,
        false,
    )?;
    load_xml_file(
        &env.loader_env(),
        &root.join("factory.xml"),
        &context,
        &mut LoadTiming::default(),
    )
    .map(|_| ())
    .map_err(|error| crate::Error::Other(error.to_string()))
}

fn exercise_method_handlers(env: &WowLuaEnv) {
    env.exec(
        r#"
        local first, second, bare = FactoryXMLFirst, FactoryXMLSecond, FactoryXMLBare
        assert(first.private ~= second.private)
        assert(first:GetScript('OnLoad') ~= second:GetScript('OnLoad'))
        assert(first:GetScript('OnEvent') ~= second:GetScript('OnEvent'))
        local payload = {}
        for _, frame in ipairs({first, second}) do
            assert(frame.private.calls == 0)
            assert(table.concat(frame.order, ',') == 'instance-load,base-load,derived-load')
            frame.expected = payload
            assert(select('#', frame:GetScript('OnEvent')(frame, 'PLAYER_ENTERING_WORLD', payload, nil)) == 0)
            assert(frame.private.calls == 3)
            assert(table.concat(frame.order, ',') ==
                'instance-load,base-load,derived-load,derived-event,base-event,instance-event')
        end
        first.BaseEvent = function(self, ...)
            return FactoryXMLRecord(self, 'first-only', ...)
        end
        first:GetScript('OnEvent')(first, 'PLAYER_ENTERING_WORLD', payload, nil)
        assert(first.private.calls == 6 and second.private.calls == 3)
        assert(first.order[8] == 'first-only')
        assert(second.order[5] == 'base-event')
        bare.expected = payload
        local label, missing, count = bare:GetScript('OnEvent')(bare, 'PLAYER_ENTERING_WORLD', payload, nil)
        assert(label == 'base-event' and missing == nil and count == 1)
        "#,
    )
    .unwrap();
    assert!(env.state().borrow().lua_errors.is_empty());
}

#[test]
fn reload_factory_xml_methods_keep_inheritance_private_state_and_reload_freshness() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("factory.xml"), FIXTURE).unwrap();
    let mut env = WowLuaEnv::new().unwrap();
    let root = directory.path().to_path_buf();
    env.set_reload_loader(move |env, _| load_fixture(env, &root));
    load_fixture(&env, directory.path()).unwrap();
    exercise_method_handlers(&env);
    let old_lua = Rc::downgrade(&env.lua);
    env.exec("FactoryXMLOldSentinel = {}; ReloadUI()").unwrap();
    assert!(env.service_reload(&mut None).unwrap());
    assert!(old_lua.upgrade().is_none());
    assert!(
        env.eval::<bool>("return FactoryXMLOldSentinel == nil")
            .unwrap()
    );
    exercise_method_handlers(&env);
}
