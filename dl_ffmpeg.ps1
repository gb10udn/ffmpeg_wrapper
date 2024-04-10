Function DownloadFfmpegExe() {
    $ffmpegUrl = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip"
    $dst = ".\ffmpeg_.exe"
    
    $tempPath00 = ".\temp"
    $tempPath01 = ".\temp\ffmpeg.zip"
    $tempPath02 = ".\temp\ffmpeg"
    $tempPath03 = ".\temp\ffmpeg\ffmpeg-master-latest-win64-gpl\bin\ffmpeg.exe"

    if ( -not (Test-Path $dst) ) {
        New-Item -ItemType Directory -Path $tempPath00
        Start-BitsTransfer -Source $ffmpegUrl -destination $tempPath01  # FIXME: 240410 Uri が存在しない場合の処理を記述する。
        Expand-Archive -Path $tempPath01 -DestinationPath $tempPath02
        Copy-Item $tempPath03 $dst
        Remove-Item $tempPath00 -Recurse
    }
}

# main process
DownloadFfmpegExe