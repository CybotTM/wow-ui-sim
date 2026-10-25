local function check(tab, view, deposit, edit, stacks)
    local function results(...)
        assert(select('#', ...) == 4)
        local v, d, e, s = ...
        assert(v == view and d == deposit and e == edit and s == stacks)
        assert(type(s) == 'number')
    end
    results(GetGuildBankTabPermissions(tab, 'ignored'))
end
check(1, nil, nil, nil, 0)
check(2, 1, nil, nil, 11)
check(3, nil, 1, nil, 22)
check(4, nil, nil, 1, 33)
check(5, 1, 1, 1, 44)
check(6, 1, 1, 1, -1)
check('2', 1, nil, nil, 11)
check(2.9, 1, nil, nil, 11)
check(258, 1, nil, nil, 11)
for _, tab in ipairs({0, -1, 7, 256}) do
    check(tab, nil, nil, nil, 0)
end
assert(select('#', GetGuildBankWithdrawLimit('ignored', {})) == 1)
assert(GetGuildBankWithdrawLimit() == 12)
