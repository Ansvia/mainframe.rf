FORMAT: 1A

# $name$ rest API documentation

Dokumentasi rest API public untuk $name$.

## Group $param.service_name_pascal_case$

### Register $param.service_name_pascal_case$ [POST /$param.service_name_snake_case$/v1/$param.service_name_snake_case$/register]

Rest API endpoint untuk mendaftarkan akun baru.
Setelah register akun tidak langsung aktif, perlu melakukan
aktifasi menggunakan endpoint `/$param.service_name_snake_case$/activate`.

+ Request JSON (application/json)

        {
          "full_name": "Robin",
          "email": "robin@example.com",
          "phone_num": "123"
        }

+ Response 200 (application/json)

        {}
