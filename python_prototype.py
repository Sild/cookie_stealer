#! /usr/bin/env python3

from Crypto.Cipher import AES
from Crypto.Protocol.KDF import PBKDF2

# Function to get rid of padding
def clean(x): 
    return x[:-x[-1]].decode('utf8')

# replace with your encrypted_value from sqlite3
encrypted_value = b"v11��I������hX\u{14}\u{1f}G��Ja�I\t��N�!�-�aN�3Q�4��\u{b}�\u{3}��\u{3}��"

# Trim off the 'v10' that Chrome/ium prepends
encrypted_value = encrypted_value[3:]

# Default values used by both Chrome and Chromium in OSX and Linux
salt = b'saltysalt'
iv = b' ' * 16
length = 16

# On Mac, replace MY_PASS with your password from Keychain
# On Linux, replace MY_PASS with 'peanuts'
my_pass = 'peanuts'
my_pass = my_pass.encode('utf8')

# 1003 on Mac, 1 on Linux
iterations = 1

key = PBKDF2(my_pass, salt, length, iterations)
cipher = AES.new(key, AES.MODE_CBC, IV=iv)

decrypted = cipher.decrypt(encrypted_value)
print(clean(decrypted))
