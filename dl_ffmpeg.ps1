Function DownloadFfmpegExe {
    param (
        [string]$ffmpegUrl = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip",
        [string]$dst = ".\ffmpeg_.exe"
    )
    $TEMP_PATH_00 = ".\temp_ffmpeg"
    $TEMP_PATH_01 = ".\temp_ffmpeg\ffmpeg.zip"
    $TEMP_PATH_02 = ".\temp_ffmpeg\ffmpeg"
    $TEMP_PATH_03 = ".\temp_ffmpeg\ffmpeg\ffmpeg-master-latest-win64-gpl\bin\ffmpeg.exe"

    if ( -not (Test-Path $dst) ) {
        New-Item -ItemType Directory -Path $TEMP_PATH_00
        Start-BitsTransfer -Source $ffmpegUrl -destination $TEMP_PATH_01  # FIXME: 240410 Uri が存在しない場合の処理を記述する。
        Expand-Archive -Path $TEMP_PATH_01 -DestinationPath $TEMP_PATH_02
        Copy-Item $TEMP_PATH_03 $dst
        Remove-Item $TEMP_PATH_00 -Recurse
    }
}

# main process
DownloadFfmpegExe