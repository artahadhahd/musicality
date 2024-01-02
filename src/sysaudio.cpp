extern "C" bool
platform_supported ()
{
#if defined(_WIN32) || defined(_WIN64) || defined(__MINGW32__)
    return true;
#else
    return false;
#endif
}

#ifdef _WIN32
#pragma comment(lib, "ole32")
#pragma comment(lib, "oleaut32")
#include <cstdio>
#include <Windows.h>
#include <mmdeviceapi.h>
#include <Audioclient.h>
#include <assert.h>
#include <cmath>
#include <string>


static WAVEFORMATEX mixFormat {};

#define discard (void)

enum : UINT8 {
    ECoInit = 1,
    EEnumerator,
    EDevice,
    ECreateAudioClient,
};

extern "C" struct
status {
    bool debug;
} status = {
    false,
};

#define ONERR(c, msg) if(FAILED(hr)){if(status.debug)puts(msg);ret = c;return ret;}

extern "C" UINT8
initialize ()
{
    UINT8 ret = EXIT_SUCCESS;
    if (status.debug) { puts("Initializing"); }
    HRESULT hr = S_OK;
    hr = CoInitializeEx(nullptr, COINIT_SPEED_OVER_MEMORY);
    ONERR(ECoInit, "Failed to initialize");

    IMMDeviceEnumerator* deviceEnum = nullptr;
    hr = CoCreateInstance(__uuidof(MMDeviceEnumerator), nullptr, CLSCTX_ALL, __uuidof(IMMDeviceEnumerator), (LPVOID*)&deviceEnum);
    ONERR(EEnumerator, "Failed to get device enumerator");

    IMMDevice* device = nullptr;
    hr = deviceEnum->GetDefaultAudioEndpoint(eRender, eConsole, &device);
    ONERR(EDevice, "Couldn't get default device");

    IAudioClient2* audioClient = nullptr;    
    hr = device->Activate(__uuidof(IAudioClient2), CLSCTX_ALL, nullptr, (LPVOID*)&audioClient);

    mixFormat.nChannels = 2;
    mixFormat.wBitsPerSample = 16;
    mixFormat.nSamplesPerSec = 44100;
    mixFormat.wFormatTag = WAVE_FORMAT_PCM;
    mixFormat.nBlockAlign = mixFormat.nChannels * mixFormat.wBitsPerSample / 8;
    mixFormat.nAvgBytesPerSec = mixFormat.nSamplesPerSec * mixFormat.nBlockAlign;

    IAudioRenderClient* audioRenderClient = nullptr;
    hr = audioClient->GetService(__uuidof(IAudioRenderClient), (LPVOID*)&audioRenderClient);
    ONERR(1, "j");

    const INT64 REFTIME_PER_SEC = 10'000'000;
    REFERENCE_TIME requestedSoundBufferDuration = REFTIME_PER_SEC * 2;
    DWORD initStreamFlags = (AUDCLNT_STREAMFLAGS_RATEADJUST | AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY);
    hr = audioClient->Initialize(AUDCLNT_SHAREMODE_SHARED,
        initStreamFlags,
        requestedSoundBufferDuration,
        0, &mixFormat, nullptr);
    ONERR(1, "k"); // no fucking idea

    goto RELEASE; // just to make sure...

    hr = audioClient->Start();
    ONERR(6, "Failed to start audio client");

RELEASE:
    // decrements a ref count, doesn't really free it
    // https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    // note: all inherit from IUknown.
    discard deviceEnum->Release();
    discard device->Release();
    discard audioClient->Release();
    discard audioRenderClient->Release();

    return ret;
}

extern "C" void
enable_debug ()
{
    status.debug = true;
}

extern "C" void
disable_debug ()
{
    status.debug = false;
}

#endif
#if 1
// int
// main ()
// {
//     HRESULT hr = S_OK;
//     hr = CoInitializeEx(nullptr, COINIT_SPEED_OVER_MEMORY);
//     check;

//     IMMDeviceEnumerator* deviceEnum = nullptr;
//     hr = CoCreateInstance(__uuidof(MMDeviceEnumerator), nullptr, CLSCTX_ALL, __uuidof(IMMDeviceEnumerator), (LPVOID*)&deviceEnum);
//     check;

//     IMMDevice* device = nullptr;
//     hr = deviceEnum->GetDefaultAudioEndpoint(eRender, eConsole, &device);
//     check;

//     discard deviceEnum->Release();

//     IAudioClient2* audioClient = nullptr;    
//     hr = device->Activate(__uuidof(IAudioClient2), CLSCTX_ALL, nullptr, (LPVOID*)&audioClient);
//     check;

//     discard device->Release();


//     const INT64 REFTIME_PER_SEC = 10'000'000;
//     REFERENCE_TIME requestedSoundBufferDuration = REFTIME_PER_SEC * 2;
//     DWORD initStreamFlags = (AUDCLNT_STREAMFLAGS_RATEADJUST | AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY);
//     hr = audioClient->Initialize(AUDCLNT_SHAREMODE_SHARED,
//         initStreamFlags,
//         requestedSoundBufferDuration,
//         0, &mixFormat, nullptr);
//     check;

//     IAudioRenderClient* audioRenderClient = nullptr;
//     hr = audioClient->GetService(__uuidof(IAudioRenderClient), (LPVOID*)&audioRenderClient);
//     check;

//     UINT32 buffSize;
//     hr = audioClient->GetBufferSize(&buffSize);
//     check;
    
//     hr = audioClient->Start();
//     check;

//     double playbacktime = 0.0f;

//     while (true) {
//         UINT32 bufferPadding;
//         hr = audioClient->GetCurrentPadding(&bufferPadding);
//         check;
//         UINT32 frameCount = buffSize - bufferPadding;
//         INT16* buffer;
//         hr = audioRenderClient->GetBuffer(frameCount, (BYTE **)&buffer);
//         check;

//         for (UINT32 f = 0; f < frameCount; ++f) {
//             float amp = (float)(sin(playbacktime * 2 * 3.1415926 * 440) + (float)sin(playbacktime * 2 * 3.1415926 * 440)) / 2;
//             INT16 y = (INT16)(3000 * amp);

//             *buffer++ =y;//left
//             *buffer++ =y;//right
//             playbacktime += 1.f / mixFormat.nSamplesPerSec;
//         }
//         hr = audioRenderClient->ReleaseBuffer(frameCount, 0);
//         check;

//         IAudioClock* audioClock;
// 		audioClient->GetService(__uuidof(IAudioClock), (LPVOID*)(&audioClock));
// 		UINT64 audioPlaybackFreq;
// 		UINT64 audioPlaybackPos;
// 		audioClock->GetFrequency(&audioPlaybackFreq);
// 		audioClock->GetPosition(&audioPlaybackPos, 0);
// 		audioClock->Release();

// 		UINT64 audioPlaybackPosInSeconds = audioPlaybackPos / audioPlaybackFreq;
// 		UINT64 audioPlaybackPosInSamples = audioPlaybackPosInSeconds * mixFormat.nSamplesPerSec;
//         printf("%f\n", audioPlaybackPosInSamples);
//     }

//     discard audioClient->Stop();
//     discard audioClient->Release();
//     discard audioRenderClient->Release();
//     return 0;
// }
#endif