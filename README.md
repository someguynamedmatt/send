## How to use:

`send --to={email_address} --subject={subject} --body={text} --file={./my.bak}`

> Send will allow you to quickly send an email/attachment to a supplied address. This is helpful if you spend a lot of time in the command line, but are too lazy/busy to leave the environment.

- Create a `Settings.json` file in the root of the directory with your sending email and the password

    - Ex:

      ```
        {
          "email": "myfakeemail@mailinator.com",
          "password": "123fake"
        }
      ```

WIP:

  - Only works with gmail right now
  - Default values not set
