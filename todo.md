# Todo (small mods)
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
- clap console app for now as frontend
  - --remind [domain] [?timeRange]
  - --update [domain] [?timeRange]
  - --check [?domain]
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