-- Explicit input, not native default state. Pinned LossOfControlData numeric icon.
A_Admin.AddLossOfControl('player', {
    locType = 'STUN', spellID = 408, displayText = 'Kidney Shot',
    iconTexture = 132298, startTime = 0, duration = 4, timeRemaining = 4,
    lockoutSchool = 0, priority = 1, displayType = 2,
})
