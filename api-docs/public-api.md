FORMAT: 1A

# $name$ rest API documentation

Dokumentasi rest API public untuk $name$.

## Group Account

### Register Account [POST /account/v1/account/register]

Rest API endpoint untuk mendaftarkan akun baru.
Setelah register akun tidak langsung aktif, perlu melakukan
aktifasi menggunakan endpoint `/account/activate`.

+ Request JSON (application/json)

        {
          "full_name": "Robin",
          "email": "robin@example.com",
          "phone_num": "123"
        }

+ Response 200 (application/json)

        {}
