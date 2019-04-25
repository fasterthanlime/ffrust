// See https://github.com/meh/rust-ffmpeg-sys/blob/master/build.rs

#include <libavcodec/avcodec.h>
#include <libavcodec/dv_profile.h>
#include <libavcodec/avfft.h>
#include <libavcodec/vaapi.h>
#include <libavcodec/vorbis_parser.h>

#include <libavformat/avformat.h>
#include <libavformat/avio.h>

#include <libavutil/adler32.h>
#include <libavutil/aes.h>
#include <libavutil/audio_fifo.h>
#include <libavutil/base64.h>
#include <libavutil/blowfish.h>
#include <libavutil/bprint.h>
#include <libavutil/buffer.h>
#include <libavutil/camellia.h>
#include <libavutil/cast5.h>
#include <libavutil/channel_layout.h>
#include <libavutil/cpu.h>
#include <libavutil/crc.h>
#include <libavutil/dict.h>
#include <libavutil/display.h>
#include <libavutil/downmix_info.h>
#include <libavutil/error.h>
#include <libavutil/eval.h>
#include <libavutil/fifo.h>
#include <libavutil/file.h>
#include <libavutil/frame.h>
#include <libavutil/hash.h>
#include <libavutil/hmac.h>
#include <libavutil/imgutils.h>
#include <libavutil/lfg.h>
#include <libavutil/log.h>
#include <libavutil/macros.h>
#include <libavutil/mathematics.h>
#include <libavutil/md5.h>
#include <libavutil/mem.h>
#include <libavutil/motion_vector.h>
#include <libavutil/murmur3.h>
#include <libavutil/opt.h>
#include <libavutil/parseutils.h>
#include <libavutil/pixdesc.h>
#include <libavutil/pixfmt.h>
#include <libavutil/random_seed.h>
#include <libavutil/rational.h>
#include <libavutil/replaygain.h>
#include <libavutil/ripemd.h>
#include <libavutil/samplefmt.h>
#include <libavutil/sha.h>
#include <libavutil/sha512.h>
#include <libavutil/stereo3d.h>
#include <libavutil/avstring.h>
#include <libavutil/threadmessage.h>
#include <libavutil/time.h>
#include <libavutil/timecode.h>
#include <libavutil/twofish.h>
#include <libavutil/avutil.h>
#include <libavutil/xtea.h>

#include <libswscale/swscale.h>

#include <libswresample/swresample.h>
