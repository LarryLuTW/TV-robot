# TV-robot

This project is used to control computer's up, down, left, and right, and volume buttons on my phone. If you want to know the development process, you can read the article [不想離開沙發，只好自己寫一個電腦遙控器了](https://medium.com/starbugs/how-to-make-a-computer-controller-7f8ffcdbe993).

## [Demo on Youtube](https://www.youtube.com/watch?v=aIx-Li1m-3c)

[![Youtube Demo Video](https://user-images.githubusercontent.com/10403741/113395919-39aab680-93cd-11eb-8a72-36f374df1927.png)](https://www.youtube.com/watch?v=aIx-Li1m-3c)

## Install

```sh
cargo install --git https://github.com/LarryLuTW/TV-robot
```

## Run

#### 1. Connect your phone to the same wifi as your computer 

#### 2. Run `tv-robot` command and the QRCode will appear on the terminal.

<img width="697" alt="Screen Shot 2021-03-31 at 3 49 53 PM" src="https://user-images.githubusercontent.com/10403741/113109894-046d5f80-9239-11eb-9f09-61dc372218c0.png">

#### 3. Scan this QRCode with your phone to control the computer.

<img width="328" alt="Screen Shot 2021-03-31 at 3 58 18 PM" src="https://user-images.githubusercontent.com/10403741/113110769-08e64800-923a-11eb-81cb-7b0abb651cde.png">

## Upgrade Dependencies

To upgrade all dependencies to their latest versions, run:

```sh
cargo upgrade -i && cargo update
```
