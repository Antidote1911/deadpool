; Inno Setup script for Deadpool Password Generator
; Compiled by the release CI with: ISCC.exe /DAppVersion=x.y.z installer.iss

#ifndef AppVersion
  #define AppVersion "0.0.0"
#endif

[Setup]
AppName=Deadpool Password Generator
AppVersion={#AppVersion}
AppPublisher=Antidote1911
AppPublisherURL=https://github.com/Antidote1911/deadpool
AppSupportURL=https://github.com/Antidote1911/deadpool/issues
AppUpdatesURL=https://github.com/Antidote1911/deadpool/releases
DefaultDirName={autopf}\Deadpool
DefaultGroupName=Deadpool
AllowNoIcons=yes
OutputDir=.
OutputBaseFilename=deadpool-windows-x86_64-installer
SetupIconFile=icon.ico
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "addtopath";   Description: "Add deadpool-cli to system PATH"; GroupDescription: "Additional tasks:"

[Files]
Source: "deadpool-cli.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "deadpool.exe";     DestDir: "{app}"; Flags: ignoreversion
Source: "icon.ico";         DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Deadpool Password Generator";    Filename: "{app}\deadpool.exe"; IconFilename: "{app}\icon.ico"
Name: "{group}\{cm:UninstallProgram,Deadpool}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Deadpool Password Generator"; Filename: "{app}\deadpool.exe"; IconFilename: "{app}\icon.ico"; Tasks: desktopicon

[Registry]
Root: HKLM; \
  Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
  ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; \
  Check: NeedsAddPath(ExpandConstant('{app}')); Tasks: addtopath

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(
      HKEY_LOCAL_MACHINE,
      'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
      'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;

[Run]
Filename: "{app}\deadpool.exe"; \
  Description: "{cm:LaunchProgram,Deadpool Password Generator}"; \
  Flags: nowait postinstall skipifsilent
