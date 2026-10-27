#ifndef AppVersion
#define AppVersion "dev"
#endif

#ifndef SourceDir
#define SourceDir "..\dist\wow-ui-sim"
#endif

#ifndef OutputDir
#define OutputDir "..\dist"
#endif

#ifndef OutputBaseFilename
#define OutputBaseFilename "wow-ui-sim-setup"
#endif

#ifdef Forever
#define ProductName "WoW UI Simulator Forever"
#define ProductId "{{6D8C21F0-652C-4AE0-A46E-0A2C90A729A6}"
#else
#define ProductName "WoW UI Simulator"
#define ProductId "{{8B6B89DF-7E56-4A69-A2E1-9B439A1D4BA1}"
#endif

[Setup]
AppId={#ProductId}
AppName={#ProductName}
AppVersion={#AppVersion}
AppPublisher=Osso
AppPublisherURL=https://github.com/Osso/wow-ui-sim
AppSupportURL=https://github.com/Osso/wow-ui-sim/issues
AppUpdatesURL=https://github.com/Osso/wow-ui-sim/releases
DefaultDirName={localappdata}\Programs\{#ProductName}
DisableProgramGroupPage=yes
OutputDir={#OutputDir}
OutputBaseFilename={#OutputBaseFilename}
Compression=lzma2
SolidCompression=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayIcon={app}\wow-sim.exe
SetupIconFile=wow-sim.ico
WizardStyle=modern

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "wow-sim.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#ProductName}"; Filename: "{app}\wow-sim.exe"; WorkingDir: "{app}"; IconFilename: "{app}\wow-sim.ico"
