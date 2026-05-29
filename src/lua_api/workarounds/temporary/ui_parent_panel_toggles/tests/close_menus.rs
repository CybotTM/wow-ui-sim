use super::*;

#[test]
fn installs_close_menus_default() {
    let env = WowLuaEnv::new().expect("lua env should initialize");
    env.exec(
        r#"
        UIMenus = { "OpenMenuFrame", "ClosedMenuFrame" }
        OpenMenuFrame = {
            shown = true,
            IsShown = function(self)
                return self.shown
            end,
            Hide = function(self)
                self.shown = false
            end,
        }
        ClosedMenuFrame = {
            shown = false,
            IsShown = function(self)
                return self.shown
            end,
            Hide = function(self)
                self.shown = false
            end,
        }
        "#,
    )
    .expect("menu fixture should install");

    patch(&env);

    let (closed_any, open_shown, closed_shown): (bool, bool, bool) = env
        .eval(
            r#"
            local closedAny = CloseMenus()
            return closedAny, OpenMenuFrame:IsShown(), ClosedMenuFrame:IsShown()
            "#,
        )
        .expect("CloseMenus probe should run");

    assert!(closed_any);
    assert!(!open_shown);
    assert!(!closed_shown);
}
