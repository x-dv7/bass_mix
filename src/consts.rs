use bass_sys::DWORD;
//use std::os::raw::c_int;

// Additional BASS_SetConfig options
pub const BASS_CONFIG_MIXER_BUFFER: DWORD = 0x10601;
pub const BASS_CONFIG_MIXER_POSEX: DWORD = 0x10602;
pub const BASS_CONFIG_SPLIT_BUFFER: DWORD = 0x10610;

// BASS_Mixer_StreamCreate flags
pub const BASS_MIXER_END: DWORD = 0x10000;// end the stream when there are no sources
pub const BASS_MIXER_NONSTOP: DWORD = 0x20000;// don't stall when there are no sources
pub const BASS_MIXER_RESUME: DWORD = 0x1000;// resume stalled immediately upon new/unpaused source
pub const BASS_MIXER_POSEX: DWORD = 0x2000;// enable BASS_Mixer_ChannelGetPositionEx support			

// BASS_Mixer_StreamAddChannel/Ex flags
pub const BASS_MIXER_CHAN_ABSOLUTE: DWORD = 0x1000;// start is an absolute position
pub const BASS_MIXER_CHAN_BUFFER: DWORD = 0x2000;// buffer data for BASS_Mixer_ChannelGetData/Level
pub const BASS_MIXER_CHAN_LIMIT: DWORD = 0x4000;// limit mixer processing to the amount available from this source
pub const BASS_MIXER_CHAN_MATRIX: DWORD = 0x10000;// matrix mixing
pub const BASS_MIXER_CHAN_PAUSE: DWORD = 0x20000;// don't process the source
pub const BASS_MIXER_CHAN_DOWNMIX: DWORD = 0x400000;// downmix to stereo/mono
pub const BASS_MIXER_CHAN_NORAMPIN: DWORD = 0x800000;// don't ramp-in the start		 

pub const BASS_MIXER_BUFFER: DWORD = BASS_MIXER_CHAN_BUFFER;
pub const BASS_MIXER_LIMIT: DWORD = BASS_MIXER_CHAN_LIMIT;
pub const BASS_MIXER_MATRIX: DWORD = BASS_MIXER_CHAN_MATRIX;
pub const BASS_MIXER_PAUSE: DWORD = BASS_MIXER_CHAN_PAUSE;
pub const BASS_MIXER_DOWNMIX: DWORD = BASS_MIXER_CHAN_DOWNMIX;
pub const BASS_MIXER_NORAMPIN: DWORD = BASS_MIXER_CHAN_NORAMPIN;		

// Mixer attributes
pub const BASS_ATTRIB_MIXER_LATENCY: DWORD = 0x15000;
pub const BASS_ATTRIB_MIXER_THREADS: DWORD = 0x15001;	

// Additional BASS_Mixer_ChannelIsActive return values
pub const BASS_ACTIVE_WAITING: DWORD = 5;			

// BASS_Split_StreamCreate flags
pub const BASS_SPLIT_SLAVE: DWORD = 0x1000;// only read buffered data
pub const BASS_SPLIT_POS: DWORD = 0x2000;			

// Splitter attributes
pub const BASS_ATTRIB_SPLIT_ASYNCBUFFER: DWORD = 0x15010;
pub const BASS_ATTRIB_SPLIT_ASYNCPERIOD: DWORD = 0x15011;		

// Envelope types
pub const BASS_MIXER_ENV_FREQ: DWORD = 1;
pub const BASS_MIXER_ENV_VOL: DWORD = 2;
pub const BASS_MIXER_ENV_PAN: DWORD = 3;
pub const BASS_MIXER_ENV_LOOP: DWORD = 0x10000;// flag: loop
pub const BASS_MIXER_ENV_REMOVE: DWORD = 0x20000;// flag: remove at end	 

// Additional sync types
pub const BASS_SYNC_MIXER_ENVELOPE: DWORD = 0x10200;
pub const BASS_SYNC_MIXER_ENVELOPE_NODE: DWORD = 0x10201;

// Additional BASS_Mixer_ChannelSetPosition flag
pub const BASS_POS_MIXER_RESET: DWORD = 0x10000;// flag: clear mixer's playback buffer	 

// Additional BASS_Mixer_ChannelGetPosition mode
pub const BASS_POS_MIXER_DELAY: DWORD = 5;	

// BASS_CHANNELINFO types
pub const BASS_CTYPE_STREAM_MIXER: DWORD = 0x10800;
pub const BASS_CTYPE_STREAM_SPLIT: DWORD = 0x10801;	