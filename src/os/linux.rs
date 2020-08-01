#![allow(non_camel_case_types)]

use libc::{c_int,c_uint,c_uchar};

pub type cc_t = c_uchar;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    c_line: cc_t,
    pub c_cc: [cc_t; NCCS],
    c_ispeed: speed_t,
    c_ospeed: speed_t
}

pub const NCCS: usize = 23;

// c_cc characters
pub const VINTR:    usize = 0;  /* Interrupt character [ISIG].  */
pub const VQUIT:    usize = 1;  /* Quit character [ISIG].  */
pub const VERASE:   usize = 2;  /* Erase character [ICANON].  */
pub const VKILL:    usize = 3;  /* Kill-line character [ICANON].  */
pub const VMIN:     usize = 4;  /* Minimum number of bytes read at once [!ICANON].  */
pub const VTIME:    usize = 5;  /* Time-out value (tenths of a second) [!ICANON].  */
pub const VEOL2:    usize = 6;  /* Second EOL character [ICANON].  */
pub const VSWTC:    usize = 7;  /* ??? */
pub const VSWTCH:   usize = 7;
pub const VSTART:   usize = 8;  /* Start (X-ON) character [IXON, IXOFF].  */
pub const VSTOP:    usize = 9;  /* Stop (X-OFF) character [IXON, IXOFF].  */
pub const VSUSP:    usize = 10; /* Suspend character [ISIG].  */
//pub const VDSUSP: usize = 11; /* Delayed suspend character [ISIG].  */
pub const VREPRINT: usize = 12; /* Reprint-line character [ICANON].  */
pub const VDISCARD: usize = 13; /* Discard character [IEXTEN].  */
pub const VWERASE:  usize = 14; /* Word-erase character [ICANON].  */
pub const VLNEXT:   usize = 15; /* Literal-next character [IEXTEN].  */
pub const VEOF:     usize = 16; /* End-of-file character [ICANON].  */
pub const VEOL:     usize = 17; /* End-of-line character [ICANON].  */

// c_iflag bits
pub const IGNBRK:   tcflag_t = 0o0000001; /* Ignore break condition.  */
pub const BRKINT:   tcflag_t = 0o0000002; /* Signal interrupt on break.  */
pub const IGNPAR:   tcflag_t = 0o0000004; /* Ignore characters with parity errors.  */
pub const PARMRK:   tcflag_t = 0o0000010; /* Mark parity and framing errors.  */
pub const INPCK:    tcflag_t = 0o0000020; /* Enable input parity check.  */
pub const ISTRIP:   tcflag_t = 0o0000040; /* Strip 8th bit off characters.  */
pub const INLCR:    tcflag_t = 0o0000100; /* Map NL to CR on input.  */
pub const IGNCR:    tcflag_t = 0o0000200; /* Ignore CR.  */
pub const ICRNL:    tcflag_t = 0o0000400; /* Map CR to NL on input.  */
pub const IUCLC:    tcflag_t = 0o0001000; /* Map upper case to lower case on input.  */
pub const IXON:     tcflag_t = 0o0002000; /* Enable start/stop output control.  */
pub const IXANY:    tcflag_t = 0o0004000; /* Any character will restart after stop.  */
pub const IXOFF:    tcflag_t = 0o0010000; /* Enable start/stop input control.  */
pub const IMAXBEL:  tcflag_t = 0o0020000; /* Ring bell when input queue is full.  */
pub const IUTF8:    tcflag_t = 0o0040000; /* Input is UTF-8 */

// c_oflag bits
pub const OPOST:    tcflag_t = 0o0000001; /* Perform output processing.  */
pub const OLCUC:    tcflag_t = 0o0000002; /* Map lower case to upper case on output.  */
pub const ONLCR:    tcflag_t = 0o0000004; /* Map NL to CR-NL on output.  */
pub const OCRNL:    tcflag_t = 0o0000010;
pub const ONOCR:    tcflag_t = 0o0000020;
pub const ONLRET:   tcflag_t = 0o0000040;
pub const OFILL:    tcflag_t = 0o0000100;
pub const OFDEL:    tcflag_t = 0o0000200;
pub const NLDLY:    tcflag_t = 0o0000400;
pub const NL0:      tcflag_t = 0o0000000;
pub const NL1:      tcflag_t = 0o0000400;
pub const CRDLY:    tcflag_t = 0o0003000;
pub const CR0:      tcflag_t = 0o0000000;
pub const CR1:      tcflag_t = 0o0001000;
pub const CR2:      tcflag_t = 0o0002000;
pub const CR3:      tcflag_t = 0o0003000;
pub const TABDLY:   tcflag_t = 0o0014000;
pub const TAB0:     tcflag_t = 0o0000000;
pub const TAB1:     tcflag_t = 0o0004000;
pub const TAB2:     tcflag_t = 0o0010000;
pub const TAB3:     tcflag_t = 0o0014000;
pub const XTABS:    tcflag_t = 0o0014000;
pub const BSDLY:    tcflag_t = 0o0020000;
pub const BS0:      tcflag_t = 0o0000000;
pub const BS1:      tcflag_t = 0o0020000;
pub const VTDLY:    tcflag_t = 0o0040000;
pub const VT0:      tcflag_t = 0o0000000;
pub const VT1:      tcflag_t = 0o0040000;
pub const FFDLY:    tcflag_t = 0o0100000;
pub const FF0:      tcflag_t = 0o0000000;
pub const FF1:      tcflag_t = 0o0100000;

