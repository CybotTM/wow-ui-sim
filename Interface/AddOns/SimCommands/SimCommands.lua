SimCommands = {}
SimCommands.commands = {}

function SimCommands:Register(name, description, action, category)
    table.insert(self.commands, {
        name = name,
        description = description or "",
        action = action,
        category = category or "General",
    })
end

function SimCommands:GetCommands()
    return self.commands
end
