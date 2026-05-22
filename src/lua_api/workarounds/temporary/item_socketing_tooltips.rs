//! Temporary item socketing tooltip workaround.
//!
//! Retail's socketing frame expects socket buttons to install gem tooltip
//! handlers during the full Blizzard item-socketing setup path. Install the
//! missing handlers until that setup path is modeled by the simulator.

use crate::lua_api::WowLuaEnv;

const ITEM_SOCKETING_TOOLTIPS_WORKAROUND_LUA: &str = r#"
local frame = ItemSocketingFrame
local container = frame and frame.SocketingContainer
if type(container) ~= "table" then
    return
end

local function install_socket_on_enter(socket, socketIndex)
    if type(socket) ~= "table" or type(socket.SetScript) ~= "function" then
        return
    end
    socket:SetScript("OnEnter", function(self)
        if type(GameTooltip) ~= "table" then
            return
        end
        if type(GameTooltip.SetOwner) == "function" then
            GameTooltip:SetOwner(self, "ANCHOR_RIGHT")
        end
        if type(GameTooltip.SetSocketGem) == "function" then
            GameTooltip:SetSocketGem(socketIndex)
        end
        if type(GameTooltip.NumLines) == "function"
            and GameTooltip:NumLines() == 0
            and type(GameTooltip.AddLine) == "function" then
            GameTooltip:AddLine("Socket Gem " .. tostring(socketIndex))
        end
        if type(GameTooltip.Show) == "function" then
            GameTooltip:Show()
        end
    end)
end

install_socket_on_enter(container.Socket1, 1)
install_socket_on_enter(container.Socket2, 2)
install_socket_on_enter(container.Socket3, 3)
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(ITEM_SOCKETING_TOOLTIPS_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_socket_tooltip_handlers() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            local function socket(name)
                return {
                    name = name,
                    scripts = {},
                    SetScript = function(self, event, script)
                        self.scripts[event] = script
                    end,
                }
            end

            ItemSocketingFrame = {
                SocketingContainer = {
                    Socket1 = socket("socket1"),
                    Socket2 = socket("socket2"),
                    Socket3 = socket("socket3"),
                },
            }
            GameTooltip = {
                lines = {},
                SetOwner = function(self, owner, anchor)
                    self.owner = owner
                    self.anchor = anchor
                end,
                SetSocketGem = function(self, socketIndex)
                    self.socketIndex = socketIndex
                end,
                NumLines = function(self)
                    return #self.lines
                end,
                AddLine = function(self, line)
                    table.insert(self.lines, line)
                end,
                Show = function(self)
                    self.shown = true
                end,
            }
            "#,
        )
        .expect("socketing test surface should install");

        patch(&env);

        let (handlers_installed, owner_is_socket, anchor, socket_index, first_line, shown): (
            bool,
            bool,
            String,
            i64,
            String,
            bool,
        ) = env
            .eval(
                r#"
                local socket = ItemSocketingFrame.SocketingContainer.Socket2
                socket.scripts.OnEnter(socket)

                return ItemSocketingFrame.SocketingContainer.Socket1.scripts.OnEnter ~= nil
                        and ItemSocketingFrame.SocketingContainer.Socket2.scripts.OnEnter ~= nil
                        and ItemSocketingFrame.SocketingContainer.Socket3.scripts.OnEnter ~= nil,
                    GameTooltip.owner == socket,
                    GameTooltip.anchor,
                    GameTooltip.socketIndex,
                    GameTooltip.lines[1],
                    GameTooltip.shown
                "#,
            )
            .expect("patched socket tooltip state should be readable");

        assert!(handlers_installed);
        assert!(owner_is_socket);
        assert_eq!(anchor, "ANCHOR_RIGHT");
        assert_eq!(socket_index, 2);
        assert_eq!(first_line, "Socket Gem 2");
        assert!(shown);
    }

    #[test]
    fn tolerates_missing_socketing_container() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        patch(&env);

        let marker: i64 = env
            .eval("return 1")
            .expect("patch without item socketing frame should not error");

        assert_eq!(marker, 1);
    }
}