// c_cflag bits
pub const CBAUD:    tcflag_t = 0o0010017; 
pub const CSIZE:    tcflag_t = 0o0000060; /* Number of bits per byte (mask).  */
pub const CS5:      tcflag_t = 0o0000000; /* 5 bits per byte.  */
pub const CS6:      tcflag_t = 0o0000020; /* 6 bits per byte.  */
pub const CS7:      tcflag_t = 0o0000040; /* 7 bits per byte.  */
pub const CS8:      tcflag_t = 0o0000060; /* 8 bits per byte.  */
pub const CSTOPB:   tcflag_t = 0o0000100; /* Two stop bits instead of one.  */
pub const CREAD:    tcflag_t = 0o0000200; /* Enable receiver.  */
pub const PARENB:   tcflag_t = 0o0000400; /* Parity enable.  */
pub const PARODD:   tcflag_t = 0o0001000; /* Odd parity instead of even.:   tcflag_t = 0o*/
pub const HUPCL:    tcflag_t = 0o0002000; /* Hang up on last close.  */
pub const CLOCAL:   tcflag_t = 0o0004000; /* Ignore modem status lines.  */
pub const CBAUDEX:  tcflag_t = 0o0010000;
pub const BOTHER:   tcflag_t = 0o0010000;
pub const CIBAUD:   tcflag_t = 0o002003600000; /* input baud rate */
pub const CMSPAR:   tcflag_t = 0o010000000000; /* mark or space (stick) parity */
pub const CRTSCTS:  tcflag_t = 0o020000000000; /* flow control */


// c_lflag bits
pub const ISIG:     tcflag_t = 0o0000001; /* Enable signals.  */
pub const ICANON:   tcflag_t = 0o0000002; /* Do erase and kill processing.  */
pub const XCASE:    tcflag_t = 0o0000004;
pub const ECHO:     tcflag_t = 0o0000010; /* Enable echo.:      tcflag_t = 0o */
pub const ECHOE:    tcflag_t = 0o0000020; /* Visual erase for ERASE.  */
pub const ECHOK:    tcflag_t = 0o0000040; /* Echo NL after KILL.:       tcflag_t = 0o*/
pub const ECHONL:   tcflag_t = 0o0000100; /* Echo NL even if ECHO is off.:      tcflag_t = 0o */
pub const NOFLSH:   tcflag_t = 0o0000200; /* Disable flush after interrupt.  */
pub const IEXTEN:   tcflag_t = 0o0000400; /* Enable DISCARD and LNEXT.  */
pub const ECHOCTL:  tcflag_t = 0o0001000; /* Echo control characters as ^X.  */
pub const ECHOPRT:  tcflag_t = 0o0002000; /* Hardcopy visual erase.  */
pub const ECHOKE:   tcflag_t = 0o0004000; /* Visual erase for KILL.  */
pub const FLUSHO:   tcflag_t = 0o0020000;
pub const PENDIN:   tcflag_t = 0o0040000; /* Retype pending input (state).  */
pub const TOSTOP:   tcflag_t = 0o0100000; /* Send SIGTTOU for background output.:       tcflag_t = 0o*/
pub const ITOSTOP:  tcflag_t = TOSTOP;
pub const EXTPROC:   tcflag_t = 0o0200000; /* External processing on pty */

// baud rates
pub const B0:       speed_t = 0o0000000;
pub const B50:      speed_t = 0o0000001;
pub const B75:      speed_t = 0o0000002;
pub const B110:     speed_t = 0o0000003;
pub const B134:     speed_t = 0o0000004;
pub const B150:     speed_t = 0o0000005;
pub const B200:     speed_t = 0o0000006;
pub const B300:     speed_t = 0o0000007;
pub const B600:     speed_t = 0o0000010;
pub const B1200:    speed_t = 0o0000011;
pub const B1800:    speed_t = 0o0000012;
pub const B2400:    speed_t = 0o0000013;
pub const B4800:    speed_t = 0o0000014;
pub const B9600:    speed_t = 0o0000015;
pub const B19200:   speed_t = 0o0000016;
pub const B38400:   speed_t = 0o0000017;
pub const EXTA:     speed_t = B19200;
pub const EXTB:     speed_t = B38400;
pub const B57600:   speed_t = 0o0010001;
pub const B115200:  speed_t = 0o0010002;
pub const B230400:  speed_t = 0o0010003;
pub const B460800:  speed_t = 0o0010004;
pub const B500000:  speed_t = 0o0010005;
pub const B576000:  speed_t = 0o0010006;
pub const B921600:  speed_t = 0o0010007;
pub const B1000000: speed_t = 0o0010010;
pub const B1152000: speed_t = 0o0010011;
pub const B1500000: speed_t = 0o0010012;
pub const B2000000: speed_t = 0o0010013;
pub const B2500000: speed_t = 0o0010014;
pub const B3000000: speed_t = 0o0010015;
pub const B3500000: speed_t = 0o0010016;
pub const B4000000: speed_t = 0o0010017;

// tcflow()
pub const TCOOFF: c_int = 0;
pub const TCOON:  c_int = 1;
pub const TCIOFF: c_int = 2;
pub const TCION:  c_int = 3;

// tcflush()
pub const TCIFLUSH:  c_int = 0;
pub const TCOFLUSH:  c_int = 1;
pub const TCIOFLUSH: c_int = 2;

// tcsetattr()
pub const TCSANOW:   c_int = 0x540E;
pub const TCSADRAIN: c_int = 0x540F;
pub const TCSAFLUSH: c_int = 0x5410;
