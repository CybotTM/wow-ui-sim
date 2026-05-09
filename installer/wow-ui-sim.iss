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

[Setup]
AppId={{8B6B89DF-7E56-4A69-A2E1-9B439A1D4BA1}
AppName=WoW UI Simulator
AppVersion={#AppVersion}
AppPublisher=Osso
AppPublisherURL=https://github.com/Osso/wow-ui-sim
AppSupportURL=https://github.com/Osso/wow-ui-sim/issues
AppUpdatesURL=https://github.com/Osso/wow-ui-sim/releases
DefaultDirName={localappdata}\Programs\WoW UI Simulator
DisableProgramGroupPage=yes
OutputDir={#OutputDir}
OutputBaseFilename={#OutputBaseFilename}
Compression=lzma2
SolidCompression=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayIcon={app}\wow-sim.exe
WizardStyle=modern

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\WoW UI Simulator"; Filename: "{app}\wow-sim.exe"; WorkingDir: "{app}"
