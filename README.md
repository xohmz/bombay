# Bombay - Monstercat API Bindings

![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/xohmz/bombay/Rust?branch=main)
![Crates.io](https://img.shields.io/crates/v/bombay)
![Crates.io (latest)](https://img.shields.io/crates/dv/bombay)

This library provides safe Rust bindings for the Monstercat API.

Finally, the prophecy foretold becomes reality as crustaceans join the
Monstercat community and Ferris joins the [crab rave]. Bombay is named
after the cat breed and after my favorite liquor.

## Disclaimer

Bombay is not endorsed by Monstercat in any way, and I don't even
know if they like me doing this. Be responsible! Do not use this library
to abuse, compete with, or otherwise irritate Monstercat. This should go
without saying, but please do not try to use this library to get around
any access or copyright restrictions.

Bombay interacts with the Monstercat API, which is completely out
of my control and subject to change at any time. For this reason, the
library can stop working correctly or at all, at any moment. It is written
to be strict and safe, for good reason: I do not want any unexpected
behavior to result in requests that would upset Monstercat. Please be
mindful of all this and avoid relying on this library for anything
critically important to you or others, and also avoid altering the code in
ways that might irritate Monstercat if something goes wrong.

I am not responsible for what you do with this project or the knowledge
you gain from it, so use it at your own risk.

## Thanks

[Monstercat], for bringing together an incredible community of talented
artists and listeners. And also for exposing this API and letting us
(I think) play with it!.

[defvs] and other contributors, for [connect-v2-docs]. I first started
writing this by poking around the Monstercat web player, but then I found
those docs and it saved me tons of work and I only had to update a few things
from there.

[DNSimple], for [dnsimple-rust]. The structure of this library's client
was inspired by that library. I do not know if their approach is "good"
or if my derived approach is either, but I appreciate them!

## Docs and Tests

There is not full coverage of endpoints and endpoint options/parameters.

There are some examples in the [Bombay docs.rs page][docs]. More will be added
as I have time. Please navigate the various modules and structs to see how to
use them. Not all functions have examples. More will be added as I have time.

There are integration tests to attempt to catch changes to the Monstercat API,
and to serve as more examples of how to use the library.  More will be added as
I have time. The error approach handling in those is a little different from
the function examples.

Some tests are ignored if the following environment variables are not set:

* MC_EMAIL
* MC_PASSWORD
* MC_TOTP_SECRET

## Capabilities

Bombay supports the following:

* [x] Artists
  * [x] Get all
  * [x] Get latest
  * [x] Get by artist name URI
  * [x] Get photo
* [x] Moods
  * [x] Get all
  * [x] Get by mood name URI
* [x] Playlists
  * [x] Get top 30 playlist
  * [x] Get by playlist ID
  * [x] Get playlist tracks
  * [x] Get user playlists
  * [x] Create playlist
  * [x] Edit playlist
  * [x] Modify a playlist item
  * [x] Modify multiple playlist items
  * [x] Delete a playlist
  * [x] Get tile image
  * [x] Get background image
* [x] Releases
  * [x] Get all
  * [x] Get latest
  * [x] Get by artist name URI
  * [x] Get by release catalog ID
  * [x] Get related by release ID
  * [x] Get cover art
  * [x] Stream track
  * [x] Download track
* [x] User
  * [x] Sign-in / Authentication
    * [x] email + password + 2FA (whichever requested: none, TOTP, or email)
    * [x] email + password + 2FA (try to use email link)
    * [x] email + password + 2FA (try to use totp)
  * [x] Get user information and settings
  * [x] Set user information and settings (supported values only)
  * [x] Set notification interests
  * [x] Set email
  * [x] Set password
  * [x] Enable 2FA email
  * [x] Disable 2FA email
  * [x] Enable 2FA TOTP
  * [x] Disable 2FA TOTP
  * [x] Get 2FA TOTP qr-code
  * [x] Get licenses
  * [x] Remove license
  * [x] Remove video claims
  * [x] Get streaming widget player code
  * [x] Generate streaming widget player code
  * [x] Generate shop discount code

And, where applicable:

* [x] Search and filtering parameters
* [x] Pagination parameters

### Next Steps

* Iterating on paginated queries/responses.
* Logging in with other social accounts.

## ❗ Security

Bombay is by no means cryptographically secure. It simply abides by the
Monstercat API. I did not make a strong effort towards best security practices
in how sign-in details are used and stored. Be sure to use Bombay in a trusted
environment.

## Issues

Throughout the development of this library, I constantly encountered responses
that did not fit my expectation and broke the deserialization. If you notice
something is not deserializing, feel free to open an issue and I will take a
look when I have time. Alternatively, put up a pull request!

## Maturity

There are still plenty of gaps. As I have time to revisit best practices, I
will improve things and there may be some breaking changes. I will bump the
Bombay version to 0.1.0 when I believe it is stable, baring any major changes
due to Monstercat's API changing. I'm not sure if there will ever be a 1.0.0,
since I have no control over their API; I don't want to create a false sense
of reliability.

## License

[MIT License], Copyright (c) 2023 xohmz

[crab rave]: https://youtu.be/LDU_Txk06tM?t=30
[Monstercat]: https://www.monstercat.com/
[defvs]: https://github.com/defvs/
[connect-v2-docs]: https://github.com/defvs/connect-v2-docs/wiki
[DNSimple]: https://dnsimple.com/
[dnsimple-rust]: https://github.com/dnsimple/dnsimple-rust
[docs]: https://docs.rs/bombay/latest/bombay/
[MIT License]: http://opensource.org/licenses/MIT
