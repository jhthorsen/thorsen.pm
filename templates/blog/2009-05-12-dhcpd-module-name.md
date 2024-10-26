---
title: Which name to pick?
---

I'm writing a new [module](http://github.com/jhthorsen/net-isc-dhcpd),
which is currently named "Net::DHCPd". It's not a good name imo, since
Net::DHCPd sounds more like a module that actually *is* a dhcp server,
which mine is not.

This is what it can, or is supposed to support in the future:

-   Interact with ISC DHCPd
-   Start/stop/restart the daemon
-   Parse and build config
-   Check config file for error
-   Parse leases file

So what should it be called? I'm thinking about naming it ISC::DHCPd,
since it's primary target is to interact with that spesific server. But
can I call it the same as what [the actual
server](https://www.isc.org/software/dhcp) is called?

Let me know if you have any idea.
