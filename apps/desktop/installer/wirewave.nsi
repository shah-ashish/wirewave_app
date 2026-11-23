; WireWave Stage 1 Windows Installer (NSIS 3.x)
; Enforces Windows 11 (Build >= 22000) and preserves %LOCALAPPDATA%\WireWave on uninstall.

!include "MUI2.nsh"
!include "LogicLib.nsh"

!ifndef APP_NAME
  !define APP_NAME "WireWave"
!endif
!ifndef APP_VERSION
  !define APP_VERSION "1.1.4"
!endif
!ifndef BIN_PATH
  !define BIN_PATH "..\..\..\target\release\wirewave-desktop.exe"
!endif
!ifndef ICON_PATH
  !define ICON_PATH "..\..\..\assets\brand\wirewave.ico"
!endif
!ifndef OUT_FILE
  !define OUT_FILE "..\..\..\target\release\WireWave-Setup-${APP_VERSION}.exe"
!endif

Name "${APP_NAME} ${APP_VERSION}"
OutFile "${OUT_FILE}"
InstallDir "$LOCALAPPDATA\Programs\${APP_NAME}"
RequestExecutionLevel user
SetCompressor /SOLID lzma

!define MUI_ICON "${ICON_PATH}"
!define MUI_UNICON "${ICON_PATH}"
!define MUI_FINISHPAGE_RUN "$INSTDIR\wirewave-desktop.exe"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Function .onInit
  ; Verify Windows 11 (Build >= 22000) via registry CurrentBuildNumber
  ReadRegStr $0 HKLM "SOFTWARE\Microsoft\Windows NT\CurrentVersion" "CurrentBuildNumber"
  ${If} $0 < 22000
    MessageBox MB_OK|MB_ICONSTOP "${APP_NAME} requires Windows 11 (Build 22000 or newer). Detected build: $0."
    Abort
  ${EndIf}

  ; Gracefully terminate any running instance of WireWave before extracting files.
  ; This releases the OS file lock on wirewave-desktop.exe so the File instruction
  ; can overwrite it without an "Error opening file for writing" dialog.
  nsExec::ExecToLog 'taskkill /F /IM wirewave-desktop.exe'
  ; Allow a brief moment for the process to fully exit before extraction begins.
  Sleep 500
FunctionEnd

Section "Install"
  SetOutPath "$INSTDIR"
  File "${BIN_PATH}"
  CreateDirectory "$SMPROGRAMS\${APP_NAME}"
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\wirewave-desktop.exe"
  CreateShortcut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\wirewave-desktop.exe"
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayName" "${APP_NAME}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayVersion" "${APP_VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayIcon" "$INSTDIR\wirewave-desktop.exe"
SectionEnd

Section "Uninstall"
  ; Intentionally preserve %LOCALAPPDATA%\WireWave\identity.json so Device ID survives reinstall.
  Delete "$INSTDIR\wirewave-desktop.exe"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
  RMDir "$SMPROGRAMS\${APP_NAME}"
  Delete "$DESKTOP\${APP_NAME}.lnk"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"
SectionEnd
