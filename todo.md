# Todo (small mods)
- download whois server config if not existing
- whois domain into json file
- create reminder for domain (x days/weeks before)
- remind via email
- update domain expiration date (whois into json file)

# What
- checks domains for expiration date
- creates a reminder for each domain
- remind via email, slack, ...
- needs either a cronjob (once a day) or service running (web?) in background for reminder

# How
- we may want to use i.e. https://crates.io/crates/check-if-email-exists for checking if the email address is valid/exists
- clap console app for now as frontend
  - --add [domain] [email] [?timeRange]
  - --update [?domain] [?email] [?timeRange]
  - --remind [?domain]
- separate module for 
  - whois to json
  - create reminder for domain
  - send reminder to recipient
- also regularly download https://github.com/FurqanSoftware/node-whois/blob/master/servers.json for whois
- reminder message: 
> You asked me to remind you about the following domain(s):
> 
> Domain XXX.XX:
> The domain XXX.XX is going to expire in Xdays/weeks/months.
> First registration was: XXX
> Expiration date: XXX
> ....
> DOMAIN YYY.YY