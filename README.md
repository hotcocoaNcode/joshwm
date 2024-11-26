# joshwm
![Image of really bad window manager here](https://cloud-r7ij49zc9-hack-club-bot.vercel.app/0screenshot_2024-11-26_at_12.35.19___am.png)
> *certified "tryharding" for hack club riceathon*
_____
`joshwm` is a "blazingly fast" window manager written in Rust with Xlib (not fast), (originally) for the Hack Club Riceathon.
## Basic features
- Framed windows
- Moving and resizing the windows
- Very iffy Ctrl-Q that may work sometimes-ish <sub>(that keybind instead of Alt-F4 because XQuartz+Xephyr has a grudge against me)</sub>
- Shift click to bring a window to the front
## Why should I use JoshWM?
__You shouldn't__. This was a learning exercise. Please don't use this. Please PLEASE don't use this.
## Why did you make this?
Because I wanted to learn Rust (better than a hello world print and a terminal calculator). 
I'm REALLY bad at making anything look good,
and Leah from slack [said I could write a WM instead of customizing one](https://hackclub.slack.com/archives/C0266FRGT/p1731786509542659?thread_ts=1731631348.490099&cid=C0266FRGT), 
which is a lot more my speed.
## Okay, but why do the riceathon if you can't rice?
Because *the funny*. Also, prizes are either get a blahaj or thigh highs... worth it even if I look like an idiot submitting *definitely not a rice*.
## Why Xlib rather than XCB?
I'm not ballsy enough for that. Let me take a break from Vulkan in peace, please...
## "X11 is dead! Long live Wayland!"
Well yes, soon probably (as in ~5-10 years) Wayland will be more widely adopted. 
But with the same as above, *let me take a break from Vulkan in peace*.

Funnily enough the parallels between the X window system/Wayland and OpenGL and Vulkan are definitely there;
- OpenGL/X window system
  - Old
  - Incredibly easy to write code for, tutorial ridden and well trodden.
  - Becoming deprecated in favor of a new better alternative
- Vulkan/Wayland
  - The better alternative in question
  - New with slightly lackluster support (although Vulkan is not too much of this anymore)
  - "You like writing 1k loc hello worlds, don't you buddy? Yeah, of course you do."