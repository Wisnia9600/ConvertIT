Option Explicit

If WScript.Arguments.Count <> 3 Then
  MsgBox "ConvertIT launcher received invalid arguments.", vbCritical, "ConvertIT"
  WScript.Quit 2
End If

Dim executablePath
Dim inputPath
Dim presetId
Dim shell
Dim command
Dim exitCode

executablePath = WScript.Arguments.Item(0)
inputPath = WScript.Arguments.Item(1)
presetId = WScript.Arguments.Item(2)

Set shell = CreateObject("WScript.Shell")

command = Quote(executablePath) _
  & " shell-convert --input " & Quote(inputPath) _
  & " --preset " & Quote(presetId)

exitCode = shell.Run(command, 0, True)
WScript.Quit exitCode

Function Quote(value)
  Quote = Chr(34) & Replace(value, Chr(34), Chr(34) & Chr(34)) & Chr(34)
End Function
