## How to use:

`send --to={to address} --subject={subject} --body={Email text} --file={/home/me/my_conf.bak}`


### Send will allow you to quickly send a an email (with an attachment) to a supplied address. This is helpful if you tend to send yourself backup files when working in your command line, but you're too busy/lazy to get out of that environment.

- Create a `Settings.json` file in the root directory with your sending email address' name/password

    - Ex:

    ```
    {
      "email": "myfakeemail@mailinator.com",
      "password": "123fake"
    }
    ```
