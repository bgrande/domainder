# What
Domainder is a simple console based tool written in Rust. 
It enables reminding you (via email) about domains going to expire.
The beauty of this solution is the domain expiry is fetched via `whois` so for most domains you don't need to specify an expiry date. 
      
## Why
For two reasons: Either you want to continue using the domain or might have forgotten about it and don't need it anymore.
And because I wanted to learn a bit more about Rust by completing a Rust app entirely on my own.

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

## Known issues
- Updating: Right now (but planned) you can't update existing reminders, just overwrite (via add)
- Multiple reminder emails: Once you're in the reminding time range you will be reminded every day. There's a solution planned for it.
- Some european tlds (like .de) don't give you an expiry date via whois. So this won't really work right now for some tlds. 


## Todo (Roadmap)
1. Mark a domain as reminded after first reminder has been successfully sent
2. Add command for updating existing reminders
3. Allow adding multiple domains per single add
4. Remove outdated (not registered domains) automatically
5. Add `delete` command to remove domains from reminder list
6. Find a solution for european (i.e. .de) tlds (another optional parameter?)