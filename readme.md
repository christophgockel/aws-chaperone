# AWS Chaperone

Chaperone is a wrapper for the AWS CLI tool to manage the "heavy lifting" of temporary session tokens.

It is currently more of a proof of concept than a tool for general usage.
Please keep this in mind when trying to use it.


## Background

Let's use listing the content of an S3 bucket as an example.

The usual command for that is:

```
aws s3 ls s3://the-bucket-name
```

With a combination of environment variables used in the background (e.g. `AWS_PROFILE` or the combination of `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`).

Now things get a bit more complicated when you need get temporary session tokens first, in order to execute the command above.

This usually involves a call to STS like this beforehand:

```
aws sts get-session-token --serial-number ...
```

This call will then return _new_ credentials that can be used for a short(er) period of time:

```
{
    "Credentials": {
        "AccessKeyId": "AS.....6N",
        "SecretAccessKey": "GT3w.......izvuDOyzo....uRhO/Q",
        "SessionToken": "FwoGZ......zEOv/////wEarFP3Eh....................3pn",
        "Expiration": "2019-11-22T22:03:19Z"
    }
}
```

Now we'd need to put these credentials either into our current environment as variables or put them into a profile somewhere in `~/.aws/credentials`.

Since this is a fairly manual and involved process, _chaperone_ will manage it for you.


## Usage

Instead of using the AWS CLI directly, we can prefix it with the `chaperone` command:

```
chaperone aws s3 ls s3://the-bucket-name
```

Chaperone will check and see if a session token is available and use it to execeute the command itself.
If no credentials could be found it will request them before executing the command and put them into `~/.chaperone/credentials` for future usage.


## Similar Projects

- aws-sudo: https://github.com/bustle/aws-sudo


## Roadmap

- [ ] Support individual credentials per profile
- [ ] Request new credentials when the existing ones are expired
- [ ] Support a default profile


## Changelog

The changelog is maintained in the separate [changelog.md](changelog.md) file.


## Thanks

Thanks and shoutout to [Felipe Seré](https://github.com/felipesere) for coming up with the name _chaperone_ for this tool.

