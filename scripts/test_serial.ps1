# Test reading from Dabao COM port
$portName = if ($args[0]) { $args[0] } else { "COM10" }
$baudRate = if ($args[1]) { [int]$args[1] } else { 115200 }

Write-Host "Opening $portName at $baudRate baud..."
$port = New-Object System.IO.Ports.SerialPort $portName, $baudRate
$port.ReadTimeout = 1000
$port.WriteTimeout = 1000

try {
    $port.Open()
    Write-Host "[SUCCESS] $portName opened. Handshaking..."
    # Send a newline to trigger prompt if Dabao console is waiting
    $port.WriteLine("")
    Start-Sleep -Milliseconds 300
    
    $bytes = $port.BytesToRead
    Write-Host "Bytes available to read: $bytes"
    if ($bytes -gt 0) {
        $buffer = New-Object byte[] $bytes
        $bytesRead = $port.Read($buffer, 0, $bytes)
        $text = [System.Text.Encoding]::UTF8.GetString($buffer, 0, $bytesRead)
        Write-Host "Received telemetry:"
        Write-Host "--------------------------------------------------"
        Write-Host $text
        Write-Host "--------------------------------------------------"
    } else {
        Write-Host "No immediate characters received (board may be idle or in sleep state)."
    }
} catch {
    Write-Host "[ERROR] Serial operation failed: $_"
} finally {
    if ($port.IsOpen) {
        $port.Close()
        Write-Host "Port closed."
    }
}
