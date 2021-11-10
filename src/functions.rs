use libloading::Library;
use once_cell::sync::Lazy;
use bass_sys::{generate_bindings, BOOL, DWORD, HSTREAM, HSYNC, QWORD, SYNCPROC};
use std::{env, os::raw::{c_int, c_void}};
use crate::BassMixerNode;

static BASS_LIBRARY: Lazy<Library> = Lazy::new(|| {
    // TODO: Privilege escalation (?)
    // https://doc.rust-lang.org/std/env/fn.current_exe.html#security
    if let Ok(mut library_path) = env::current_exe() {
        library_path.pop();

        #[cfg(target_os = "windows")]
        library_path.push("bassmix.dll");

        #[cfg(target_os = "linux")]
        library_path.push("libbassmix.so");

        #[cfg(target_os = "macos")]
        library_path.push("libbassmix.dylib");

        if let Ok(library) = unsafe { Library::new(library_path) } {
            return library;
        } else {
            panic!("Failed to load the library.");
        }
    } else {
        panic!("Failed to load the library.");
    }
});

generate_bindings! {
    binding BASS_MIXER_GET_VERSION fn BASS_Mixer_GetVersion() -> DWORD;
    binding BASS_MIXER_STREAM_CREATE fn BASS_Mixer_StreamCreate(frequency: DWORD, channels: DWORD, flags: DWORD) -> HSTREAM;
    binding BASS_MIXER_STREAM_ADD_CHANNEL fn BASS_Mixer_StreamAddChannel(
        handle: HSTREAM,
        channel: DWORD,
        flags: DWORD
    ) -> BOOL;
    binding BASS_MIXER_STREAM_ADD_CHANNEL_EX fn BASS_Mixer_StreamAddChannelEx(
        handle: HSTREAM,
        channel: DWORD,
        flags: DWORD,
        start: QWORD,
        length: QWORD
    ) -> BOOL;
    binding BASS_MIXER_STREAM_GET_CHANNELS fn BASS_Mixer_StreamGetChannels(
        handle: HSTREAM,
        channels: *mut DWORD,
        count: DWORD
    ) -> DWORD;
    binding BASS_MIXER_CHANNEL_GET_MIXER fn BASS_Mixer_ChannelGetMixer(handle: DWORD) -> HSTREAM;
    binding BASS_MIXER_CHANNEL_IS_ACTIVE fn BASS_Mixer_ChannelIsActive(handle: DWORD) -> DWORD;
    binding BASS_MIXER_CHANNEL_FLAGS fn BASS_Mixer_ChannelFlags(handle: DWORD, flags: DWORD, mask: DWORD) -> DWORD;
    binding BASS_MIXER_CHANNEL_REMOVE fn BASS_Mixer_ChannelRemove(handle: DWORD) -> BOOL;
    binding BASS_MIXER_CHANNEL_SET_POSITION fn BASS_Mixer_ChannelSetPosition(
        handle: DWORD,
        pos: DWORD,
        mode: DWORD
    ) -> BOOL;
    binding BASS_MIXER_CHANNEL_GET_POSITION fn BASS_Mixer_ChannelGetPosition(handle: DWORD, mode: DWORD) -> QWORD;
    binding BASS_MIXER_CHANNEL_GET_POSITION_EX fn BASS_Mixer_ChannelGetPositionEx(
        handle: DWORD, 
        mode: DWORD,
        delay: DWORD
    ) -> QWORD;
    binding BASS_MIXER_CHANNEL_GET_LEVEL fn BASS_Mixer_ChannelGetLevel(handle: DWORD) -> DWORD;
    binding BASS_MIXER_CHANNEL_GET_LEVEL_EX fn BASS_Mixer_ChannelGetLevelEx(
        handle: DWORD, 
        levels: *mut f32, 
        length: f32, 
        flags: DWORD
    ) -> BOOL;
    binding BASS_MIXER_CHANNEL_GET_DATA fn BASS_Mixer_ChannelGetData(handle: DWORD, buffer: *mut c_void, length: DWORD) -> DWORD;
    binding BASS_MIXER_CHANNEL_SET_SYNC fn BASS_Mixer_ChannelSetSync(
        handle: DWORD,
        sync_type: DWORD,
        parameter: QWORD,
        proc: *mut SYNCPROC,
        user: *mut c_void
    ) -> HSYNC;
    binding BASS_MIXER_CHANNEL_REMOVE_SYNC fn BASS_Mixer_ChannelRemoveSync(handle: DWORD, sync: HSYNC) -> BOOL;
    binding BASS_MIXER_CHANNEL_SET_MATRIX fn BASS_Mixer_ChannelSetMatrix(handle: DWORD, matrix: *const c_void) -> BOOL;
    binding BASS_MIXER_CHANNEL_SET_MATRIX_EX fn BASS_Mixer_ChannelSetMatrixEx(
        handle: DWORD, 
        matrix: *const c_void,
        time: f32
    ) -> BOOL;
    binding BASS_MIXER_CHANNEL_GET_MATRIX fn BASS_Mixer_ChannelGetMatrix(handle: DWORD, matrix: *mut c_void) -> BOOL;
    binding BASS_MIXER_CHANNEL_SET_ENVELOPE fn BASS_Mixer_ChannelSetEnvelope(
        handle: DWORD, 
        envelope_type: DWORD,
        nodes: *const BassMixerNode,
        count: DWORD
    ) -> BOOL;
    binding BASS_MIXER_CHANNEL_SET_ENVELOPE_POS fn BASS_Mixer_ChannelSetEnvelopePos(
        handle: DWORD, 
        envelope_type: DWORD,
        pos: QWORD
    ) -> BOOL;
    binding BASS_MIXER_CHANNEL_GET_ENVELOPE_POS fn BASS_Mixer_ChannelGetEnvelopePos(
        handle: DWORD, 
        envelope_type: DWORD,
        value: *mut f32
    ) -> QWORD;
    
    binding BASS_SPLIT_STREAM_CREATE fn BASS_Split_StreamCreate(channel: DWORD, flags: DWORD, chanmap: *const c_int) -> HSTREAM;
    binding BASS_SPLIT_STREAM_GET_SOURCE fn BASS_Split_StreamGetSource(handle: HSTREAM) -> DWORD;
    binding BASS_SPLIT_STREAM_GET_SPLITS fn BASS_Split_StreamGetSplits(handle: DWORD, splits: *mut HSTREAM, count: DWORD) -> DWORD;
    binding BASS_SPLIT_STREAM_RESET fn BASS_Split_StreamReset(handle: DWORD) -> BOOL;
    binding BASS_SPLIT_STREAM_RESET_EX fn BASS_Split_StreamResetEx(handle: DWORD, offset: DWORD) -> BOOL;
    binding BASS_SPLIT_STREAM_GET_AVAILABLE fn BASS_Split_StreamGetAvailable(handle: DWORD) -> DWORD;
}