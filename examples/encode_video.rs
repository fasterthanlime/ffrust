use const_cstr::const_cstr;
use ffrust;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::ptr;

fn main() {
    unsafe {
        let codec_name = "libx264";

        let codec = ffrust::avcodec_find_encoder_by_name(CString::new("libx264").unwrap().as_ptr());
        if codec.is_null() {
            panic!("Codec '{}' not found", codec_name);
        }

        let mut c = ffrust::avcodec_alloc_context3(codec);
        if c.is_null() {
            panic!("Could not allocate video codec context");
        }

        let mut pkt = ffrust::av_packet_alloc();
        if pkt.is_null() {
            panic!("Could not allocate packet");
        }

        {
            let c = &mut (*c);

            // put sample parameters
            c.bit_rate = 400_000;
            // resolution must be a multiple of two
            c.width = 352;
            c.height = 288;
            c.time_base = ffrust::AVRational { num: 1, den: 25 };
            c.framerate = ffrust::AVRational { num: 25, den: 1 };
            c.gop_size = 10;
            c.max_b_frames = 1;
            c.pix_fmt = ffrust::AVPixelFormat::AV_PIX_FMT_YUV420P;
        }

        if (*codec).id == ffrust::AVCodecID::AV_CODEC_ID_H264 {
            ffrust::av_opt_set(
                (*c).priv_data,
                const_cstr!("preset").as_ptr(),
                const_cstr!("slow").as_ptr(),
                0,
            );
        }

        let ret = ffrust::avcodec_open2(c, codec, ptr::null_mut());
        if ret < 0 {
            panic!("Could not open codec: {}", err2str(ret));
        }

        let mut f = std::fs::File::create("output.h264").unwrap();

        let mut frame = ffrust::av_frame_alloc();
        if frame.is_null() {
            panic!("Could not allocate video frame");
        }
        {
            let frame = &mut (*frame);
            let c = &*c;
            frame.format = c.pix_fmt as i32;
            frame.width = c.width;
            frame.height = c.height;
        }

        let ret = ffrust::av_frame_get_buffer(frame, 32);
        if ret < 0 {
            panic!("Could not allocate the video frame data: {}", err2str(ret));
        }

        for i in 0..25 * 2 {
            let ret = ffrust::av_frame_make_writable(frame);
            if ret < 0 {
                panic!("Could not make frame writable: {}", err2str(ret));
            }

            {
                let c = &mut *c;
                let frame = &mut *frame;

                // prepare a dummy image
                // Y
                for y in 0..c.height {
                    for x in 0..c.width {
                        *frame.data[0].offset((y * frame.linesize[0] + x) as isize) =
                            (x + y + i * 3) as u8;
                    }
                }

                // Cb and Cr
                for y in 0..c.height / 2 {
                    for x in 0..c.width / 2 {
                        *frame.data[1].offset((y * frame.linesize[1] + x) as isize) =
                            (128 + y + i * 2) as u8;
                        *frame.data[2].offset((y * frame.linesize[2] + x) as isize) =
                            (64 + y + i * 5) as u8;
                    }
                }

                frame.pts = i as i64;
            }
            encode(c, frame, pkt, &mut f);
        }

        // flush the encoder
        encode(c, ptr::null_mut(), pkt, &mut f);

        // add sequence end code to have a real MPEG file
        let endcode: [u8; 4] = [0, 0, 1, 0xb7];
        f.write(&endcode[..]).unwrap();
        drop(f);

        ffrust::avcodec_free_context(&mut c);
        ffrust::av_frame_free(&mut frame);
        ffrust::av_packet_free(&mut pkt);

        println!("All good!");
    }
}

unsafe fn encode(
    enc_ctx: *mut ffrust::AVCodecContext,
    frame: *mut ffrust::AVFrame,
    pkt: *mut ffrust::AVPacket,
    file: &mut std::fs::File,
) {
    // send the frame to the encoder
    if !frame.is_null() {
        println!("Send frame {}", (*frame).pts);
    }

    let mut ret = ffrust::avcodec_send_frame(enc_ctx, frame);
    if ret < 0 {
        panic!("Error sending a frame for encoding: {}", err2str(ret));
    }

    while ret >= 0 {
        ret = ffrust::avcodec_receive_packet(enc_ctx, pkt);
        if ret == ffrust::AVERROR(libc::EAGAIN) || ret == ffrust::AVERROR_EOF {
            return;
        } else if ret < 0 {
            panic!("Error during encoding: {}", err2str(ret));
        }

        {
            let pkt = &*pkt;
            println!("Write packet {} (size={})", pkt.pts, pkt.size);
            let data = std::slice::from_raw_parts(pkt.data, pkt.size as usize);
            file.write(data).unwrap();
        }
    }
}

fn err2str(errnum: libc::c_int) -> String {
    unsafe {
        const BUFCAP: usize = 1024;
        let mut buf = [0i8; BUFCAP];
        ffrust::av_strerror(errnum, &mut buf[0], BUFCAP as usize) as usize;
        CStr::from_ptr(&mut buf[0])
            .to_string_lossy()
            .to_owned()
            .to_string()
    }
}
