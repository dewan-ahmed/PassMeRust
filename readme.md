# PassMeRust Password Strength Calculator

Simple and efficient command-line password strength assessment tool.

**PassMeRust** is a command-line tool written in Rust that calculates the strength of a password. It checks the provided password against a list of common passwords and a dictionary of words to determine its strength. The password strength is measured based on factors such as length, character variety, and presence in common password lists. It also supports space characters within passwords for more diverse passphrases.

## Features

- Calculates the strength of a password based on its length, character variety, and the types of characters used (alphabets, numbers, symbols, and spaces).
- Checks if the password is present in a list of common passwords.
- Verifies if the password is a dictionary word.
- Provides a score and a strength rating for the password.
- Supports both console input and command-line argument for entering the password.

## How Password Strength is Calculated

### Character Set Calculation (`calculate_charset` function):

The character set size plays a vital role in determining the password's entropy. The more diverse the set of characters used in a password, the higher its entropy. The function `calculate_charset` analyzes the password to determine the possible characters it's composed of:

- **Numbers**: Consists of 10 characters (0-9).
- **Lowercase Letters**: Consists of 26 characters (a-z).
- **Uppercase Letters**: Consists of 26 characters (A-Z).
- **Special Characters and Spaces**: For our calculations, we assume the presence of 33 special characters and the space character. This encompasses space, punctuation, and symbols from the ASCII table.

Based on the types of characters detected in the password, `calculate_charset` returns a cumulative count, which is then used in the entropy calculation.

### Strength Assessment (`get_strength` function):

After calculating the entropy using the character set size and password length, the `get_strength` function categorizes the password into one of five strength classes:

- **Very Weak**: Entropy is less than 28.
- **Weak**: Entropy is between 28 and 36.
- **Moderate**: Entropy is between 36 and 60.
- **Strong**: Entropy is between 60 and 128.
- **Very Strong**: Entropy is above 128.

The strength is then outputted to the user, providing a simple, human-readable assessment of the password's robustness.

## Requirements

- Rust (1.54.0 or later)

## Installation

1. Clone this repository:

   ```
   git clone https://github.com/dewan-ahmed/PassMeRust.git
   ```

2. Change to the project directory:

   ```
   cd PassMeRust
   ```

3. Build the project:

   ```
   cargo build --release
   ```

4. Run the program:

   ```
   cargo run --release
   ```

## Usage

To calculate the strength of a password, run the program and follow the prompts to enter the password. Alternatively, you can provide the password as a command-line argument:

```
cargo run --release -- "<password>"
```

Replace `<password>` with the actual password you want to check. If your password contains spaces or special characters, ensure it is wrapped in double quotes. The program will output a strength rating such as "Very Weak", "Weak", "Moderate", "Strong", or "Very Strong".

## Examples

1. Calculating password strength interactively:

   ```
   $ cargo run --release
   Enter a password: **********
   Password Strength: Strong
   ```

2. Calculating password strength with a command-line argument:

   ```
   $ cargo run --release -- MySecurePassword123
   Password Strength: Weak
   ```

3. Using a password with spaces:

   ```
   $ cargo run --release -- "My Secure Password 123"
   Password Strength: Very Strong
   ```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Disclaimer

This tool is meant to be used for educational and informational purposes only. It does not guarantee the absolute security of a password. Users are advised to follow best practices and use strong, unique passwords for their accounts.
