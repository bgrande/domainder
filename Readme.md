# Why
Domainder is a simple console based tool written in Rust. 
It enables reminding you (via email) about domains going to expire.

For two reasons: Either you want to continue using the domain or might have forgotten about it and don't need it anymore.

# How to use
First, to enable sending emails you should add an email account (smtp). For this, copy `config/email.example.json` to `config/email.json`. 
Then, add your email address, the server's (smtp) hostname, the account's username and password to `config/email.json`.

## Commands
- `Add` - to add a domain with following parameters: `--email, --domain, --time`. While `time` is optional.   
- `Remind` - to actually remind you via email about an expiring domain   

## Add
Here's an example of how to add a domain to remind you about: `domainder add --email you@domain.tld --domain yourdomain.com [--time "1w"]`
The parameters are:
- `--email` to specify the email address the reminder should send the reminder to
- `--domain` to specify the domain to remind about
- `--time` to specify the time range prior the expiry date you should be reminded about the expiry. Default value here is `1m`. You can use any value like `[1-9]+m` for months, `[1-9]+w` for weeks and `[1-9]+d` for days. Be aware that months are calculated as 4 weeks internally right now, so it's not a 100% precise.   
## Remind
Now you have added your domains you can run the command for reminding: `domainder remind`. 
Of course, this doesn't make sense when you have to run it manually, so you should add it as a cronjob like:
```bash
0 6 * * * /path/to/domainder remind
```
So every day at 6am it will run the reminding job and send an email if the email is about to expire.