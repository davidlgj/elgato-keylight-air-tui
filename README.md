# Simple Elgato Keylight Air TUI

This is a super simple TUI to control brightness and temperature of my Elgato Keylight Air.
There are already a couple of different options to do this and I mainly created this as
an excuse to play around a bit with Rust :)

I.e. this is probably not what you want.




https://github.com/user-attachments/assets/880659ac-3cd1-44f8-bc80-c7272732fc4e






### Design requirements

 * Needs to take ip address of light since I have a some network "challenges" and the light is behind a NAT
 * Must work on both Mac OS X (untested) and Linux since I'm forced to use Mac at work.


## Thanks

Thanks to @adamesch for his repo documenting the API of the lamp, https://github.com/adamesch/elgato-key-light-api

## License

Copyright (c) David Jensen <david.lgj@gmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
