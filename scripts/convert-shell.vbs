Option Explicit

If WScript.Arguments.Count <> 4 Then
  MsgBox "ConvertIT launcher received invalid arguments.", vbCritical, "ConvertIT"
  WScript.Quit 2
End If

Dim helperScript
Dim executablePath
Dim inputPath
Dim presetId
Dim shell
Dim command
Dim exitCode

helperScript = WScript.Arguments.Item(0)
executablePath = WScript.Arguments.Item(1)
inputPath = WScript.Arguments.Item(2)
presetId = WScript.Arguments.Item(3)

Set shell = CreateObject("WScript.Shell")

command = "powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -WindowStyle Hidden -File " _
  & Quote(helperScript) _
  & " -ExecutablePath " & Quote(executablePath) _
  & " -InputPath " & Quote(inputPath) _
  & " -PresetId " & Quote(presetId)

exitCode = shell.Run(command, 0, True)
WScript.Quit exitCode

Function Quote(value)
  Quote = Chr(34) & Replace(value, Chr(34), Chr(34) & Chr(34)) & Chr(34)
End Function
